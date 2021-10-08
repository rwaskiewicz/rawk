# { foo=7; bar=5; baz=3; foo%=bar%=baz; print foo; }
{
    foo=7;
    bar=5;
    baz=3;
    foo%=bar%=baz;
    print foo;
} # EXPECT: 1
