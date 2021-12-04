# DATA: ''
# {if ($1) print "should print"; else print "this should not";}
{
    if ($1)
        print "should print";
    else
        print "this should not";
} # EXPECT: "this should not"
