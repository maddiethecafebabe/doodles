
	.globl main
	.text
main:
	la $a0, fname
	la $a1, input_buf
	lw $a2, input_buf_len
	jal read_input_file

	move $s0, $v0 # buf
	move $s1, $v1 # cnt_read
	move $a0, $s0
	
	jal get_binary_digits_len
	move $s2, $v0 # len(line)
	sw $s2 line_len
	
	# count(lines)
	addi $t0, $s2, 1
	divu $s4, $s1, $t0
	sw $s4, line_count
	
	li $s3, 0
main_loop_1:
	move $a0, $s3 # index
	move $a1, $s0 # buf
	move $a2, $s2 # len(line)
	move $a3, $s4 # count(lines)
	jal column_sum_at_index
	
	# push to stack
	subi $sp, $sp, 4
	sw $v0, ($sp)
	
	addi $s3, $s3, 1
	blt $s3, $s2, main_loop_1

	# $t0 = cnt_lines / 2
	li $t0, 2
	divu $t0, $s4, $t0
	li $s3, 0
	li $t6, 0
main_loop_2:
	lw $t1, ($sp)
	addi $sp, $sp, 4

	li $t2, 1	
	bge $t1, $t0, one
	li $t2, 0
one:		
	sllv $t5, $t2, $s3
	or $t6, $t6, $t5
	
	addi $s3, $s3, 1
	blt $s3, $s2, main_loop_2

	move $s5, $t6 # $s5 = gamma
	not $s6, $s5 # $s6 = ~gamma
	move $a0, $s2
	jal make_mask
	and $s6, $s6, $v0
	
	sw $s5, part1_gamma
	sw $s6, part1_epsilon

	mul $t5, $s5, $s6
	sw $t5, part1_solution
	
	jal do_part2
	sw $v0, part2_solution

	jal print_solutions
exit:
	li $v0, 10
	syscall

do_part2:
	# get oxygen
	subi $sp, $sp, 4
	sw $ra, ($sp)
	
	# copy the input buf over to the scratchpad0	
	la $a0, input_buf
	la $a1, scratchpad0
	move $a2, $s4 # max linecnt
	jal linecpy

	la $a0, scratchpad0
	la $a1, scratchpad1
	move $a2, $s4
	li $a3, 0
	la $t8, most_common_at_index
	jal line_find_match
	sw $v0, part2_o_rate

	lw $ra, ($sp)
	addi $sp, $sp, 4
	jr $ra


# $a0: buf
# $a1: outbuf
# $a2: cur_count
# $a3: cur_index
# $t8: callback
# $v0: result number
line_find_match:
	subi $sp, $sp, 24
	sw $s0, ($sp)
	sw $s1, 4($sp)
	sw $s2, 8($sp)
	sw $s3, 12($sp)
	sw $ra, 16($sp)
	sw $s4, 20($sp)

	move $s0, $a0 # buf
	move $s1, $a1 # outbuf
	move $s2, $a2 # cur_count
	move $s3, $a3 # cur_index

line_find_match_loop:
	lw $t0, line_len
	move $a0, $s3
	move $a1, $s0
	jalr $t8


	move $a0, $v0
	li $v0, 1
	syscall
	move $v0, $a0

	move $t0, $s3 # index_offset
	move $t1, $zero # line_offset
	move $t2, $zero # cur_iter_cnt
	move $t3, $zero # new_dst_count
	move $s4, $v0 # most common/least digit
line_find_match_copy_loop:
	bge $t2, $s2, line_find_match_copy_loop_exit	
	
	# line_offset = (line_len + 1) * cur_iter_cnt
	lw $t1, line_len
	addi $t1, $t1, 1
	mul $t1, $t1, $t2
	
	# construct the pointer offset in $t7
	# ptr_offset = line_offset + index_offset
	add $t7, $t1, $t0
	
	# cur_index_char = $t6 = ptr[ptr_offset]
	add $t6, $s0, $t7
	lb $t6, ($t6)
	
	subi $t6, $t6, '0'
	bne $t6, $s4, line_find_match_copy_loop_tail
	
	# we found a match!
	addi $t3, $t3, 1
	
	# copy it over
	
	# ptr offset to start of line
	# $t1
	
	# src = $s0 + line_offset, dst = $s1 + line_offset, len = line_len, + 1 newline
	li $t7, 0 # idx
lfmcl_copy:
	lw $t6, line_len
	bge $t7, $t6, lfmcl_copy_end
	
	add $t6, $t1, $t7 # ptroffset = line_offset + idx
	add $t5, $s0, $t6 # src + ptroff
	lb $t4, ($t5) # src byte
	add $t5, $s1, $t6
	sb $t4, ($t5)
	
lfmcl_copy_tail:
	addi $t7, $t7, 1
	b lfmcl_copy
lfmcl_copy_end:
	li $t4, '\n'
	sb $t4, 1($t5)
line_find_match_copy_loop_tail:	
	addi $t2, $t2, 1
	b line_find_match_copy_loop
line_find_match_copy_loop_exit:
	move $s2, $t3

	li $t1, 1 # while (cur_count > 1)
	ble $s2, $t1 line_find_match_exit
	lw $t1, line_len
	bge $s3, $t1, exit

	addi $s3, $s3, 1 # cur_index++
	xor $s0, $s0, $s1 # swap(buf, outbuf)
	xor $s1, $s1, $s0
	xor $s0, $s0, $s1
	b line_find_match_loop
