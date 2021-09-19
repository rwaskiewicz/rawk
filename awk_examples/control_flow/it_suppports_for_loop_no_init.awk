# {result = "";for (; i<10; i=i+1) {result = result i;} print result;}
{
    result = "";
    for (;i<10; i=i+1) {
        result = result "," i;
    }
} # EXPECT: 123456789
