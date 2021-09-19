# {result = "";for (i=0;; i=i+1) {result = result i; if (i>=10) { break; } } print result;}
{
    result = "";
    for (i=0;;i=i+1) {
        result = result "," i;
        if (i>=10) {
            break;
        }
    }
} # EXPECT: 012345678910
