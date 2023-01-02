# 1 > 0 { foo = 23; } 1 {} { foo += 3; } 2 < 1 { foo += 5; } { print foo; }
1 > 0 { foo = 23; }
1
{ foo += 3; }
2 < 1 { foo += 5; }
{ print foo; }
# Expect 26