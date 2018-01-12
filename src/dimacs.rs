use std::fs::File;
use std::io::{BufRead, BufReader};
use solver::Problem;

fn read_file(filename: &str) -> BufReader<File> {
    return BufReader::new(match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("File {} can't be opened: {}", filename, err),
    });
}

/// Reads weight-extended DIMACS format.
pub fn read(filename: &str) -> Problem {
    let file = read_file(&filename);
    let mut properties: Option<(u16, u16)> = None;
    let mut formula: Vec<Vec<i16>> = vec![];
    let mut weights: Vec<u16> = vec![0];

    'l: for line in file.lines() {
        if line.is_err() { continue; }
        let uline = line.unwrap();
        let fields: Vec<&str> = uline.split_whitespace().collect();
        match &*fields[0] {
            "p" => { // Parse properties
                let variables = fields[2].parse::<u16>();
                let clausules = fields[3].parse::<u16>();
                if variables.is_ok() && clausules.is_ok() {
                    properties = Some((variables.unwrap(), clausules.unwrap())); 
                }
            },
            "w" => { // Parse weights
                if properties.is_some() && fields.len() == properties.unwrap().0 as usize + 1 {
                    let mut ofields = fields.clone();
                    weights = ofields.drain(1..).map(|x| { 
                        match x.parse::<u16>() {
                            Ok(x) => x,
                            Err(_) => { panic!("Clause variable must be in range 0-65535."); },
                        }
                    }).collect();
                }
            },
            "c" => (), // Ignore comments
            "%" => { break 'l; } // Stop reading
            _ => { // Parse numbers
                if fields[0].parse::<i16>().is_ok() {
                    if properties.is_some() {
                        let mut ofields = fields.clone();
                        ofields.pop();
                        formula.push(ofields.iter().map(|x| {
                            match x.parse::<i16>() {
                                Ok(x) => { x },
                                Err(_) => { panic!("Clause variable must be in range 0-65535."); },
                            }
                        }).collect());
                    } else { panic!("The DIMACS file is corrupted."); }
                } else { panic!("Unrecognized data type in DIMACS file."); }
            },
        }
    }
    
    if properties.is_none() { panic!("The DIMACS failed to read."); }
    
    Problem {
        variables: properties.unwrap().0,
        clausules: properties.unwrap().1,
        formula: formula,
        maximum: weights.iter().sum(),
        weights: weights,
    }
}