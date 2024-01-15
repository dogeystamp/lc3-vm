//////////////////////////////
////// instruction execution
//////////////////////////////

use crate::vm::VM;

////////////////
// Main part
////////////////

pub fn get_instruction(vm: &mut VM) -> u16 {
    return vm.mem.get_mem(vm.registers.pc);
}

#[derive(Debug)]
pub enum OpCode {
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

pub fn get_opcode(instruction: u16) -> OpCode {
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
        OpCode::BR => op_br(vm, instr),
        OpCode::ADD => op_add(vm, instr),
        OpCode::LD => op_ld(vm, instr),
        OpCode::ST => op_st(vm, instr),
        OpCode::JSR => op_jsr(vm, instr),
        OpCode::AND => op_and(vm, instr),
        OpCode::LDR => op_ldr(vm, instr),
        OpCode::STR => op_str(vm, instr),
        OpCode::RTI => unimplemented!("RTI (privilege violation)"),
        OpCode::NOT => op_not(vm, instr),
        OpCode::LDI => op_ldi(vm, instr),
        OpCode::STI => op_sti(vm, instr),
        OpCode::JMP => op_jmp(vm, instr),
        OpCode::RES => unimplemented!("RES (illegal instruction)"),
        OpCode::LEA => op_lea(vm, instr),
        OpCode::TRAP => op_trap(vm, instr),
        OpCode::NOOP => no_op(vm, instr),
    }
}

////////////////
// Helpers
////////////////

/// Sign extend a value, given the amount of bits it currently has
fn sign_extend(x: u16, bits: usize) -> u16 {
    if (x >> (bits - 1) & 1) == 1 {
        x | (0xffff << bits)
    } else {
        x
    }
}

fn no_op(vm: &mut VM, instr: u16) {}

////////////////
// Load ops
////////////////

fn op_lea(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let offset = sign_extend(instr & 0x1ff, 9);

    vm.registers
        .set_reg_with_cond(dr, vm.registers.pc.wrapping_add(offset));
}

fn op_ld(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let offset = sign_extend(instr & 0x1ff, 9);

    vm.registers
        .set_reg_with_cond(dr, vm.mem.get_mem(vm.registers.pc.wrapping_add(offset)));
}

fn op_ldi(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let offset = sign_extend(instr & 0x1ff, 9);
    let indirect = vm.mem.get_mem(vm.registers.pc.wrapping_add(offset));

    vm.registers.set_reg_with_cond(dr, vm.mem.get_mem(indirect));
}

fn op_ldr(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let base_r = (instr >> 6) & 0b111;
    let offset = sign_extend(instr & 0x3f, 6);

    let addr = vm.registers.get_reg(base_r).wrapping_add(offset);
    vm.registers.set_reg_with_cond(dr, vm.mem.get_mem(addr));
}

////////////////
// Jumps/branches
////////////////

fn op_jsr(vm: &mut VM, instr: u16) {
    // this function also includes JSRR
    vm.registers.r7 = vm.registers.pc;

    if (instr >> 11) & 1 == 0 {
        let base_r = (instr >> 6) & 0b111;
        vm.registers.pc = vm.registers.get_reg(base_r);
    } else {
        let offset = sign_extend(instr & 0x7ff, 11);
        vm.registers.pc = vm.registers.pc.wrapping_add(offset);
    }
}

fn op_br(vm: &mut VM, instr: u16) {
    let offset = sign_extend(instr & 0x1ff, 9);
    // technically the COND we have is just a part of the PSR register in the spec
    // therefore isolate the last 3 bits
    let cond = vm.registers.cond & 0x7;

    let need_cond = (instr >> 9) & 0x7;

    // BRnzp is unconditional
    // if we haven't performed any instructions that set conditions,
    // COND might just be 0
    if need_cond & cond != 0 || need_cond == 0b111 {
        vm.registers.pc = vm.registers.pc.wrapping_add(offset);
    }
}

fn op_jmp(vm: &mut VM, instr: u16) {
    // RET is a special case of this where BaseR is R7
    let base_r = (instr >> 6) & 0b111;

    vm.registers.pc = vm.registers.get_reg(base_r);
}

////////////////
// Store ops
////////////////

fn op_st(vm: &mut VM, instr: u16) {
    let sr = (instr >> 9) & 0b111;
    let offset = sign_extend(instr & 0x1ff, 9);
    vm.mem.set_mem(
        vm.registers.pc.wrapping_add(offset),
        vm.registers.get_reg(sr),
    );
}

fn op_sti(vm: &mut VM, instr: u16) {
    let sr = (instr >> 9) & 0b111;
    let offset = sign_extend(instr & 0x1ff, 9);

    let addr = vm.mem.get_mem(vm.registers.pc.wrapping_add(offset));
    vm.mem.set_mem(addr, vm.registers.get_reg(sr));
}

fn op_str(vm: &mut VM, instr: u16) {
    let sr = (instr >> 9) & 0b111;
    let base_r = (instr >> 6) & 0b111;
    let offset = sign_extend(instr & 0x3f, 6);

    // NOTE:
    // this is how rodrigo did it:
    //
    //  let addr = (vm.registers.get_reg(base_r) as u32 + offset as u32) as u16;
    //
    // apparently narrowing casts automatically wrap integers
    // this is more explicit
    let addr = vm.registers.get_reg(base_r).wrapping_add(offset);
    vm.mem.set_mem(addr, vm.registers.get_reg(sr));
}

////////////////
// Arithmetic
////////////////

fn op_add(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let sr1 = (instr >> 6) & 0b111;

    if (instr >> 5) & 1 == 0 {
        let sr2 = instr & 0b111;

        let res = vm
            .registers
            .get_reg(sr1)
            .wrapping_add(vm.registers.get_reg(sr2));
        vm.registers.set_reg_with_cond(dr, res);
    } else {
        let imm = sign_extend(instr & 0x1f, 5);

        let res = vm.registers.get_reg(sr1).wrapping_add(imm);
        vm.registers.set_reg_with_cond(dr, res);
    }
}

fn op_and(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let sr1 = (instr >> 6) & 0b111;

    if (instr >> 5) & 1 == 0 {
        let sr2 = instr & 0b111;

        let res = vm.registers.get_reg(sr1) & vm.registers.get_reg(sr2);
        vm.registers.set_reg_with_cond(dr, res);
    } else {
        let imm = sign_extend(instr & 0x1f, 5);

        let res = vm.registers.get_reg(sr1) & imm;
        vm.registers.set_reg_with_cond(dr, res);
    }
}

fn op_not(vm: &mut VM, instr: u16) {
    let dr = (instr >> 9) & 0b111;
    let sr = (instr >> 6) & 0b111;

    // NOTE
    // rustc is very friendly and tells you off if you use ~ as bitwise not
    let res = !vm.registers.get_reg(sr);
    vm.registers.set_reg_with_cond(sr, res);
}

////////////////
// Trap/trap routines
////////////////

fn op_trap(vm: &mut VM, instr: u16) {
    // conform to spec
    // (we don't actually need it in this implementation)
    vm.registers.r7 = vm.registers.pc;

    let trap_vector = instr & 0xff;
    match trap_vector {
        0x20 => trap_getc(vm),
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

fn trap_getc(vm: &mut VM) {
    while vm.mem.get_mem(0xFE00) & 1 == 0 {}
    vm.registers.r0 = vm.mem.get_mem(0xFE02) & 0xFF;
}
