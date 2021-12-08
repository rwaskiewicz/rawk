# {print foo=3,2;print foo;}
{
    print foo=3,2;
    print foo;
} # EXPECT: "3 2\n3"
