# DATA: 32
# { print foo=$1; } { print foo*=2; } { print foo+2; }
{
    print foo=$1;
}
{
    print foo*=2;
}
{
    print foo+2;
} # EXPECT: 32\n64\n66
