# DATA: "abac"
# OPTIONS: F="a"
# { print $1$2$3; }
{
    print $1$2$3;
} # EXPECT: " bc"
