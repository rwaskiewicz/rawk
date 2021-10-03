# {val = 1; print "The value is", (val > 2 ? "greater than" : "less than or equal to"), "2";}
{
    val = 1;
    print "The value is", (val > 2 ? "greater than" : "less than or equal to");, "2";
} # EXPECT: The value is less than or equal to 2
