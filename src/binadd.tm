states 32
start
??0
??1
0?0
0?1
1?0
1?1
0?0R
0?1R
1?0R
1?1R
000
001
010
011
100
101
110
111
000C
001C
010C
011C
100C
101C
110C
111C
B
B0
B1
accept +
reject -
alphabet 5 0 1 # $ ^
start 0 0?0 ^ R
start 1 1?0 ^ R

??0 0 0?0 $ R
??0 1 1?0 $ R
??0 $ ??0 $ R

0?0 0 0?0 0 R
0?0 1 0?0 1 R
0?0 # 0?0R # R

1?0 0 1?0 0 R
1?0 1 1?0 1 R
1?0 # 1?0R # R

0?0R 0 000 $ R
0?0R 1 010 $ R
0?0R $ 0?0R $ R

1?0R 0 100 $ R
1?0R 1 110 $ R
1?0R $ 1?0R $ R

000 0 000 0 R
000 1 000 1 R
000 # 000C # R


001 0 001 0 R
001 1 001 1 R
001 # 001C # R

010 0 010 0 R
010 1 010 1 R
010 # 010C # R

011 0 011 0 R
011 1 011 1 R
011 # 011C # R

100 0 100 0 R
100 1 100 1 R
100 # 100C # R

101 0 101 0 R
101 1 101 1 R
101 # 101C # R

110 0 110 0 R
110 1 110 1 R
110 # 110C # R

111 0 111 0 R
111 1 111 1 R
111 # 111C # R

000C 0 B0 $ R
001C 1 B0 $ R
010C 1 B0 $ R
011C 0 B1 $ R
100C 1 B0 $ R
101C 0 B1 $ R
110C 0 B1 $ R
111C 1 B1 $ R

B0 0 B0 0 L
B0 1 B0 1 L
B0 # B0 # L
B0 $ B0 $ L
B0 _ B0 _ L
B0 ^ ??0 ^ R

B1 0 B1 0 L
B1 1 B1 1 L
B1 # B1 # L
B1 $ B1 $ L
B1 _ B1 _ L
B1 ^ ??1 ^ R