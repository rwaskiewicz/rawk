# DATA: 0 3 5
# { print $($(1+1)); }
{
    print $($(1+1));  # $($2)
} # EXPECT: 5
