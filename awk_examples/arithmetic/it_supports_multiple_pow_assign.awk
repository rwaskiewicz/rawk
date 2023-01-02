# { foo=2; foo^=foo^=foo; print foo; }
{
    foo=2;
    # foo becomes 4 from the rightmost `foo^=foo`, so `foo^=(foo^=foo) === 4^4`
    foo^=foo^=foo;
    print foo;
} # EXPECT: 256
