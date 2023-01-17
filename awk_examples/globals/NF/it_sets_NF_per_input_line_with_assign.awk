# DATA: hello world\ni\ncome in\npeace
# { if (NF == 2) { NF = 23; } print NF; }
{
    if (NF == 2) {
        NF = 23;
    }
    print NF;
} # EXPECT: 23\n1\n23\n1
