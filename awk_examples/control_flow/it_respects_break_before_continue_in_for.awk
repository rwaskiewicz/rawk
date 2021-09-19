# { for (i=1; i<2; i=i+1) { break; i = 99; continue; } print i; }
{
    for (i=1; i<2; i=i+1) {
        break;
        i = 99;
        continue;
    }
    print i;
} # EXPECT: 1

