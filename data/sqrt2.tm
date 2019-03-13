states 56
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
new_digit_is_one
add_finished1
erase_old_x
reset_new_x
print_new_x
print_new_x0
accept +
reject -
alphabet 10 0 1 @ x y z r s u v


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
new v new v L
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
mark_digits_x v mark_digits x R
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
mark_digits_z v mark_digits_z1 z R
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
mark_digits_z1 v mark_digits_z2 v R
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
mark_digits_z2 v find_x r S
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
find_x v find_x1 v L
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
find_x1 v find_x v L
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
first_r v first_r2 v R
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
first_r1 v last_r v R
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
first_r2 v first_r v R
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
last_r1 v last_r v R
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
last_r2 v last_r3 v R
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
find_digits v find_digits2 v L
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
find_digits1 v find_1st_digit v R
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
find_digits2 v find_digits v L
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
find_1st_digit1 v find_1st_digit v R
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
found_1st_digit0 v found_1st_digit1 v R
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
found_1st_digit1 v find_2nd_digit v R
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
find_2nd_digit0 v find_2nd_digit v R
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
add_zero v add_zero1 v R
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
add_zero1 v add_zero v R
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
add_one v add_one2 v R
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
add_one1 v carry v R
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
add_one2 v add_one v R
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
carry0 v carry v R
carry0 _ carry _ R


add_finished @ add_finished0 @ R
add_finished 0 add_finished1 0 L
add_finished 1 add_finished1 1 L
add_finished @ add_finished1 @ L
add_finished x add_finished1 x L
add_finished y add_finished1 y L
add_finished z add_finished1 z L
add_finished r add_finished1 r L
add_finished s add_finished1 s L
add_finished u add_finished1 u L
add_finished v add_finished1 v L
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
add_finished0 v erase_old_x v R
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
add_finished1 v add_finished v L
add_finished1 _ add_finished _ L

erase_old_x x erase_old_x1 _ L
erase_old_x z erase_old_x1 y L
erase_old_x 0 erase_old_x2 0 R
erase_old_x 1 erase_old_x2 1 R
erase_old_x @ erase_old_x2 @ R
erase_old_x x erase_old_x2 x R
erase_old_x y erase_old_x2 y R
erase_old_x z erase_old_x2 z R
erase_old_x r erase_old_x2 r R
erase_old_x s erase_old_x2 s R
erase_old_x u erase_old_x2 u R
erase_old_x v erase_old_x2 v R
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
erase_old_x1 v print_new_x v L
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
erase_old_x2 v erase_old_x v R
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
print_new_x0 v erase_old_y v R
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
erase_old_y v erase_old_y1 v R
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
erase_old_y0 v print_new_y v L
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
erase_old_y1 v erase_old_y v R
erase_old_y1 _ erase_old_y _ R

print_new_y @ new_digit_is_one @ R
print_new_y 0 reset_new_x y R
print_new_y 1 reset_new_x y R
print_new_y @ reset_new_x y R
print_new_y x reset_new_x y R
print_new_y y reset_new_x y R
print_new_y z reset_new_x y R
print_new_y r reset_new_x y R
print_new_y s reset_new_x y R
print_new_y u reset_new_x y R
print_new_y v reset_new_x y R
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
reset_new_x v reset_new_x1 v R

reset_new_x0 0 flag_result_digits x S
reset_new_x0 1 flag_result_digits x S
reset_new_x0 @ flag_result_digits x S
reset_new_x0 x flag_result_digits x S
reset_new_x0 y flag_result_digits x S
reset_new_x0 z flag_result_digits x S
reset_new_x0 r flag_result_digits x S
reset_new_x0 s flag_result_digits x S
reset_new_x0 u flag_result_digits x S
reset_new_x0 v flag_result_digits x S
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
reset_new_x1 v reset_new_x v R
reset_new_x1 _ reset_new_x _ R



mock 0 mock 0 R
mock 1 mock 1 R
mock @ mock @ R
mock x mock x R
mock y mock y R
mock z mock z R
mock r mock r R
mock s mock s R
mock u mock u R
mock v mock v R
mock _ mock _ R