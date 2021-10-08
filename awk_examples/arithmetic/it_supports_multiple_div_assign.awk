# { foo=81; bar=9; baz=3; foo/=bar/=baz; print foo; }
{
    foo=81;
    bar=9;
    baz=3;
    foo/=bar/=baz;
    print foo;
} # EXPECT: 27
