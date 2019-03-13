states 70
begin
begin_1
new
mark_digits
mark_digits_x
mark_digits_z
mark_digits_z1
mark_digits_z2
find_digits
find_x
find_x1
first_r
first_r1
first_r2
last_r
last_r1
last_r2
last_r3
found_1st_digit1
find_2nd_digit
find_1st_digit1
find_digits1
found_1st_digit
add_zero
find_digits2
find_1st_digit
found_2nd_digit
found_1st_digit0
find_2nd_digit0
add_zero1
add_one
add_one1
add_one2
add_finished
carry
carry0
mock
new_digit_zero
reset_new_x1
erase_old_x1
add_finished0
reset_new_x0
erase_old_x2
erase_old_y0
erase_old_y
print_new_y
erase_old_y1
flag_result_digits
new_digit_one
add_finished1
erase_old_x
reset_new_x
print_new_x
print_new_x0
print_one_digit0
cleanup1
unflag_result_digits
print_zero_digit
print_one_digit
flag_result_digits1
flag_result_digits0
print_one_digit2
cleanup
print_zero_digit0
unflag_result_digits0
print_zero_digit1
print_one_digit1
print_zero_digit2
accept +
reject -
alphabet 12 0 1 @ x y z r s u t v w


begin _ begin_1 @ R
begin_1 _ new 1 S

new @ mark_digits @ R
new 0 new 0 L
new 1 new 1 L
new x new x L
new y new y L
new z new z L
new r new r L
new s new s L
new u new u L
new t new t L
new v new v L
new w new w L
new _ new _ L

mark_digits 0 mark_digits_x 0 R
mark_digits 1 mark_digits_x 1 R
mark_digits _ mark_digits_z _ R

mark_digits_x 0 mark_digits x R
mark_digits_x 1 mark_digits x R
mark_digits_x @ mark_digits x R
mark_digits_x x mark_digits x R
mark_digits_x y mark_digits x R
mark_digits_x z mark_digits x R
mark_digits_x r mark_digits x R
mark_digits_x s mark_digits x R
mark_digits_x u mark_digits x R
mark_digits_x t mark_digits x R
mark_digits_x v mark_digits x R
mark_digits_x w mark_digits x R
mark_digits_x _ mark_digits x R

mark_digits_z 0 mark_digits_z1 z R
mark_digits_z 1 mark_digits_z1 z R
mark_digits_z @ mark_digits_z1 z R
mark_digits_z x mark_digits_z1 z R
mark_digits_z y mark_digits_z1 z R
mark_digits_z z mark_digits_z1 z R
mark_digits_z r mark_digits_z1 z R
mark_digits_z s mark_digits_z1 z R
mark_digits_z u mark_digits_z1 z R
mark_digits_z t mark_digits_z1 z R
mark_digits_z v mark_digits_z1 z R
mark_digits_z w mark_digits_z1 z R
mark_digits_z _ mark_digits_z1 z R

mark_digits_z1 0 mark_digits_z2 0 R
mark_digits_z1 1 mark_digits_z2 1 R
mark_digits_z1 @ mark_digits_z2 @ R
mark_digits_z1 x mark_digits_z2 x R
mark_digits_z1 y mark_digits_z2 y R
mark_digits_z1 z mark_digits_z2 z R
mark_digits_z1 r mark_digits_z2 r R
mark_digits_z1 s mark_digits_z2 s R
mark_digits_z1 u mark_digits_z2 u R
mark_digits_z1 t mark_digits_z2 t R
mark_digits_z1 v mark_digits_z2 v R
mark_digits_z1 w mark_digits_z2 w R
mark_digits_z1 _ mark_digits_z2 _ R

mark_digits_z2 0 find_x r S
mark_digits_z2 1 find_x r S
mark_digits_z2 @ find_x r S
mark_digits_z2 x find_x r S
mark_digits_z2 y find_x r S
mark_digits_z2 z find_x r S
mark_digits_z2 r find_x r S
mark_digits_z2 s find_x r S
mark_digits_z2 u find_x r S
mark_digits_z2 t find_x r S
mark_digits_z2 v find_x r S
mark_digits_z2 w find_x r S
mark_digits_z2 _ find_x r S

