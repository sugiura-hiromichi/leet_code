plot "euler1.dat" every ::step::step using 1:2 lt 1 pt 7 ps 5 title "EulerMethod", "euler1.dat" using 1:2 with line title "Locus" lt 1, \
step=step+1
if(step<1000) reread
step=0
