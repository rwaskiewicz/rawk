# { foo=3; bar=6; bar/=foo; print bar; }
{
    foo=3;
    bar=6;
    bar/=foo;
    print bar;
} # EXPECT: 2
