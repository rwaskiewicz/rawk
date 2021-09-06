{
    i = 0;
    j = 0;
    while (i < 1) {
        do {
            j = 2;
            break;
            j = 3;
        } while (j < 0);
        i = 4;
        break;
        i = 5;
    }
    print "i is",i,"j is",j;
} # EXPECT: i is 4 and j is 2
# { i = 0; j = 0; while (i < 1) { do { j = 2; break; j = 3; } while (j <= 0); i = 4; break; i = 5; } print "i is",i,"j is",j; }