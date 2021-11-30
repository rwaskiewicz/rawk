# { i = 0; j = 0; while (i < 2) { i=i+1; while (j < 3) { j = j+2; break; print "This is the j loop - this should not print"; } break; print "This is the i loop - this should not print"; } print "i is", i, "and j is", j; }
{
    i = 0;
    j = 0;
    while (i < 2) {
        i=i+1;
        while (j < 3) {
            j = j+2;
            break;
            print "This is the j loop - this should not print";
        }
        break;
        print "This is the i loop - this should not print";
    }
    print "i is", i, "and j is", j;
} # EXPECT: i is 1 and j is 2
