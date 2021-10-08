# { foo=3; bar=7; bar*=foo; print bar; }
{
    foo=3;
    bar=7;
    bar*=foo;
    print bar;
} # EXPECT: 21
