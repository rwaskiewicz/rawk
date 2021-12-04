# {if ("hello") print "should print"; else print "this should not";}
{
    if ("hello")
        print "should print";
    else
        print "this should not";
} # EXPECT: "should print"
