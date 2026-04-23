# caravel-pac-macros

Proc macro support crate for [`caravel-pac`](https://crates.io/crates/caravel-pac). You should not depend on this crate directly — use `caravel-pac` instead.

## `#[register_block(addr)]`

Attribute macro that turns a struct into a memory-mapped `repr(C)` register block. It adds a `new()` method that returns a static reference at the given base address.

```rust
use caravel_pac_macros::register_block;
use volatile_register::RW;

#[register_block(0x2000_0000)]
pub struct UartRegisters {
    pub data:    RW<u32>,
    pub control: RW<u32>,
    pub status:  RW<u32>,
}

// Expands to (roughly):
// #[repr(C)]
// pub struct UartRegisters { ... }
// impl UartRegisters {
//     pub fn new() -> &'static Self { unsafe { &*(0x2000_0000 as *const Self) } }
// }

let uart = UartRegisters::new();
unsafe { uart.data.write(0x41); }
```
