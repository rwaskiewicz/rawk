{
    i = 0;
    j = 0;
    do {
        while (j < 1) {
            j = 2;
            continue;
            j = 3;
        }
        i = 4;
        continue;
        i = 5;
    } while (i <= 3)
    print "i is",i,"j is",j;
} # EXPECT: i is 4 and j is 2
# { i = 0; j = 0; do { while (j < 1) { j = 2; continue; j = 3; } i = 4; continue; i = 5; } while (i <= 3); print "i is",i,"j is",j; }