find_x x first_r _ S
find_x @ find_digits @ S
find_x 0 find_x1 0 L
find_x 1 find_x1 1 L
find_x y find_x1 y L
find_x z find_x1 z L
find_x r find_x1 r L
find_x s find_x1 s L
find_x u find_x1 u L
find_x t find_x1 t L
find_x v find_x1 v L
find_x w find_x1 w L
find_x _ find_x1 _ L

find_x1 0 find_x 0 L
find_x1 1 find_x 1 L
find_x1 @ find_x @ L
find_x1 x find_x x L
find_x1 y find_x y L
find_x1 z find_x z L
find_x1 r find_x r L
find_x1 s find_x s L
find_x1 u find_x u L
find_x1 t find_x t L
find_x1 v find_x v L
find_x1 w find_x w L
find_x1 _ find_x _ L

first_r r first_r1 r R
first_r 0 first_r2 0 R
first_r 1 first_r2 1 R
first_r @ first_r2 @ R
first_r x first_r2 x R
first_r y first_r2 y R
first_r z first_r2 z R
first_r s first_r2 s R
first_r u first_r2 u R
first_r t first_r2 t R
first_r v first_r2 v R
first_r w first_r2 w R
first_r _ first_r2 _ R


first_r1 0 last_r 0 R
first_r1 1 last_r 1 R
first_r1 @ last_r @ R
first_r1 x last_r x R
first_r1 y last_r y R
first_r1 z last_r z R
first_r1 r last_r r R
first_r1 s last_r s R
first_r1 u last_r u R
first_r1 t last_r t R
first_r1 v last_r v R
first_r1 w last_r w R
first_r1 _ last_r _ R

first_r2 0 first_r 0 R
first_r2 1 first_r 1 R
first_r2 @ first_r @ R
first_r2 x first_r x R
first_r2 y first_r y R
first_r2 z first_r z R
first_r2 r first_r r R
first_r2 s first_r s R
first_r2 u first_r u R
first_r2 t first_r t R
first_r2 v first_r v R
first_r2 w first_r w R
first_r2 _ first_r _ R

last_r r last_r1 r R
last_r _ last_r2 r R

last_r1 0 last_r 0 R
last_r1 1 last_r 1 R
last_r1 @ last_r @ R
last_r1 x last_r x R
last_r1 y last_r y R
last_r1 z last_r z R
last_r1 r last_r r R
last_r1 s last_r s R
last_r1 u last_r u R
last_r1 t last_r t R
last_r1 v last_r v R
last_r1 w last_r w R
last_r1 _ last_r _ R

last_r2 0 last_r3 0 R
last_r2 1 last_r3 1 R
last_r2 @ last_r3 @ R
last_r2 x last_r3 x R
last_r2 y last_r3 y R
last_r2 z last_r3 z R
last_r2 r last_r3 r R
last_r2 s last_r3 s R
last_r2 u last_r3 u R
last_r2 t last_r3 t R
last_r2 v last_r3 v R
last_r2 w last_r3 w R
last_r2 _ last_r3 _ R

last_r3 0 find_x r S
last_r3 1 find_x r S
last_r3 @ find_x r S
last_r3 x find_x r S
last_r3 y find_x r S
last_r3 z find_x r S
last_r3 r find_x r S
last_r3 s find_x r S
last_r3 u find_x r S
last_r3 t find_x r S
last_r3 v find_x r S
last_r3 w find_x r S
last_r3 _ find_x r S

find_digits @ find_digits1 @ R
find_digits 0 find_digits2 0 L
find_digits 1 find_digits2 1 L
find_digits x find_digits2 x L
find_digits y find_digits2 y L
find_digits z find_digits2 z L
find_digits r find_digits2 r L
find_digits s find_digits2 s L
find_digits u find_digits2 u L
find_digits t find_digits2 t L
find_digits v find_digits2 v L
find_digits w find_digits2 w L
find_digits _ find_digits2 _ L

