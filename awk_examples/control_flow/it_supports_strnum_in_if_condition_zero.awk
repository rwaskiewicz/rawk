# DATA: 0
# {if ($1) print "this should not"; else print "should print";}
{
    if ($1)
        print "this should not";
    else
        print "should print";
} # EXPECT: "should print"
