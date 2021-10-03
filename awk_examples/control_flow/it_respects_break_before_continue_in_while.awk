{
    i=1;
    while (i>=2) {
        break;
        i = 2;
        continue;
    }
    print i;
} # EXPECT: 1
# { i=1; while (i>=2) { break; i = 2; continue; } print i; }'
