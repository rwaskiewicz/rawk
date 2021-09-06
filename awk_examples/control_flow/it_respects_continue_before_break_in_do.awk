{
    i=0;
    do {
        i = i + 2;
        continue;
        break;
    } while (i<=2);
    print i;
} # EXPECT: 4
# { i=0; do { i = i + 2; continue; break; } while (i<=2); print i; }