states 48
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
0?0S
0?1S
000S
100S
001S
101S
B
B0
B1
BC
C
shift0
shift1
shift#
shiftSecond0
shiftSecond1
shiftSecond#
toStart
start0
accept +
reject -
alphabet 5 0 1 # $ ^
start 0 0?0 ^ R
start 1 1?0 ^ R
start # shift# ^ R

shift0 0 shift0 0 R
shift0 1 shift1 0 R
shift0 # shiftSecond# 0 R

shift1 0 shift0 1 R
shift1 1 shift1 1 R
shift1 # shiftSecond# 1 R

shift# 0 shift0 # R
shift# 1 shift1 # R
shift# # shiftSecond# # R

shiftSecond0 0 shiftSecond0 0 R
shiftSecond0 1 shiftSecond1 0 R
shiftSecond0 # shiftSecond# 0 R
shiftSecond0 _ toStart 0 L

shiftSecond1 0 shiftSecond0 1 R
shiftSecond1 1 shiftSecond1 1 R
shiftSecond1 # shiftSecond# 1 R
shiftSecond1 _ toStart 1 L

shiftSecond# 0 shiftSecond0 # R
shiftSecond# 1 shiftSecond1 # R
shiftSecond# # shiftSecond# # R
shiftSecond# _ toStart # L

toStart 0 toStart 0 L
toStart 1 toStart 1 L
toStart # toStart # L
toStart ^ start0 ^ L

start0 ^ 0?0 ^ R

??0 0 0?0 $ R
??0 1 1?0 $ R
??0 $ ??0 $ R
??0 # 0?0S # L

0?0S $ 0?0 $ R
0?0S ^ 0?0 ^ R

??1 0 0?1 $ R
??1 1 1?1 $ R
??1 $ ??1 $ R
??1 # 0?1S # L

0?1S $ 0?1 $ R
0?1S ^ 0?1 ^ R

0?0 0 0?0 0 R
0?0 1 0?0 1 R
0?0 # 0?0R # R

1?0 0 1?0 0 R
1?0 1 1?0 1 R
1?0 # 1?0R # R

0?0R 0 000 $ R
0?0R 1 010 $ R
0?0R $ 0?0R $ R
0?0R # 000S # L

000S $ 000 $ R
000S # 000 # R

1?0R 0 100 $ R
1?0R 1 110 $ R
1?0R $ 1?0R $ R
1?0R # 100S # L

100S $ 100 $ R
100S # 100 # R

0?1 0 0?1 0 R
0?1 1 0?1 1 R
0?1 # 0?1R # R

1?1 0 1?1 0 R
1?1 1 1?1 1 R
1?1 # 1?1R # R

0?1R 0 001 $ R
0?1R 1 011 $ R
0?1R $ 0?1R $ R
0?1R # 001S # L

001S $ 001 $ R
001S # 001 # R

1?1R 0 101 $ R
1?1R 1 111 $ R
1?1R $ 1?1R $ R
1?1R # 101S # L

101S $ 101 $ R
101S # 101 # R

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

000C _ BC _ L
000C 0 B0 $ R
001C 1 B0 $ R
010C 1 B0 $ R
011C 0 B1 $ R
100C 1 B0 $ R
101C 0 B1 $ R
110C 0 B1 $ R
111C 1 B1 $ R

000C $ 000C $ R
001C $ 001C $ R
010C $ 010C $ R
011C $ 011C $ R
100C $ 100C $ R
101C $ 101C $ R
110C $ 110C $ R
111C $ 111C $ R

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

BC 0 BC 0 L
BC 1 BC 1 L
BC # BC # L
BC $ BC $ L
BC _ BC _ L
BC ^ C ^ L

C # C # R
C ^ C ^ R
C $ C $ R
C 0 C 0 R
C _ accept _ R
