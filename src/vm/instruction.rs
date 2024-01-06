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
        OpCode::LD => todo!("LD"),
        OpCode::ST => todo!("ST"),
        OpCode::JSR => todo!("JSR"),
        OpCode::AND => todo!("AND"),
        OpCode::LDR => todo!("LDR"),
        OpCode::STR => todo!("STR"),
        OpCode::RTI => todo!("RTI"),
        OpCode::NOT => todo!("NOT"),
        OpCode::LDI => todo!("LDI"),
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
    let dr = (instr >> 8) & 0b111;
    let offset = sign_extend(instr & 0xff, 9);
    vm.registers.set_reg(dr, vm.registers.pc + offset);
}

fn op_trap(vm: &mut VM, instr: u16) {
    // conform to spec
    // (we don't actually need it in this implementation)
    vm.registers.r7 = vm.registers.pc;

    let trap_vector = instr & 0xff;
    match trap_vector {
        0x20 => todo!("GETC"),
        0x21 => todo!("OUT"),
        0x22 => todo!("PUTS"),
        0x23 => todo!("IN"),
        0x24 => todo!("PUTSP"),
        0x25 => vm.running = false,
        _ => unimplemented!(),
    }
}
