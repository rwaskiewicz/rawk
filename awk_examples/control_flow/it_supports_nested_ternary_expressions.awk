# {val = 1; result = val ? (val > 1 ? 2 : 3) : 50; print result;}
{
    val = 1;
    result = val ? (val > 1 ? 2 : 3) : 50;
    print result;
} # EXPECT: 3
