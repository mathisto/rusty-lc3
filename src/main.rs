use vm::VM;

pub const MEMORY_SIZE: usize = std::u16::MAX as usize

pub fn execute_program(vm: &mut VM) {
  while cm.registers.pc < MEMORY_SIZE as u16 {
    // Read current instruction at memory address stored in Program Counter (PC)
    let instruction = vm.read_memory(vm.registers.pc);
    vm.registers.pc += 1 // Increment PC to next instruction
    instruction::execute_instruction(instruction, vm) // Get opcode and execute
  }
}

pub fn read_memory(&mut self, address: u16) -> u16 {
  self.memory[address as usize]
}

pub fn execute_instruction(instr: u16, vm: &mut VM) {
  // Extract opcode from left side of 16 bit instr
  let op_code = get_op_code(&instr); 

  // Dispatch on the opcode and execute relevant instr
  match op_code {
    Some(OpCode::ADD)  => add(instr, vm),
    Some(OpCode::AND)  => and(instr, vm),
    Some(OpCode::NOT)  => not(instr, vm),
    Some(OpCode::BR)   => br(instr, vm),
    Some(OpCode::JMP)  => jmp(instr, vm),
    Some(OpCode::JSR)  => jsr(instr, vm),
    Some(OpCode::LD)   => ld(instr, vm),
    Some(OpCode::LDI)  => ldi(instr, vm),
    Some(OpCode::LDR)  => ldr(instr, vm),
    Some(OpCode::LEA)  => lea(instr, vm),
    Some(OpCode::ST)   => st(instr, vm),
    Some(OpCode::STI)  => sti(instr, vm),
    Some(OpCode::STR)  => str(instr, vm),
    Some(OpCode::TRAP) => trap(instr, vm),
    _ => {}
  }
}

let base_address = f.read_u16::<BigEndian>().expect("error");

// Load the assembly code into LC-3's memory
let mut address = base_address as usize;
loop { 
  match f.read_u16::<BigEndian>() { // Snag 2 bytes at a shot
    Ok(instruction) => {
      vm.write_memory(address, instruction); // Load inst
      address += 1; // Increment instruction and loop
    }
    Err(e) => {
      if e.kind() == std::io::ErrorKind::UnexpectedEof {
        println!("OK")
      } else {
        println!("Failed: {}", e);
      }
      break;
    }
  }
}
