# {result = "hell";for (i=0; i<10; i=i+1) {result = result i; continue; result="???";} print result;}
{
    result = "hell";
    for (i=0; i<10; i=i+1) {
        result = result "," i;
        continue;
        result="???";
    }
} # EXPECT: hell0123456789
