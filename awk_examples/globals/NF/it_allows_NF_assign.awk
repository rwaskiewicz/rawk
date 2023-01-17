# DATA: hello world
# { print "NF",NF; NF=23; print "NF",NF; }
{
    print "NF",NF;
    NF=23;
    print "NF",NF;
} # EXPECT: NF 2\nNF 23
