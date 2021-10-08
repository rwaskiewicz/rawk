# { foo=2; foo^=foo^=foo; print foo; }
{
    foo=2;
    foo^=foo^=foo;
    print foo;
} # EXPECT: 256
