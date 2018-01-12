set term qt
set output 'fitness-over-gen.png'
plot 'evo.txt' with linespoints ls 1
pause 2