find_digits1 0 find_1st_digit 0 R
find_digits1 1 find_1st_digit 1 R
find_digits1 @ find_1st_digit @ R
find_digits1 x find_1st_digit x R
find_digits1 y find_1st_digit y R
find_digits1 z find_1st_digit z R
find_digits1 r find_1st_digit r R
find_digits1 s find_1st_digit s R
find_digits1 u find_1st_digit u R
find_digits1 t find_1st_digit t R
find_digits1 v find_1st_digit v R
find_digits1 w find_1st_digit w R
find_digits1 _ find_1st_digit _ R

find_digits2 0 find_digits 0 L
find_digits2 1 find_digits 1 L
find_digits2 @ find_digits @ L
find_digits2 x find_digits x L
find_digits2 y find_digits y L
find_digits2 z find_digits z L
find_digits2 r find_digits r L
find_digits2 s find_digits s L
find_digits2 u find_digits u L
find_digits2 t find_digits t L
find_digits2 v find_digits v L
find_digits2 w find_digits w L
find_digits2 _ find_digits _ L

find_1st_digit x found_1st_digit x L
find_1st_digit y found_1st_digit y L
find_1st_digit z found_2nd_digit z L
find_1st_digit _ find_1st_digit1 _ R

find_1st_digit1 0 find_1st_digit 0 R
find_1st_digit1 1 find_1st_digit 1 R
find_1st_digit1 @ find_1st_digit @ R
find_1st_digit1 x find_1st_digit x R
find_1st_digit1 y find_1st_digit y R
find_1st_digit1 z find_1st_digit z R
find_1st_digit1 r find_1st_digit r R
find_1st_digit1 s find_1st_digit s R
find_1st_digit1 u find_1st_digit u R
find_1st_digit1 t find_1st_digit t R
find_1st_digit1 v find_1st_digit v R
find_1st_digit1 w find_1st_digit w R
find_1st_digit1 _ find_1st_digit _ R

found_1st_digit 0 add_zero 0 R
found_1st_digit 1 found_1st_digit0 1 R

found_1st_digit0 0 found_1st_digit1 0 R
found_1st_digit0 1 found_1st_digit1 1 R
found_1st_digit0 @ found_1st_digit1 @ R
found_1st_digit0 x found_1st_digit1 x R
found_1st_digit0 y found_1st_digit1 y R
found_1st_digit0 z found_1st_digit1 z R
found_1st_digit0 r found_1st_digit1 r R
found_1st_digit0 s found_1st_digit1 s R
found_1st_digit0 u found_1st_digit1 u R
found_1st_digit0 t found_1st_digit1 t R
found_1st_digit0 v found_1st_digit1 v R
found_1st_digit0 w found_1st_digit1 w R
found_1st_digit0 _ found_1st_digit1 _ R

found_1st_digit1 0 find_2nd_digit 0 R
found_1st_digit1 1 find_2nd_digit 1 R
found_1st_digit1 @ find_2nd_digit @ R
found_1st_digit1 x find_2nd_digit x R
found_1st_digit1 y find_2nd_digit y R
found_1st_digit1 z find_2nd_digit z R
found_1st_digit1 r find_2nd_digit r R
found_1st_digit1 s find_2nd_digit s R
found_1st_digit1 u find_2nd_digit u R
found_1st_digit1 t find_2nd_digit t R
found_1st_digit1 v find_2nd_digit v R
found_1st_digit1 w find_2nd_digit w R
found_1st_digit1 _ find_2nd_digit _ R

find_2nd_digit x found_2nd_digit x L
find_2nd_digit y found_2nd_digit y L
find_2nd_digit _ find_2nd_digit0 _ R

find_2nd_digit0 0 find_2nd_digit 0 R
find_2nd_digit0 1 find_2nd_digit 1 R
find_2nd_digit0 @ find_2nd_digit @ R
find_2nd_digit0 x find_2nd_digit x R
find_2nd_digit0 y find_2nd_digit y R
find_2nd_digit0 z find_2nd_digit z R
find_2nd_digit0 r find_2nd_digit r R
find_2nd_digit0 s find_2nd_digit s R
find_2nd_digit0 u find_2nd_digit u R
find_2nd_digit0 t find_2nd_digit t R
find_2nd_digit0 v find_2nd_digit v R
find_2nd_digit0 w find_2nd_digit w R
find_2nd_digit0 _ find_2nd_digit _ R

