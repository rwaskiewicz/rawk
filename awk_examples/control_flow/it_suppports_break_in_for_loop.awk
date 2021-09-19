# {result = "hell";for (i=0; i<10; i=i+1) {result = result i; break;} print result;}
{
    result = "hell";
    for (i=0; i<10; i=i+1) {
        result = result "," i;
        break;
    }
} # EXPECT: hell0
