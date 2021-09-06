{
    i=1;
    do {
        break;
        i = 2;
        continue;
    } while (i>=2);
    print i;
} # EXPECT: 1
# { i=1; do { break; i = 2; continue; } while (i>=2); print i; }'