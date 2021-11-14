# DATA: "a"
# OPTIONS: F=a
# { print "|"$1"|"$2"|"; }
{
    print "|"$1"|"$2"|";
} # EXPECT: "|||"