line_find_match_exit:
	# only 1 match left, phew

	lw $s0, ($sp)
	lw $s1, 4($sp)
	lw $s2, 8($sp)
	lw $s3, 12($sp)
	lw $ra, 16($sp)
	lw $s4, 20($sp)
	addi $sp, $sp, 24
	jr $ra


# $a0: index
# $a1: buf
# $a2: count(lines)
# $v0: most common, either '0' or '1'
most_common_at_index:
	subi $sp, $sp, 4
	sw $ra ($sp)
	
	move $a3, $a2
	lw $a2, line_len
	jal column_sum_at_index
	
	sub $t0, $a2, $v0 # t0 = count(lines) - ones
	bgt $t0, $a2, most_neg
	li $v0, 1
	b most_ret
most_neg:
	li $v0, 0
most_ret:
	lw $ra ($sp)
	addi $sp, $sp, 4
	jr $ra

# $a0: index
# $a1: buf
# $a2: count(lines)
# $v0: most common, either '0' or '1'
least_common_at_index:
	subi $sp, $sp, 4
	sw $ra ($sp)
	
	move $a3, $a2
	lw $a2, line_len
	jal column_sum_at_index
	
	sub $t0, $a2, $v0 # t0 = count(lines) - ones
	ble $t0, $a2, least_pos
	li $v0, 1
	b most_ret
least_pos:
	li $v0, 0
least_ret:
	lw $ra ($sp)
	addi $sp, $sp, 4
	jr $ra

			
# $a0: src
# $a1: dest
# $a2: cnt
linecpy:
	li $t0, 0
	lw $t1, line_len
	addi $t1, $t1, 1 # \n
	mul $t1, $t1, $a2
linecpy_loop:
	add $t3, $a0, $t0
	add $t4, $a1, $t0

	lb $t2 ($t3)
	sb $t2 ($t4)
	
	addi $t0, $t0, 1
	blt $t0, $t1, linecpy_loop
	jr $ra

print_solutions:
	la $a0, day
	li $v0, 4
	syscall
	la $a0, part1_prompt
	syscall
	lw $a0, part1_solution
	li $v0, 1
	syscall
	la $a0, newline,
	li $v0, 4
	syscall
	la $a0, part2_prompt
	syscall
	lw $a0, part2_solution
	li $v0, 1
	syscall
	la $a0, newline
	li $v0, 4
	syscall
	jr $ra

# parameters:
#	$a0: bitcnt
# returns:
# 	$v0: mask
make_mask:
	li $v0, 0
	
	bgtz $a0, make_mask_loop_head
	jr $ra
make_mask_loop_head:
	subi $sp, $sp, 8
	sw $s0, 0($sp)
	sw $s1, 4($sp)
	li $s0, 0
	li $s1, 1
make_mask_loop:
	subi $a0, $a0, 1	
	sllv $s0, $s1, $a0
	or $v0, $v0, $s0

	bgtz $a0, make_mask_loop
	
	lw $s0, 8($sp)
	lw $s1, 4($sp)
	addi $sp, $sp, 8
	
	jr $ra


# parameters:
#	$a0: index
#	$a1: buf
#	$a2: len(line)
#	$a3: count(lines)
column_sum_at_index:
	li $t0, 0	# $t0 = line_idx
	addi $t1, $a2, 1 # $t1 = step
	li $t6, 0 # $t6 = sum
column_sum_at_loop:
	bge $t0, $a3, column_sum_at_exit
	
	mul $t2, $t0, $t1 # ptr_offset = line_idx * len(line)
	add $t3, $a1, $t2 # ptr = buf + ptr_offset
	add $t3, $t3, $a0 # ptr = ptr + index
	lb $t4, 0($t3)    # c = *ptr
	
	subi $t5, $t4, '0'
	add $t6, $t6, $t5
	
	addi $t0, $t0, 1
	b column_sum_at_loop
column_sum_at_exit:
	move $v0, $t6
	jr $ra


# parameters:
#	$a0: buf
# returns:
# 	$v0: len
get_binary_digits_len:
	li $t0, 16 # max
	li $t1, 1
get_binary_digits_len_loop:
	
	add $t2, $a0, $t1
	lb $t2, 0($t2)
	beq $t2, '\n', get_binary_digits_len_exit
	
	addi $t1, $t1, 1
	ble $t1, $t0, get_binary_digits_len_loop
get_binary_digits_len_exit:
	move $v0, $t1
	jr $ra

# parameters:
# 	$a0: fname
# 	$a1: buf
# 	$a2: len
# returns:
#	$v0: buf
#	$v1: bytes read
read_input_file:
	move $t1, $a0
	move $t2, $a1
	move $t3, $a2
	li $v0, 13
	li $a1, 0
	syscall
	bltz $v0, read_input_file_exit
	move $t0, $v0
	li $v0, 14
	move $a0, $t0
	move $a1, $t2
	move $a2, $t3
	syscall
	move $t4, $v0
	li $v0, 16
	move $a0, $t0
	syscall
	move $v0, $a1
	move $v1, $t4
read_input_file_exit:
	jr $ra

	.data
fname:	.asciiz "./test-input.txt"
input_buf: .space 13000
input_buf_len: .word 13000
scratchpad0: .space 13000
scratchpad1: .space 13000
part1_solution: .word 0
part2_solution: .word 0
day: .asciiz "===== Day 03 =====\n"
part1_prompt: .asciiz "  Part1: "
part2_prompt: .asciiz "  Part2: "
newline: .asciiz "\n"
part2_o_left: .word 0
part2_co2_left: .word 0
line_len: .word 0
line_count: .word 0
part1_gamma: .word 0 
part1_epsilon: .word 0
part2_o_rate: .word 0
part2_co2_rate: .word 0