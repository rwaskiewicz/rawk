# DATA: hello world
# DATA: I come in peace!
# NF > 2 { print $0; }
NF > 2 { print $0; }
# EXPECT: [EMPTY]\nI come in peace!
