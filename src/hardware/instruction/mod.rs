pub enum OpCode {
  BR = 0, // Branch
  ADD = 1,    // Add
  LD = 2,     // Load
  JSR = 3,    // Jump Register
  AND = 4,    // Bitwise And
  LDR = 5,    // Load Register
  STR = 6,    // Store Register
  RTI = 7,    // Unused
  NOT = 8,    // Bitwise Not
  LDI = 9,    // Load Indirect
  STI = 10,    // Store Indirect
  JMP = 11,    // Jump
  RES = 12,    // Reserved (unused)
  LEA = 13,    // Load Effective Address
  TRAP = 14,   // Execute Trap
}

pub fn get_op_code(instruction: &u16) -> Option<OpCode> {
  match instruction >> 12 {
    0 => Some(OpCode::BR),
    1 => Some(OpCode::ADD),
    2 => Some(OpCode::LD),
    3 => Some(OpCode::ST),
    4 => Some(OpCode::JSR),
    5 => Some(OpCode::AND),
    6 => Some(OpCode::LDR),
    7 => Some(OpCode::STR),
    8 => Some(OpCode::RTI),
    9 => Some(OpCode::NOT),
    10 => Some(OpCode::LDI),
    11 => Some(OpCode::STI),
    12 => Some(OpCode::JMP),
    13 => Some(OpCode::RES),
    14 => Some(OpCode::LEA),
    15 => Some(OpCode::TRAP),
    _ => None,
  }
}
