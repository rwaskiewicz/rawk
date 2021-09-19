# { for (i=0; i<=2; i=i+2) { continue; break; } print i; }
{
    for (i=0; i<=2; i=i+2) {
        continue;
        break;
    }
    print i;
} # EXPECT: 4
