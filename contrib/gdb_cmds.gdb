# misc helpers for gdb debugging

# first run lc3, then
#
# 	echo 0 | doas tee /proc/sys/kernel/yama/ptrace_scope
#
# to disable security measures that prevent debugging,
# then
#
# 	rust-gdb -p $(pgrep lc3)
#
# then in the gdb shell
#
# 	source contrib/gdb_cmds.gdb
#
# you can then use the commands defined here


define vmb
	# set a breakpoint at VM addr $0
	break lc3::vm::instruction::execute_instruction if vm.registers.pc == $arg0 + 1
	set $vmb_break = $bpnum
end

define vmj
	# jump execution to VM addr $0
	vmb $arg0
	c
	d $vmb_break
	p vm.registers.pc-1
end

define vms
	# step in vm with breakpoint $0
	# before this break at the second line in lc3::vm::instruction::execute_instruction
	# pass the number of this breakpoint in as $0
	enable $arg0
	c
	p opcode
	p vm.registers.pc-1
	disable $arg0
end
