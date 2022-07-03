plot "rk1.dat" every ::step::step u 2:3 lt 1 pt 7 ps 5 ti "RUnge Ktta", "rk1.dat" u 2:3 w l lt 1 ti "", \
"~/rust/b8/euler/src/euler1.dat" every ::step::step u 2:3 lt 2 pt 7 ps 5 ti "Euler", "~/rust/b8/euler/src/euler1.dat" u 2:3 w l lt 2 ti ""
step=step+1 
if(step<1000) reread
step=0
