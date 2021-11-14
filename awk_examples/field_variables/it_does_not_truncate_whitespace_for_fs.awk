# DATA: | Alice  ,40 ,25 |
# OPTIONS: F=,
# { print $1$2$3; }
{
    print $1$2$3;
} # EXPECT: | Alice  40 25 |
