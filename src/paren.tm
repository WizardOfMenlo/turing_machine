states 6
start
open
close
verify
accept +
reject -
alphabet 5 ^ $ [ ( )
start ( close ^ R
start _ accept _ R

close ( close ( R
close ^ close ^ R
close $ close $ R
close [ close [ R
close ) open $ L
close _ verify _ L

open ) open ) L
open $ open $ L
open ^ close [ R
open ( close $ R

verify [ accept [ L
verify $ verify $ L