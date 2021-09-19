# for (i=2; i<10; i=i+1) { for (j=3; j<=12; j=j+1) { continue; print "This is the j loop - this should not print"; } continue; print "This is the i loop - this should not print"; } print "i is", i, "and j is", j;
{
    for (i=2; i<10; i=i+1) {
        for (j=3; j<=12; j=j+1) {
            continue;
            print "This is the j loop - this should not print";
        }
        continue;
        print "This is the i loop - this should not print";
    }
    print "i is", i, "and j is", j;
} # EXPECT: i is 10 and j is 13
