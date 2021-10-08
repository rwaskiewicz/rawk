# { foo=3; bar=1; foo-=bar-=foo; print foo; }
{
    foo=3;
    bar=1;
    foo-=bar-=foo;
    print foo;
} # EXPECT: 5
