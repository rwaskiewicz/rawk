# {result = "";for (i=0; i<10;) {result = result i; i = i + 1;} print result;}
{
    result = "";
    for (i=0; i<10; i=i+1) {
        result = result "," i;
        i = i + 1;
    }
} # EXPECT: 0123456789
