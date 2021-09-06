{
    i=0;
    j=0;
    do {
        i=i+1;
        do {
            j = j+1;
            continue;
            print "This is the j loop - this should not print";
        } while (j < 3);
        continue;
        print "This is the i loop - this should not print";
    } while (i < 2);
    print "i is", i, "and j is", j;
} # EXPECT: i is 2 and j is 4