found_2nd_digit 0 add_zero 0 R
found_2nd_digit 1 add_one 1 R
found_2nd_digit _ add_one _ R

add_zero r add_finished s S
add_zero u add_finished v S
add_zero 0 add_zero1 0 R
add_zero 1 add_zero1 1 R
add_zero @ add_zero1 @ R
add_zero x add_zero1 x R
add_zero y add_zero1 y R
add_zero z add_zero1 z R
add_zero s add_zero1 s R
add_zero t add_zero1 t R
add_zero v add_zero1 v R
add_zero w add_zero1 w R
add_zero _ add_zero1 _ R

add_zero1 0 add_zero 0 R
add_zero1 1 add_zero 1 R
add_zero1 @ add_zero @ R
add_zero1 x add_zero x R
add_zero1 y add_zero y R
add_zero1 z add_zero z R
add_zero1 r add_zero r R
add_zero1 s add_zero s R
add_zero1 u add_zero u R
add_zero1 t add_zero t R
add_zero1 v add_zero v R
add_zero1 w add_zero w R
add_zero1 _ add_zero _ R

add_one r add_finished v S
add_one u add_one1 s R
add_one 0 add_one2 0 R
add_one 1 add_one2 1 R
add_one @ add_one2 @ R
add_one x add_one2 x R
add_one y add_one2 y R
add_one z add_one2 z R
add_one s add_one2 s R
add_one t add_one2 t R
add_one v add_one2 v R
add_one w add_one2 w R
add_one _ add_one2 _ R

add_one1 0 carry 0 R
add_one1 1 carry 1 R
add_one1 @ carry @ R
add_one1 x carry x R
add_one1 y carry y R
add_one1 z carry z R
add_one1 r carry r R
add_one1 s carry s R
add_one1 u carry u R
add_one1 t carry t R
add_one1 v carry v R
add_one1 w carry w R
add_one1 _ carry _ R


add_one2 0 add_one 0 R
add_one2 1 add_one 1 R
add_one2 @ add_one @ R
add_one2 x add_one x R
add_one2 y add_one y R
add_one2 z add_one z R
add_one2 r add_one r R
add_one2 s add_one s R
add_one2 u add_one u R
add_one2 t add_one t R
add_one2 v add_one v R
add_one2 w add_one w R
add_one2 _ add_one _ R

carry r add_finished u S
carry _ new_digit_zero u S
carry u carry0 r R

carry0 0 carry 0 R
carry0 1 carry 1 R
carry0 @ carry @ R
carry0 x carry x R
carry0 y carry y R
carry0 z carry z R
carry0 r carry r R
carry0 s carry s R
carry0 u carry u R
carry0 t carry t R
carry0 v carry v R
carry0 w carry w R
carry0 _ carry _ R


add_finished @ add_finished0 @ R
add_finished 0 add_finished1 0 L
add_finished 1 add_finished1 1 L
add_finished x add_finished1 x L
add_finished y add_finished1 y L
add_finished z add_finished1 z L
add_finished r add_finished1 r L
add_finished s add_finished1 s L
add_finished u add_finished1 u L
add_finished t add_finished1 t L
add_finished v add_finished1 v L
add_finished w add_finished1 w L
add_finished _ add_finished1 _ L

add_finished0 0 erase_old_x 0 R
add_finished0 1 erase_old_x 1 R
add_finished0 @ erase_old_x @ R
add_finished0 x erase_old_x x R
add_finished0 y erase_old_x y R
add_finished0 z erase_old_x z R
add_finished0 r erase_old_x r R
add_finished0 s erase_old_x s R
add_finished0 u erase_old_x u R
add_finished0 t erase_old_x t R
add_finished0 v erase_old_x v R
add_finished0 w erase_old_x w R
add_finished0 _ erase_old_x _ R

add_finished1 0 add_finished 0 L
add_finished1 1 add_finished 1 L
add_finished1 @ add_finished @ L
add_finished1 x add_finished x L
add_finished1 y add_finished y L
add_finished1 z add_finished z L
add_finished1 r add_finished r L
add_finished1 s add_finished s L
add_finished1 u add_finished u L
add_finished1 t add_finished t L
add_finished1 v add_finished v L
add_finished1 w add_finished w L
add_finished1 _ add_finished _ L

