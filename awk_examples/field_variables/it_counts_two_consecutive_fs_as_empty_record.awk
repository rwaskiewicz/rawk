# DATA: Hello,,World!
# OPTIONS: F=,
# { print $1$2$3; }
{
    print $1$2$3;
} # EXPECT: HelloWorld!
