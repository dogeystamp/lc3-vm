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
        OpCode::BR => no_op(vm, instr),
        OpCode::ADD => no_op(vm, instr),
        OpCode::LD => no_op(vm, instr),
        OpCode::ST => no_op(vm, instr),
        OpCode::JSR => no_op(vm, instr),
        OpCode::AND => no_op(vm, instr),
        OpCode::LDR => no_op(vm, instr),
        OpCode::STR => no_op(vm, instr),
        OpCode::RTI => no_op(vm, instr),
        OpCode::NOT => no_op(vm, instr),
        OpCode::LDI => no_op(vm, instr),
        OpCode::STI => no_op(vm, instr),
        OpCode::JMP => no_op(vm, instr),
        OpCode::RES => no_op(vm, instr),
        OpCode::LEA => op_lea(vm, instr),
        OpCode::TRAP => no_op(vm, instr),
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
    let dr = (instr >> 8) & 0b111;
    let offset = sign_extend(instr & 0xff, 9);
    vm.registers.set_reg(dr, vm.registers.pc + offset);
}