erase_old_x x erase_old_x1 _ L
erase_old_x z erase_old_x1 y L
erase_old_x 0 erase_old_x2 0 R
erase_old_x 1 erase_old_x2 1 R
erase_old_x @ erase_old_x2 @ R
erase_old_x y erase_old_x2 y R
erase_old_x r erase_old_x2 r R
erase_old_x s erase_old_x2 s R
erase_old_x u erase_old_x2 u R
erase_old_x t erase_old_x2 t R
erase_old_x v erase_old_x2 v R
erase_old_x w erase_old_x2 w R
erase_old_x _ erase_old_x2 _ R

erase_old_x1 0 print_new_x 0 L
erase_old_x1 1 print_new_x 1 L
erase_old_x1 @ print_new_x @ L
erase_old_x1 x print_new_x x L
erase_old_x1 y print_new_x y L
erase_old_x1 z print_new_x z L
erase_old_x1 r print_new_x r L
erase_old_x1 s print_new_x s L
erase_old_x1 u print_new_x u L
erase_old_x1 t print_new_x t L
erase_old_x1 v print_new_x v L
erase_old_x1 w print_new_x w L
erase_old_x1 _ print_new_x _ L

erase_old_x2 0 erase_old_x 0 R
erase_old_x2 1 erase_old_x 1 R
erase_old_x2 @ erase_old_x @ R
erase_old_x2 x erase_old_x x R
erase_old_x2 y erase_old_x y R
erase_old_x2 z erase_old_x z R
erase_old_x2 r erase_old_x r R
erase_old_x2 s erase_old_x s R
erase_old_x2 u erase_old_x u R
erase_old_x2 t erase_old_x t R
erase_old_x2 v erase_old_x v R
erase_old_x2 w erase_old_x w R
erase_old_x2 _ erase_old_x _ R

print_new_x @ print_new_x0 @ R
print_new_x y find_digits z S
print_new_x _ find_digits x S

print_new_x0 0 erase_old_y 0 R
print_new_x0 1 erase_old_y 1 R
print_new_x0 @ erase_old_y @ R
print_new_x0 x erase_old_y x R
print_new_x0 y erase_old_y y R
print_new_x0 z erase_old_y z R
print_new_x0 r erase_old_y r R
print_new_x0 s erase_old_y s R
print_new_x0 u erase_old_y u R
print_new_x0 t erase_old_y t R
print_new_x0 v erase_old_y v R
print_new_x0 w erase_old_y w R
print_new_x0 _ erase_old_y _ R

erase_old_y y erase_old_y0 _ L
erase_old_y 0 erase_old_y1 0 R
erase_old_y 1 erase_old_y1 1 R
erase_old_y @ erase_old_y1 @ R
erase_old_y x erase_old_y1 x R
erase_old_y z erase_old_y1 z R
erase_old_y r erase_old_y1 r R
erase_old_y s erase_old_y1 s R
erase_old_y u erase_old_y1 u R
erase_old_y t erase_old_y1 t R
erase_old_y v erase_old_y1 v R
erase_old_y w erase_old_y1 w R
erase_old_y _ erase_old_y1 _ R

erase_old_y0 0 print_new_y 0 L
erase_old_y0 1 print_new_y 1 L
erase_old_y0 @ print_new_y @ L
erase_old_y0 x print_new_y x L
erase_old_y0 y print_new_y y L
erase_old_y0 z print_new_y z L
erase_old_y0 r print_new_y r L
erase_old_y0 s print_new_y s L
erase_old_y0 u print_new_y u L
erase_old_y0 t print_new_y t L
erase_old_y0 v print_new_y v L
erase_old_y0 w print_new_y w L
erase_old_y0 _ print_new_y _ L

erase_old_y1 0 erase_old_y 0 R
erase_old_y1 1 erase_old_y 1 R
erase_old_y1 @ erase_old_y @ R
erase_old_y1 x erase_old_y x R
erase_old_y1 y erase_old_y y R
erase_old_y1 z erase_old_y z R
erase_old_y1 r erase_old_y r R
erase_old_y1 s erase_old_y s R
erase_old_y1 u erase_old_y u R
erase_old_y1 t erase_old_y t R
erase_old_y1 v erase_old_y v R
erase_old_y1 w erase_old_y w R
erase_old_y1 _ erase_old_y _ R

