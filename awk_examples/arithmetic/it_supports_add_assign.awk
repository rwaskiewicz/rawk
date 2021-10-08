# { foo=3; bar+=foo; print bar; }
{
    foo=3;
    bar+=foo;
    print bar;
} # EXPECT: 3
