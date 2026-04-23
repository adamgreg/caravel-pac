# caravel-pac

Peripheral Access Crate (PAC) for the [Caravel](https://github.com/chipfoundry/caravel) & [Caravan](https://github.com/chipfoundry/caravan) ChipIgnite management SoCs from [ChipFoundry](https://chipfoundry.io/).

Safe access to the memory-mapped registers is provided by [volatile-register](https://docs.rs/volatile-register/). Reads are safe, while writes are unsafe.

The types for many registers are implemented using [https://docs.rs/bitfield-struct/](bitfield-struct) to give easy access to separate bitfields within the register.

Each peripheral is accessed via a zero-cost `new()` method that returns a static reference to the memory-mapped registers at the correct base address:

```rust,no_run
use caravel_pac::{CtrlRegisters, FlashCoreRegisters, LaRegisters};

let ctrl  = CtrlRegisters::new();
let flash = FlashCoreRegisters::new();
let la    = LaRegisters::new();
```

## Example Usage

```toml
[dependencies]
caravel-pac = "1.0.0"
```

```rust,no_run
use caravel_pac::{GpioRegisters, SpiMasterRegisters};

// Enable output on the GPIO pin
let gpio = GpioRegisters::new();
unsafe {
    gpio.oe.write(1);
    gpio.output.write(1);
}

// Read GPIO input
let val = gpio.input.read();

// Set the SPI start bit using a read-modify-write of the register
let spi = SpiMasterRegisters::new();
unsafe {
    spi.control.modify(|x| x.with_start(true));
}
```
