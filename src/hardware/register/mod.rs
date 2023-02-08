pub struct Registers {
  pub r0: u16,
  pub r1: u16,
  pub r2: u16,
  pub r3: u16,
  pub r4: u16,
  pub r5: u16,
  pub r6: u16,
  pub r7: u16,
  pub pc: u16,
  pub cond: u16,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      r0: 0,         // General Purpose Registers
      r1: 0,
      r2: 0,
      r3: 0,
      r4: 0,
      r5: 0,
      r6: 0,
      r7: 0,
      pc: PC_START,  // Program Counter
      cond: 0,       // Condition Flag
    }
  }

  pub fn update(&mut self, index: u16, value: u16) {
    match index {
      0 => self.r0 = value,
      1 => self.r1 = value,
      2 => self.r2 = value,
      3 => self.r3 = value,
      4 => self.r4 = value,
      5 => self.r5 = value,
      6 => self.r6 = value,
      7 => self.r7 = value,
      8 => self.pc = value,
      9 => self.cond = value,
      _ => panic!("Index is out of bounds. "),
    }
  }

  pub fn get(&self, index: u16) -> {
    match index {
      0 => self.r0,
      1 => self.r1,
      2 => self.r2,
      3 => self.r3,
      4 => self.r4,
      5 => self.r5,
      6 => self.r6,
      7 => self.r7,
      8 => self.pc,
      9 => self.cond,
      _ => panic!("Index is out of bounds. "),
    }
  }
}

enum ConditionFlag {
  POS = 1 << 0, // Positive - 001 << 0 => 001
  ZRO = 1 << 1, // Zero     - 001 << 1 => 010
  NEG = 1 << 2, // Negative - 001 << 2 => 100
}

pub fn update_r_cond_register(&mut self, r: u16) {
  if self.get(r) == 0 { // 9 is the conditional registers index
    self.update(9, ConditionFlag::ZRO as u16);
  } else if (self.get(r) >> 15) != 0 {
    // sign bit (left most bit) shifted all the way to the right is 1
    self.update(9, ConditionFlag::NEG as u16);
  } else {
    self.update(9, ConditionFlag::POS as u16);
  }
}
