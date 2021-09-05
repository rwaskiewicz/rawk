{
    while (i < 2) {
        i=i+1;
        while (j < 3) {
            j = j+1;
            continue;
            print "This is the j loop - this should not print";
        }
        continue;
        print "This is the i loop - this should not print";
    }
    print "i is", i, "and j is", j;
}
# EXPECT: i is 2 and j is 3