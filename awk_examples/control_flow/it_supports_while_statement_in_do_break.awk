{
    i = 0;
    j = 0;
    do {
        while (i < 1) {
            j = 2;
            break;
            j = 3;
        }
        i = 4;
        break;
        i = 5;
    } while (j <= 0);
    print "i is",i,"j is",j;
} # EXPECT: i is 4 and j is 2
# { i = 0; j = 0; do { while (i < 1) { j = 2; break; j = 3; } i = 4; break; i = 5; } while (j <= 0); print "i is",i,"j is",j; }
