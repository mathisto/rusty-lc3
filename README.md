# A Rust implementation of the LC-3 virtual machine
Following along with [Rodrigo Araujo's "Let's buils and LC-3 Virtual Machine"](https://www.rodrigoaraujo.me/posts/lets-build-an-lc-3-virtual-machine/)

### Complete:
- [x] Project structure
- [x] Grok bitshifting
- [x] Grok endianness
- [x] Build application skeleton
- [x] Add main loop

### TODO
- [ ] Implemenation of individual op codes:
  - [ ] BR = 0,     // Branch
  - [ ] ADD = 1,    // Add
  - [ ] LD = 2,     // Load
  - [ ] JSR = 3,    // Jump Register
  - [ ] AND = 4,    // Bitwise And
  - [ ] LDR = 5,    // Load Register
  - [ ] STR = 6,    // Store Register
  - [ ] RTI = 7,    // Unused
  - [ ] NOT = 8,    // Bitwise Not
  - [ ] LDI = 9,    // Load Indirect
  - [ ] STI = 10,   // Store Indirect
  - [ ] JMP = 11,   // Jump
  - [ ] RES = 12,   // Reserved (unused)
  - [ ] LEA = 13,   // Load Effective Address
  - [ ] TRAP = 14,  // Execute Trap
- [ ] Test suite
  - Makefile with example `.asm` files piped in?