print_new_y @ new_digit_one @ R
print_new_y 0 reset_new_x y R
print_new_y 1 reset_new_x y R
print_new_y x reset_new_x y R
print_new_y y reset_new_x y R
print_new_y z reset_new_x y R
print_new_y r reset_new_x y R
print_new_y s reset_new_x y R
print_new_y u reset_new_x y R
print_new_y t reset_new_x y R
print_new_y v reset_new_x y R
print_new_y w reset_new_x y R
print_new_y _ reset_new_x y R

reset_new_x _ reset_new_x0 _ R
reset_new_x 0 reset_new_x1 0 R
reset_new_x 1 reset_new_x1 1 R
reset_new_x @ reset_new_x1 @ R
reset_new_x x reset_new_x1 x R
reset_new_x y reset_new_x1 y R
reset_new_x z reset_new_x1 z R
reset_new_x r reset_new_x1 r R
reset_new_x s reset_new_x1 s R
reset_new_x u reset_new_x1 u R
reset_new_x t reset_new_x1 t R
reset_new_x v reset_new_x1 v R
reset_new_x w reset_new_x1 w R

reset_new_x0 0 flag_result_digits x S
reset_new_x0 1 flag_result_digits x S
reset_new_x0 @ flag_result_digits x S
reset_new_x0 x flag_result_digits x S
reset_new_x0 y flag_result_digits x S
reset_new_x0 z flag_result_digits x S
reset_new_x0 r flag_result_digits x S
reset_new_x0 s flag_result_digits x S
reset_new_x0 u flag_result_digits x S
reset_new_x0 t flag_result_digits x S
reset_new_x0 v flag_result_digits x S
reset_new_x0 w flag_result_digits x S
reset_new_x0 _ flag_result_digits x S

reset_new_x1 0 reset_new_x 0 R
reset_new_x1 1 reset_new_x 1 R
reset_new_x1 @ reset_new_x @ R
reset_new_x1 x reset_new_x x R
reset_new_x1 y reset_new_x y R
reset_new_x1 z reset_new_x z R
reset_new_x1 r reset_new_x r R
reset_new_x1 s reset_new_x s R
reset_new_x1 u reset_new_x u R
reset_new_x1 t reset_new_x t R
reset_new_x1 v reset_new_x v R
reset_new_x1 w reset_new_x w R
reset_new_x1 _ reset_new_x _ R

flag_result_digits s flag_result_digits0 t R
flag_result_digits v flag_result_digits0 w R
flag_result_digits 0 flag_result_digits1 0 R
flag_result_digits 1 flag_result_digits1 1 R
flag_result_digits @ flag_result_digits1 @ R
flag_result_digits x flag_result_digits1 x R
flag_result_digits y flag_result_digits1 y R
flag_result_digits z flag_result_digits1 z R
flag_result_digits r flag_result_digits1 r R
flag_result_digits u flag_result_digits1 u R
flag_result_digits t flag_result_digits1 t R
flag_result_digits w flag_result_digits1 w R
flag_result_digits _ flag_result_digits1 _ R

flag_result_digits0 0 unflag_result_digits 0 R
flag_result_digits0 1 unflag_result_digits 1 R
flag_result_digits0 @ unflag_result_digits @ R
flag_result_digits0 x unflag_result_digits x R
flag_result_digits0 y unflag_result_digits y R
flag_result_digits0 z unflag_result_digits z R
flag_result_digits0 r unflag_result_digits r R
flag_result_digits0 s unflag_result_digits s R
flag_result_digits0 u unflag_result_digits u R
flag_result_digits0 t unflag_result_digits t R
flag_result_digits0 v unflag_result_digits v R
flag_result_digits0 w unflag_result_digits w R
flag_result_digits0 _ unflag_result_digits _ R

