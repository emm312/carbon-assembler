// LINE DRAWER
>0
// def line(a: r1 b: r2)
// X.x = X & 0b11110000
// X.y = X & 0b00001111
// dx = b.x - a.x
// dx = r3
// r7 = 0b11110000
// r6 = 0b00001111

// get b.x
RLD r2 .line

