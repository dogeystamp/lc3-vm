//////////////////////////////
////// instruction execution
//////////////////////////////

use crate::vm::VM;

#[derive(Debug)]
enum OpCode {
    // branch
    BR = 0,
    // add
    ADD,
    // load
    LD,
    // store
    ST,
    // jump register
    JSR,
    // bitwise and
    AND,
    // load register
    LDR,
    // store register
    STR,
    // return from interrupt (unused)
    RTI,
    // bitwise not
    NOT,
    // load indirect
    LDI,
    // store indirect
    STI,
    // jump
    JMP,
    // reserved (unused)
    RES,
    // load effective address
    LEA,
    // trap
    TRAP,
    // no-operation (not a real opcode)
    NOOP,
}

fn get_opcode(instruction: u16) -> OpCode {
    // the opcode is stored in the left 4 bits
    match instruction >> 12 {
        0 => OpCode::BR,
        1 => OpCode::ADD,
        2 => OpCode::LD,
        3 => OpCode::ST,
        4 => OpCode::JSR,
        5 => OpCode::AND,
        6 => OpCode::LDR,
        7 => OpCode::STR,
        8 => OpCode::RTI,
        9 => OpCode::NOT,
        10 => OpCode::LDI,
        11 => OpCode::STI,
        12 => OpCode::JMP,
        13 => OpCode::RES,
        14 => OpCode::LEA,
        15 => OpCode::TRAP,
        _ => OpCode::NOOP,
    }
}

pub fn execute_instruction(vm: &mut VM, instr: u16) {
    let opcode = get_opcode(instr);

    match opcode {
        OpCode::BR => todo!("BR"),
        OpCode::ADD => todo!("ADD"),
        OpCode::LD => op_ld(vm, instr),
        OpCode::ST => todo!("ST"),
        OpCode::JSR => op_jsr(vm, instr),
        OpCode::AND => todo!("AND"),
        OpCode::LDR => op_ldr(vm, instr),
        OpCode::STR => todo!("STR"),
        OpCode::RTI => todo!("RTI"),
        OpCode::NOT => todo!("NOT"),
        OpCode::LDI => op_ldi(vm, instr),
        OpCode::STI => todo!("STI"),
        OpCode::JMP => todo!("JMP"),
        OpCode::RES => todo!("RES"),
        OpCode::LEA => op_lea(vm, instr),
        OpCode::TRAP => op_trap(vm, instr),
        OpCode::NOOP => no_op(vm, instr),
    }
}

/// Sign extend a value, given the amount of bits it currently has
fn sign_extend(x: u16, bits: usize) -> u16 {
    if (x >> (bits - 1) & 1) == 1 {
        x | (0xffff << bits)
    } else {
        x
    }
}

fn no_op(vm: &mut VM, instr: u16) {}

fn op_lea(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let offset = sign_extend(instr & 0x1ff, 9);

    vm.registers.set_reg_with_cond(dr, vm.registers.pc + offset);
}

fn op_ld(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let offset = sign_extend(instr & 0x1ff, 9);

    vm.registers.set_reg_with_cond(dr, vm.mem.get_mem(vm.registers.pc + offset));
}

fn op_ldi(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let offset = sign_extend(instr & 0x1ff, 9);
    let indirect = vm.mem.get_mem(vm.registers.pc + offset);

    vm.registers.set_reg_with_cond(dr, vm.mem.get_mem(indirect));
}

fn op_ldr(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let base_r = (instr >> 6) & 0b111;
    let offset = sign_extend(instr & 0x3f, 6);

    let addr = vm.registers.get_reg(base_r) + offset;
    vm.registers.set_reg_with_cond(dr, vm.mem.get_mem(addr));
}

fn op_jsr(vm: &mut VM, instr: u16) {
    // this function also includes JSRR
    vm.registers.r7 = vm.registers.pc;

    if (instr >> 11) & 1 == 0 {
        let base_r = (instr >> 6) & 0b111;
        vm.registers.pc = vm.registers.get_reg(base_r);
    } else {
        let offset = sign_extend(instr & 0x7ff, 11);
        vm.registers.pc += offset;
    }
}

fn op_trap(vm: &mut VM, instr: u16) {
    // conform to spec
    // (we don't actually need it in this implementation)
    vm.registers.r7 = vm.registers.pc;

    let trap_vector = instr & 0xff;
    match trap_vector {
        0x20 => todo!("GETC"),
        0x21 => todo!("OUT"),
        0x22 => trap_puts(vm),
        0x23 => todo!("IN"),
        0x24 => todo!("PUTSP"),
        0x25 => vm.running = false,
        _ => unimplemented!(),
    }
}

fn trap_puts(vm: &mut VM) {
    let mut idx = vm.registers.r0;
    loop {
        let c = vm.mem.get_mem(idx) as u8 as char;
        if c == '\0' {
            break;
        }

        print!("{}", c);
        idx += 1;
    }
}