flag_result_digits1 0 flag_result_digits 0 R
flag_result_digits1 1 flag_result_digits 1 R
flag_result_digits1 @ flag_result_digits @ R
flag_result_digits1 x flag_result_digits x R
flag_result_digits1 y flag_result_digits y R
flag_result_digits1 z flag_result_digits z R
flag_result_digits1 r flag_result_digits r R
flag_result_digits1 s flag_result_digits s R
flag_result_digits1 u flag_result_digits u R
flag_result_digits1 t flag_result_digits t R
flag_result_digits1 v flag_result_digits v R
flag_result_digits1 w flag_result_digits w R
flag_result_digits1 _ flag_result_digits _ R


unflag_result_digits s unflag_result_digits0 r R
unflag_result_digits v unflag_result_digits0 u R
unflag_result_digits 0 find_digits 0 S
unflag_result_digits 1 find_digits 1 S
unflag_result_digits @ find_digits @ S
unflag_result_digits x find_digits x S
unflag_result_digits y find_digits y S
unflag_result_digits z find_digits z S
unflag_result_digits r find_digits r S
unflag_result_digits u find_digits u S
unflag_result_digits t find_digits t S
unflag_result_digits w find_digits w S
unflag_result_digits _ find_digits _ S

unflag_result_digits0 0 unflag_result_digits 0 R
unflag_result_digits0 1 unflag_result_digits 1 R
unflag_result_digits0 @ unflag_result_digits @ R
unflag_result_digits0 x unflag_result_digits x R
unflag_result_digits0 y unflag_result_digits y R
unflag_result_digits0 z unflag_result_digits z R
unflag_result_digits0 r unflag_result_digits r R
unflag_result_digits0 s unflag_result_digits s R
unflag_result_digits0 u unflag_result_digits u R
unflag_result_digits0 t unflag_result_digits t R
unflag_result_digits0 v unflag_result_digits v R
unflag_result_digits0 w unflag_result_digits w R
unflag_result_digits0 _ unflag_result_digits _ R

new_digit_zero @ print_zero_digit @ R
new_digit_zero 0 new_digit_zero 0 L
new_digit_zero 1 new_digit_zero 1 L
new_digit_zero x new_digit_zero x L
new_digit_zero y new_digit_zero y L
new_digit_zero z new_digit_zero z L
new_digit_zero r new_digit_zero r L
new_digit_zero s new_digit_zero s L
new_digit_zero u new_digit_zero u L
new_digit_zero t new_digit_zero t L
new_digit_zero v new_digit_zero v L
new_digit_zero w new_digit_zero w L
new_digit_zero _ new_digit_zero _ L

print_zero_digit 0 print_zero_digit0 0 R
print_zero_digit 1 print_zero_digit0 1 R
print_zero_digit _ print_zero_digit1 0 R

print_zero_digit0 0 print_zero_digit _ R
print_zero_digit0 1 print_zero_digit _ R
print_zero_digit0 @ print_zero_digit _ R
print_zero_digit0 x print_zero_digit _ R
print_zero_digit0 y print_zero_digit _ R
print_zero_digit0 z print_zero_digit _ R
print_zero_digit0 r print_zero_digit _ R
print_zero_digit0 s print_zero_digit _ R
print_zero_digit0 u print_zero_digit _ R
print_zero_digit0 t print_zero_digit _ R
print_zero_digit0 v print_zero_digit _ R
print_zero_digit0 w print_zero_digit _ R
print_zero_digit0 _ print_zero_digit _ R


print_zero_digit1 0 print_zero_digit2 0 R
print_zero_digit1 1 print_zero_digit2 1 R
print_zero_digit1 @ print_zero_digit2 @ R
print_zero_digit1 x print_zero_digit2 x R
print_zero_digit1 y print_zero_digit2 y R
print_zero_digit1 z print_zero_digit2 z R
print_zero_digit1 r print_zero_digit2 r R
print_zero_digit1 s print_zero_digit2 s R
print_zero_digit1 u print_zero_digit2 u R
print_zero_digit1 t print_zero_digit2 t R
print_zero_digit1 v print_zero_digit2 v R
print_zero_digit1 w print_zero_digit2 w R
print_zero_digit1 _ print_zero_digit2 _ R

