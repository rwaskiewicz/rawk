# { foo=3; foo^=foo; print foo }
{
    foo=3;
    foo^=foo;
    print foo;
} # EXPECT: 27
