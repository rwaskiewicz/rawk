{
    i=0;
    while (i<=2) {
        i = i + 2;
        continue;
        break;
    }
    print i;
} # EXPECT: 4
# { i=0; while (i<=2) { i = i + 2; continue; break; } print i; }
