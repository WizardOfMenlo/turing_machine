states 20
start
write0
write1
write0S
write1S
goStart
goStartC
startSplit
split
expect0
expect1
expect0C
expect1C
compare
compareNext
goToNextChar
validate
check
accept +
reject -
alphabet 5 ^ $ # 0 1
start 0 write0 ^ R
start 1 write1 ^ R
start _ accept _ R

write0 0 write0 0 R
write0 1 write1 0 R
write0 _ goStart 0 L

write1 0 write0 1 R
write1 1 write1 1 R
write1 _ goStart 1 L

goStart 0 goStart 0 L
goStart 1 goStart 1 L
goStart ^ startSplit ^ R

startSplit 0 split 0 R
startSplit 1 split 1 R
startSplit 0 startSplit 0 R
startSplit 1 startSplit 1 R

split 0 write0S # R
split 1 write1S # R

write0S 0 write0S 0 R
write0S 1 write1S 0 R
write0S _ goStartC 0 L

write1S 0 write0S 1 R
write1S 1 write1S 1 R
write1S _ goStartC 1 L

goStartC 0 goStartC 0 L
goStartC 1 goStartC 1 L
goStartC # goStartC # L
goStartC ^ compare ^ R

compare 0 expect0 $ R
compare 1 expect1 $ R

expect0 0 expect0 0 R
expect0 1 expect0 1 R
expect0 # expect0C # R

expect1 0 expect1 0 R
expect1 1 expect1 1 R
expect1 # expect1C # R

expect0C 0 compareNext $ L
expect0C $ expect0C $ R

expect1C 1 compareNext $ L
expect1C $ expect1C $ R

compareNext 0 compareNext 0 L
compareNext 1 compareNext 1 L
compareNext # compareNext # L
compareNext $ compareNext $ L
compareNext ^ goToNextChar ^ R

goToNextChar $ goToNextChar $ R
goToNextChar 0 compare 0 S
goToNextChar 1 compare 1 S
goToNextChar # validate # L

validate $ validate $ L
validate ^ check ^ R

check # check # R
check $ check $ R
check _ accept _ R