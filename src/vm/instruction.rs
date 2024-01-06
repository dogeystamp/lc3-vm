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

pub fn execute_instruction(vm: &mut VM) {
    let instruction: u16 = vm.mem.get_mem(vm.registers.pc);
    let opcode = get_opcode(instruction);

    match opcode {
        OpCode::BR => no_op(vm),
        OpCode::ADD => no_op(vm),
        OpCode::LD => no_op(vm),
        OpCode::ST => no_op(vm),
        OpCode::JSR => no_op(vm),
        OpCode::AND => no_op(vm),
        OpCode::LDR => no_op(vm),
        OpCode::STR => no_op(vm),
        OpCode::RTI => no_op(vm),
        OpCode::NOT => no_op(vm),
        OpCode::LDI => no_op(vm),
        OpCode::STI => no_op(vm),
        OpCode::JMP => no_op(vm),
        OpCode::RES => no_op(vm),
        OpCode::LEA => no_op(vm),
        OpCode::TRAP => no_op(vm),
        OpCode::NOOP => no_op(vm),
    }
}

fn no_op(vm: &mut VM) {}
