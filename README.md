# little computer 3 (LC-3) virtual machine

LC-3 is an assembly language designed to educate students about the inner workings of computers.
This project is a virtual machine implementation,
which can run programs designed to work on LC-3 hardware.

For more information, read the [blog post](https://www.jmeiners.com/lc3-vm/) that inspired this, written by Justin Meiners and Ryan Pendleton.
Also, the [LC-3 ISA specification](https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf)
may help to understand the code.

## usage

```bash
git clone https://github.com/dogeystamp/lc3-vm
cd lc3-vm
```

Sample programs are provided in the `programs/` directory.
`2048.obj` and `rogue.obj` are respectively from [Meiners](https://github.com/justinmeiners/lc3-rogue) and [Pendleton](https://github.com/rpendleton/lc3-2048).
Note that these are outdated compared to the latest versions on GitHub.
To run them, use the following commands:

```bash
cargo run -- programs/2048.obj
cargo run -- programs/rogue.obj
```

For extra information about using the lc3-vm command line, run

```bash
cargo run -- --help
```

## gallery

2048:
```
+--------------------------+
|                          |
|                          |
|                          |
|         2                |
|                          |
|                          |
|                          |
|                     2    |
|                          |
+--------------------------+
```

Rogue:
```
##################  ############
###################     ########
#######################        #
########################  #  #  
###############################D
################################
################################
@ ##############################
#  #############################
##    ##########################
#####  #########################
######  ########################
#######   ######################
#########    ###################
############  ##  ##############
#############      #############
```

## debugging

A debug option is available in the command line, which will print execution data to stderr.
This can be piped to a separate file, and viewed with `tail -f` in a separate terminal:
```bash
$ cargo run -- --debug programs/2048.obj 2>trace

# (separate terminal)
$ tail -f trace

PC: 0x3314, op: BR, params: 0x7fd
R0: 0x0
R1: 0x1fbc
R2: 0x0
R3: 0x0
R4: 0x0
R5: 0x3017
R6: 0x3ffc
R7: 0x32db
COND: 0x2 (Z)
```

Alternatively, you can attach to a VM with `rust-gdb`.
First, start a VM:
```
cargo run -- programs/2048.obj
```

Then, in a new terminal window:
```
# (separate terminal)
rust-gdb -p (pgrep lc3)
```

Helper utilities are available in `contrib/gdb_cmds.gdb`, along with instructions for using them.
These are provided as-is, with no guarantee that they will work well.