print_zero_digit2 0 cleanup 0 R
print_zero_digit2 1 cleanup 1 R
print_zero_digit2 @ cleanup @ R
print_zero_digit2 x cleanup x R
print_zero_digit2 y cleanup y R
print_zero_digit2 z cleanup z R
print_zero_digit2 r cleanup r R
print_zero_digit2 s cleanup s R
print_zero_digit2 u cleanup u R
print_zero_digit2 t cleanup t R
print_zero_digit2 v cleanup v R
print_zero_digit2 w cleanup w R
print_zero_digit2 _ cleanup _ R

new_digit_one @ print_one_digit @ R
new_digit_one 0 new_digit_one 0 L
new_digit_one 1 new_digit_one 1 L
new_digit_one x new_digit_one x L
new_digit_one y new_digit_one y L
new_digit_one z new_digit_one z L
new_digit_one r new_digit_one r L
new_digit_one s new_digit_one s L
new_digit_one u new_digit_one u L
new_digit_one t new_digit_one t L
new_digit_one v new_digit_one v L
new_digit_one w new_digit_one w L
new_digit_one _ new_digit_one _ L

print_one_digit 0 print_one_digit0 0 R
print_one_digit 1 print_one_digit0 1 R
print_one_digit _ print_one_digit1 1 R

print_one_digit0 0 print_one_digit 0 R
print_one_digit0 1 print_one_digit 1 R
print_one_digit0 @ print_one_digit @ R
print_one_digit0 x print_one_digit x R
print_one_digit0 y print_one_digit y R
print_one_digit0 z print_one_digit z R
print_one_digit0 r print_one_digit r R
print_one_digit0 s print_one_digit s R
print_one_digit0 u print_one_digit u R
print_one_digit0 t print_one_digit t R
print_one_digit0 v print_one_digit v R
print_one_digit0 w print_one_digit w R
print_one_digit0 _ print_one_digit _ R

print_one_digit1 0 print_one_digit2 0 R
print_one_digit1 1 print_one_digit2 1 R
print_one_digit1 @ print_one_digit2 @ R
print_one_digit1 x print_one_digit2 x R
print_one_digit1 y print_one_digit2 y R
print_one_digit1 z print_one_digit2 z R
print_one_digit1 r print_one_digit2 r R
print_one_digit1 s print_one_digit2 s R
print_one_digit1 u print_one_digit2 u R
print_one_digit1 t print_one_digit2 t R
print_one_digit1 v print_one_digit2 v R
print_one_digit1 w print_one_digit2 w R
print_one_digit1 _ print_one_digit2 _ R

print_one_digit2 0 cleanup 0 R
print_one_digit2 1 cleanup 1 R
print_one_digit2 @ cleanup @ R
print_one_digit2 x cleanup x R
print_one_digit2 y cleanup y R
print_one_digit2 z cleanup z R
print_one_digit2 r cleanup r R
print_one_digit2 s cleanup s R
print_one_digit2 u cleanup u R
print_one_digit2 t cleanup t R
print_one_digit2 v cleanup v R
print_one_digit2 w cleanup w R
print_one_digit2 _ cleanup _ R

cleanup _ new _ S
cleanup 0 cleanup1 _ R
cleanup 1 cleanup1 _ R
cleanup @ cleanup1 _ R
cleanup x cleanup1 _ R
cleanup y cleanup1 _ R
cleanup z cleanup1 _ R
cleanup r cleanup1 _ R
cleanup s cleanup1 _ R
cleanup u cleanup1 _ R
cleanup t cleanup1 _ R
cleanup v cleanup1 _ R
cleanup w cleanup1 _ R

cleanup1 0 cleanup 0 R
cleanup1 1 cleanup 1 R
cleanup1 @ cleanup @ R
cleanup1 x cleanup x R
cleanup1 y cleanup y R
cleanup1 z cleanup z R
cleanup1 r cleanup r R
cleanup1 s cleanup s R
cleanup1 u cleanup u R
cleanup1 t cleanup t R
cleanup1 v cleanup v R
cleanup1 w cleanup w R
cleanup1 _ cleanup _ R


mock 0 mock 0 R
mock 1 mock 1 R
mock @ mock @ R
mock x mock x R
mock y mock y R
mock z mock z R
mock r mock r R
mock s mock s R
mock u mock u R
mock t mock t R
mock v mock v R
mock w mock w R
mock _ mock _ R