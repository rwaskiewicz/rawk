# {result = "";for (i=0; i<10;) {result = result i; i = i + 1;} print result;}
{
    result = "";
    for (i=0; i<10;) {
        result = result i;
        i = i + 1;
    }
    print result;
} # EXPECT: 0123456789
