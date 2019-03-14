states 70
start
move_ptr_left
goto_start_and_execute
shift[
shift+
shift
shift-
decrement_cell
start_execution
ipshift-
ipshift#
ipshift^
move_ip_forward
increment_cell
write_old_symbol>
ipshift<
ipshift+
ipshift]
gostart_execution
write_old_symbol+
shift<
shift]
write_old_symbol[
write_old_symbol]
write_old_symbol-
decrement_next
mock
move_ptr_right
ipshift>
write_old_symbol<
increment_next
loop_end
shift>
start_tape
start_tape1
goto_start
move_ip
create_ip
loop_start
ipshift[
copy_ptr_left0
copy_ptr_left
copy_ptr_left1
copy_ptr_right0
copy_ptr_right
copy_ptr_right1
shift_to_dollar>
shift_to_dollar<
start_shift
shift_to_dollar+
check_ptr_close
shift_to_dollar[
shift_to_dollar-
branch_close
loop
shift_to_dollar]
go_to_bracket
shift_left+
shift_left[
shift_left>
shift_left<
jump
shift_left-
shift_left]
go_to_close_bracket
check_ptr_open
goto_new_start
branch_open
accept +
reject -
alphabet 12 < > + - [ ] 0 1 @ ^ # $
start _ accept _ R
start < shift< @ R
start > shift> @ R
start + shift+ @ R
start - shift- @ R
start [ shift[ @ R
start ] shift] @ R

shift< _ start_tape < R
shift< < shift< < R
shift< > shift> < R
shift< + shift+ < R
shift< - shift- < R
shift< [ shift[ < R
shift< ] shift] < R

shift> _ start_tape > R
shift> < shift< > R
shift> > shift> > R
shift> + shift+ > R
shift> - shift- > R
shift> [ shift[ > R
shift> ] shift] > R

shift+ _ start_tape + R
shift+ < shift< + R
shift+ > shift> + R
shift+ + shift+ + R
shift+ - shift- + R
shift+ [ shift[ + R
shift+ ] shift] + R

shift- _ start_tape - R
shift- < shift< - R
shift- > shift> - R
shift- + shift+ - R
shift- - shift- - R
shift- [ shift[ - R
shift- ] shift] - R

shift[ _ start_tape [ R
shift[ < shift< [ R
shift[ > shift> [ R
shift[ + shift+ [ R
shift[ - shift- [ R
shift[ [ shift[ [ R
shift[ ] shift] [ R


shift] _ start_tape ] R
shift] < shift< ] R
shift] > shift> ] R
shift] + shift+ ] R
shift] - shift- ] R
shift] [ shift[ ] R
shift] ] shift] ] R


start_tape _ start_tape1 # R
start_tape1 _ goto_start ^ L

goto_start @ create_ip @ R
goto_start < goto_start < L
goto_start > goto_start > L
goto_start + goto_start + L
goto_start - goto_start - L
goto_start [ goto_start [ L
goto_start ] goto_start ] L
goto_start 0 goto_start 0 L
goto_start 1 goto_start 1 L
goto_start ^ goto_start ^ L
goto_start # goto_start # L

create_ip < ipshift< $ R
create_ip > ipshift> $ R
create_ip + ipshift+ $ R
create_ip - ipshift- $ R
create_ip [ ipshift[ $ R
create_ip ] ipshift] $ R

ipshift< _ goto_start_and_execute < L
ipshift< < ipshift< < R
ipshift< > ipshift> < R
ipshift< + ipshift+ < R
ipshift< - ipshift- < R
ipshift< [ ipshift[ < R
ipshift< ] ipshift] < R
ipshift< # ipshift# < R
ipshift< ^ ipshift^ < R


ipshift> _ goto_start_and_execute > L
ipshift> < ipshift< > R
ipshift> > ipshift> > R
ipshift> + ipshift+ > R
ipshift> - ipshift- > R
ipshift> [ ipshift[ > R
ipshift> ] ipshift] > R
ipshift> # ipshift# > R
ipshift> ^ ipshift^ > R

ipshift+ _ goto_start_and_execute + L
ipshift+ < ipshift< + R
ipshift+ > ipshift> + R
ipshift+ + ipshift+ + R
ipshift+ - ipshift- + R
ipshift+ [ ipshift[ + R
ipshift+ ] ipshift] + R
ipshift+ # ipshift# + R
ipshift+ ^ ipshift^ + R


ipshift- _ goto_start_and_execute - L
ipshift- < ipshift< - R
ipshift- > ipshift> - R
ipshift- + ipshift+ - R
ipshift- - ipshift- - R
ipshift- [ ipshift[ - R
ipshift- ] ipshift] - R
ipshift- # ipshift# - R
ipshift- ^ ipshift^ - R


ipshift[ _ goto_start_and_execute [ L
ipshift[ < ipshift< [ R
ipshift[ > ipshift> [ R
ipshift[ + ipshift+ [ R
ipshift[ - ipshift- [ R
ipshift[ [ ipshift[ [ R
ipshift[ ] ipshift] [ R
ipshift[ # ipshift# [ R
ipshift[ ^ ipshift^ [ R

ipshift] _ goto_start_and_execute ] L
ipshift] < ipshift< ] R
ipshift] > ipshift> ] R
ipshift] + ipshift+ ] R
ipshift] - ipshift- ] R
ipshift] [ ipshift[ ] R
ipshift] ] ipshift] ] R
ipshift] # ipshift# ] R
ipshift] ^ ipshift^ ] R

ipshift# _ goto_start_and_execute # L
ipshift# < ipshift< # R
ipshift# > ipshift> # R
ipshift# + ipshift+ # R
ipshift# - ipshift- # R
ipshift# [ ipshift[ # R
ipshift# ] ipshift] # R
ipshift# # ipshift# # R
ipshift# ^ ipshift^ # R

ipshift^ _ goto_start_and_execute ^ L
ipshift^ < ipshift< ^ R
ipshift^ > ipshift> ^ R
ipshift^ + ipshift+ ^ R
ipshift^ - ipshift- ^ R
ipshift^ [ ipshift[ ^ R
ipshift^ ] ipshift] ^ R
ipshift^ # ipshift# ^ R
ipshift^ ^ ipshift^ ^ R

goto_start_and_execute $ start_execution $ R
goto_start_and_execute < goto_start_and_execute < L
goto_start_and_execute > goto_start_and_execute > L
goto_start_and_execute + goto_start_and_execute + L
goto_start_and_execute - goto_start_and_execute - L
goto_start_and_execute [ goto_start_and_execute [ L
goto_start_and_execute ] goto_start_and_execute ] L
goto_start_and_execute 0 goto_start_and_execute 0 L
goto_start_and_execute 1 goto_start_and_execute 1 L
goto_start_and_execute ^ goto_start_and_execute ^ L
goto_start_and_execute # goto_start_and_execute # L

start_execution < move_ptr_left < R
start_execution > move_ptr_right > R
start_execution + increment_cell + R
start_execution - decrement_cell - R
start_execution [ loop_start [ R
start_execution ] loop_end ] R
start_execution # accept # R


increment_cell ^ increment_next ^ R
increment_cell < increment_cell < R
increment_cell > increment_cell > R
increment_cell + increment_cell + R
increment_cell - increment_cell - R
increment_cell [ increment_cell [ R
increment_cell ] increment_cell ] R
increment_cell 0 increment_cell 0 R
increment_cell 1 increment_cell 1 R
increment_cell @ increment_cell @ R
increment_cell # increment_cell # R

increment_next _ move_ip 1 L
increment_next 0 move_ip 1 L
increment_next 1 move_ip 1 L

decrement_cell ^ decrement_next ^ R
decrement_cell < decrement_cell < R
decrement_cell > decrement_cell > R
decrement_cell + decrement_cell + R
decrement_cell - decrement_cell - R
decrement_cell [ decrement_cell [ R
decrement_cell ] decrement_cell ] R
decrement_cell 0 decrement_cell 0 R
decrement_cell 1 decrement_cell 1 R
decrement_cell @ decrement_cell @ R
decrement_cell # decrement_cell # R

decrement_next _ move_ip 0 L
decrement_next 0 move_ip 0 L
decrement_next 1 move_ip 0 L


move_ip $ move_ip_forward $ R
move_ip < move_ip < L
move_ip > move_ip > L
move_ip + move_ip + L
move_ip - move_ip - L
move_ip [ move_ip [ L
move_ip ] move_ip ] L
move_ip 0 move_ip 0 L
move_ip 1 move_ip 1 L
move_ip @ move_ip @ L
move_ip ^ move_ip ^ L
move_ip # move_ip # L

move_ip_forward < write_old_symbol< $ L
move_ip_forward > write_old_symbol> $ L
move_ip_forward + write_old_symbol+ $ L
move_ip_forward - write_old_symbol- $ L
move_ip_forward [ write_old_symbol[ $ L
move_ip_forward ] write_old_symbol] $ L


write_old_symbol< $ gostart_execution < R
write_old_symbol> $ gostart_execution > R
write_old_symbol+ $ gostart_execution + R
write_old_symbol- $ gostart_execution - R
write_old_symbol[ $ gostart_execution [ R
write_old_symbol] $ gostart_execution ] R

gostart_execution $ start_execution $ R

move_ptr_left ^ copy_ptr_left ^ L
move_ptr_left < move_ptr_left < R
move_ptr_left > move_ptr_left > R
move_ptr_left + move_ptr_left + R
move_ptr_left - move_ptr_left - R
move_ptr_left [ move_ptr_left [ R
move_ptr_left ] move_ptr_left ] R
move_ptr_left 0 move_ptr_left 0 R
move_ptr_left 1 move_ptr_left 1 R
move_ptr_left @ move_ptr_left @ R
move_ptr_left # move_ptr_left # R
move_ptr_left $ move_ptr_left $ R

copy_ptr_left 0 copy_ptr_left0 ^ R
copy_ptr_left 1 copy_ptr_left1 ^ R
copy_ptr_left # goto_start_and_execute # L

copy_ptr_left0 ^ goto_start_and_execute 0 L
copy_ptr_left1 ^ goto_start_and_execute 1 L

move_ptr_right ^ copy_ptr_right ^ R
move_ptr_right < move_ptr_right < R
move_ptr_right > move_ptr_right > R
move_ptr_right + move_ptr_right + R
move_ptr_right - move_ptr_right - R
move_ptr_right [ move_ptr_right [ R
move_ptr_right ] move_ptr_right ] R
move_ptr_right 0 move_ptr_right 0 R
move_ptr_right 1 move_ptr_right 1 R
move_ptr_right @ move_ptr_right @ R
move_ptr_right # move_ptr_right # R
move_ptr_right $ move_ptr_right $ R

copy_ptr_right 0 copy_ptr_right0 ^ L
copy_ptr_right 1 copy_ptr_right1 ^ L
copy_ptr_right _ copy_ptr_right0 ^ L

copy_ptr_right0 ^ move_ip 0 L
copy_ptr_right1 ^ move_ip 1 L

loop_start < check_ptr_open < L
loop_start > check_ptr_open > L
loop_start + check_ptr_open + L
loop_start - check_ptr_open - L
loop_start [ check_ptr_open [ L
loop_start ] check_ptr_open ] L

check_ptr_open ^ branch_open ^ R
check_ptr_open < check_ptr_open < R
check_ptr_open > check_ptr_open > R
check_ptr_open + check_ptr_open + R
check_ptr_open - check_ptr_open - R
check_ptr_open [ check_ptr_open [ R
check_ptr_open ] check_ptr_open ] R
check_ptr_open 0 check_ptr_open 0 R
check_ptr_open 1 check_ptr_open 1 R
check_ptr_open @ check_ptr_open @ R
check_ptr_open # check_ptr_open # R

branch_open 0 jump 0 L
branch_open 1 move_ip 1 L
branch_open _ jump 0 L

jump $ go_to_close_bracket $ R
jump < jump < L
jump > jump > L
jump + jump + L
jump - jump - L
jump [ jump [ L
jump ] jump ] L
jump 0 jump 0 L
jump 1 jump 1 L
jump ^ jump ^ L
jump # jump # L

go_to_close_bracket ] shift_left] $ L
go_to_close_bracket < go_to_close_bracket < R
go_to_close_bracket > go_to_close_bracket > R
go_to_close_bracket + go_to_close_bracket + R
go_to_close_bracket - go_to_close_bracket - R
go_to_close_bracket [ go_to_close_bracket [ R
go_to_close_bracket @ go_to_close_bracket @ R
go_to_close_bracket ^ go_to_close_bracket ^ R
go_to_close_bracket # go_to_close_bracket # R
go_to_close_bracket $ go_to_close_bracket $ R


shift_left< < shift_left< < L
shift_left< > shift_left> < L
shift_left< + shift_left+ < L
shift_left< - shift_left- < L
shift_left< [ shift_left[ < L
shift_left< ] shift_left] < L

shift_left> < shift_left< > L
shift_left> > shift_left> > L
shift_left> + shift_left+ > L
shift_left> - shift_left- > L
shift_left> [ shift_left[ > L
shift_left> ] shift_left] > L

shift_left+ < shift_left< + L
shift_left+ > shift_left> + L
shift_left+ + shift_left+ + L
shift_left+ - shift_left- + L
shift_left+ [ shift_left[ + L
shift_left+ ] shift_left] + L

shift_left- < shift_left< - L
shift_left- > shift_left> - L
shift_left- + shift_left+ - L
shift_left- - shift_left- - L
shift_left- [ shift_left[ - L
shift_left- ] shift_left] - L

shift_left[ $ goto_new_start [ R
shift_left[ < shift_left< [ L
shift_left[ > shift_left> [ L
shift_left[ + shift_left+ [ L
shift_left[ - shift_left- [ L
shift_left[ [ shift_left[ [ L
shift_left[ ] shift_left] [ L

shift_left] < shift_left< ] L
shift_left] > shift_left> ] L
shift_left] + shift_left+ ] L
shift_left] - shift_left- ] L
shift_left] [ shift_left[ ] L
shift_left] ] shift_left] ] L

goto_new_start $ start_execution $ R
goto_new_start < goto_new_start < R
goto_new_start > goto_new_start > R
goto_new_start + goto_new_start + R
goto_new_start - goto_new_start - R
goto_new_start [ goto_new_start [ R
goto_new_start ] goto_new_start ] R



loop_end # check_ptr_close # R
loop_end < check_ptr_close < R
loop_end > check_ptr_close > R
loop_end + check_ptr_close + R
loop_end - check_ptr_close - R
loop_end [ check_ptr_close [ R
loop_end ] check_ptr_close ] R

check_ptr_close ^ branch_close ^ R
check_ptr_close < check_ptr_close < R
check_ptr_close > check_ptr_close > R
check_ptr_close + check_ptr_close + R
check_ptr_close - check_ptr_close - R
check_ptr_close [ check_ptr_close [ R
check_ptr_close ] check_ptr_close ] R
check_ptr_close 0 check_ptr_close 0 R
check_ptr_close 1 check_ptr_close 1 R
check_ptr_close @ check_ptr_close @ R
check_ptr_close # check_ptr_close # R

branch_close 0 loop 0 L
branch_close 1 move_ip 1 L
branch_close _ loop 0 L

loop $ go_to_bracket $ L
loop < loop < L
loop > loop > L
loop + loop + L
loop - loop - L
loop [ loop [ L
loop ] loop ] L
loop 0 loop 0 L
loop 1 loop 1 L
loop ^ loop ^ L
loop # loop # L

go_to_bracket [ start_shift [ R
go_to_bracket < go_to_bracket < L
go_to_bracket > go_to_bracket > L
go_to_bracket + go_to_bracket + L
go_to_bracket - go_to_bracket - L
go_to_bracket ] go_to_bracket ] L

start_shift $ goto_start_and_execute $ R
start_shift < shift_to_dollar< $ R
start_shift > shift_to_dollar> $ R
start_shift + shift_to_dollar+ $ R
start_shift - shift_to_dollar- $ R
start_shift [ shift_to_dollar[ $ R
start_shift ] shift_to_dollar] $ R


shift_to_dollar< $ goto_start_and_execute < R
shift_to_dollar< < shift_to_dollar< < R
shift_to_dollar< > shift_to_dollar< < R
shift_to_dollar< + shift_to_dollar< < R
shift_to_dollar< - shift_to_dollar< < R
shift_to_dollar< [ shift_to_dollar< < R
shift_to_dollar< ] shift_to_dollar< < R

shift_to_dollar> $ goto_start_and_execute > R
shift_to_dollar> < shift_to_dollar< > R
shift_to_dollar> > shift_to_dollar> > R
shift_to_dollar> + shift_to_dollar+ > R
shift_to_dollar> - shift_to_dollar- > R
shift_to_dollar> [ shift_to_dollar[ > R
shift_to_dollar> ] shift_to_dollar] > R

shift_to_dollar+ $ goto_start_and_execute + R
shift_to_dollar+ < shift_to_dollar< + R
shift_to_dollar+ > shift_to_dollar> + R
shift_to_dollar+ + shift_to_dollar+ + R
shift_to_dollar+ - shift_to_dollar- + R
shift_to_dollar+ [ shift_to_dollar[ + R
shift_to_dollar+ ] shift_to_dollar] + R

shift_to_dollar- $ goto_start_and_execute - R
shift_to_dollar- < shift_to_dollar< - R
shift_to_dollar- > shift_to_dollar> - R
shift_to_dollar- + shift_to_dollar+ - R
shift_to_dollar- - shift_to_dollar- - R
shift_to_dollar- [ shift_to_dollar[ - R
shift_to_dollar- ] shift_to_dollar] - R

shift_to_dollar[ $ goto_start_and_execute [ R
shift_to_dollar[ < shift_to_dollar< [ R
shift_to_dollar[ > shift_to_dollar> [ R
shift_to_dollar[ + shift_to_dollar+ [ R
shift_to_dollar[ - shift_to_dollar- [ R
shift_to_dollar[ [ shift_to_dollar[ [ R
shift_to_dollar[ ] shift_to_dollar] [ R


shift_to_dollar] $ goto_start_and_execute ] R
shift_to_dollar] < shift_to_dollar< [ R
shift_to_dollar] > shift_to_dollar> [ R
shift_to_dollar] + shift_to_dollar+ [ R
shift_to_dollar] - shift_to_dollar- [ R
shift_to_dollar] [ shift_to_dollar[ [ R
shift_to_dollar] ] shift_to_dollar] [ R


mock < mock < R
mock > mock > R
mock + mock + R
mock - mock - R
mock [ mock [ R
mock ] mock ] R
mock 0 mock 0 R
mock 1 mock 1 R
mock @ mock @ R
mock ^ mock ^ R
mock # mock # R
mock $ mock $ R