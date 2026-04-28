//! ChipIgnite Caravel Management SoC Register Definitions

#![no_std]

use volatile_register::{RO, RW};

pub use caravel_pac_macros::*;

/// Base address for user area
pub const MPRJ_BASE: usize = 0x3000_0000;

// Enable doctest of examples in README.md
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

// =============================================================================
// Register Block Structures
// =============================================================================

/// Control registers
#[register_block(0xf000_0000)]
pub struct CtrlRegisters {
    pub reset: RW<CtrlResetBits>, // 0x0000
    pub scratch: RW<u32>,         // 0x0004
    pub bus_errors: RO<u32>,      // 0x0008
}

/// DEBUG_MODE_OUT register
#[register_block(0xf000_0800)]
pub struct DebugModeOutRegister {
    pub value: RW<u32>, // 0x0800
}

/// DEBUG_OEB_OUT register
#[register_block(0xf000_1000)]
pub struct DebugOebOutRegister {
    pub value: RW<u32>, // 0x1000
}

/// FLASH_CORE registers
#[register_block(0xf000_1800)]
pub struct FlashCoreRegisters {
    pub mmap_dummy_bits: RW<u32>,                  // 0x1800
    pub master_cs: RW<u32>,                        // 0x1804
    pub master_phy_config: RW<FlashPhyConfigBits>, // 0x1808
    pub master_rxtx: RW<u32>,                      // 0x180C
    pub master_status: RO<FlashStatusBits>,        // 0x1810
}

/// FLASH_PHY_CLK_DIVISOR register
#[register_block(0xf000_2000)]
pub struct FlashPhyClockDivisorRegister {
    pub value: RW<u32>, // 0x2000
}

/// GPIO registers
#[register_block(0xf000_2800)]
pub struct GpioRegisters {
    /// DM[2:1] bits of sky130_fd_io__gpiov2 cell
    /// - `0`: output driver disabled
    /// - `1`: output driver enabled (if oe is also set)
    pub mode1: RW<u32>, // 0x2800
    /// DM[0] bit of sky130_fd_io__gpiov2 cell
    /// - `0`: input driver disabled (if mode1 is 0), strong output (if mode1 is 1)
    /// - `1`: input driver enabled (if mode1 is 0), 5k resistive output (if mode1 is 1)
    pub mode0: RW<u32>, // 0x2804
    /// Input enable:
    /// - `0`: input buffer disabled
    /// - `1`: input buffer enabled
    pub ien: RW<u32>, // 0x2808
    /// Output enable:
    /// - `0`: output driver disabled
    /// - `1`: output driver enabled (if mode1 is also set)
    pub oe: RW<u32>, // 0x280C
    /// Input state
    pub input: RO<u32>, // 0x2810
    /// Output setting
    pub output: RW<u32>, // 0x2814
}

/// Logic Analyzer registers (128 bits each, split into 4x32)
#[register_block(0xf000_3000)]
pub struct LaRegisters {
    pub ien: [RW<u32>; 4],    // 0x3000-0x300C
    pub oe: [RW<u32>; 4],     // 0x3010-0x301C
    pub input: [RO<u32>; 4],  // 0x3020-0x302C
    pub output: [RW<u32>; 4], // 0x3030-0x303C
}

/// MPRJ_WB_IENA_OUT register
#[register_block(0xf000_3800)]
pub struct MprjWbIenaOutRegister {
    pub value: RW<u32>, // 0x3800
}

/// SPI_ENABLED_OUT register
#[register_block(0xf000_4000)]
pub struct SpiEnabledOutRegister {
    pub value: RW<u32>, // 0x4000
}

/// SPI Master registers
#[register_block(0xf000_4800)]
pub struct SpiMasterRegisters {
    pub control: RW<SpiControlBits>,   // 0x4800
    pub status: RO<SpiStatusBits>,     // 0x4804
    pub mosi: RW<u32>,                 // 0x4808
    pub miso: RO<u32>,                 // 0x480C
    pub cs: RW<SpiCsBits>,             // 0x4810
    pub loopback: RW<SpiLoopbackBits>, // 0x4814
    pub clk_divider: RW<u32>,          // 0x4818
}

/// Timer0 registers
#[register_block(0xf000_5000)]
pub struct Timer0Registers {
    pub load: RW<u32>,                  // 0x5000
    pub reload: RW<u32>,                // 0x5004
    pub en: RW<u32>,                    // 0x5008
    pub update_value: RW<u32>,          // 0x500C
    pub value: RO<u32>,                 // 0x5010
    pub ev_status: RO<TimerEventBits>,  // 0x5014
    pub ev_pending: RW<TimerEventBits>, // 0x5018
    pub ev_enable: RW<TimerEventBits>,  // 0x501C
}

/// UART registers
#[register_block(0xf000_5800)]
pub struct UartRegisters {
    pub rxtx: RW<u32>,                 // 0x5800
    pub txfull: RO<u32>,               // 0x5804
    pub rxempty: RO<u32>,              // 0x5808
    pub ev_status: RO<UartEventBits>,  // 0x580C
    pub ev_pending: RW<UartEventBits>, // 0x5810
    pub ev_enable: RW<UartEventBits>,  // 0x5814
    pub txempty: RO<u32>,              // 0x5818
    pub rxfull: RO<u32>,               // 0x581C
}

/// UART_ENABLED_OUT register
#[register_block(0xf000_6000)]
pub struct UartEnabledOutRegister {
    pub value: RW<u32>, // 0x6000
}

/// USER_IRQ_0 registers
#[register_block(0xf000_6800)]
pub struct UserIrq0Registers {
    pub input: RO<u32>,                   // 0x6800
    pub mode: RW<u32>,                    // 0x6804
    pub edge: RW<u32>,                    // 0x6808
    pub ev_status: RO<UserIrqEventBits>,  // 0x680C
    pub ev_pending: RW<UserIrqEventBits>, // 0x6810
    pub ev_enable: RW<UserIrqEventBits>,  // 0x6814
}

/// USER_IRQ_1 registers
#[register_block(0xf000_7000)]
pub struct UserIrq1Registers {
    pub input: RO<u32>,                   // 0x7000
    pub mode: RW<u32>,                    // 0x7004
    pub edge: RW<u32>,                    // 0x7008
    pub ev_status: RO<UserIrqEventBits>,  // 0x700C
    pub ev_pending: RW<UserIrqEventBits>, // 0x7010
    pub ev_enable: RW<UserIrqEventBits>,  // 0x7014
}

/// USER_IRQ_2 registers
#[register_block(0xf000_7800)]
pub struct UserIrq2Registers {
    pub input: RO<u32>,                   // 0x7800
    pub mode: RW<u32>,                    // 0x7804
    pub edge: RW<u32>,                    // 0x7808
    pub ev_status: RO<UserIrqEventBits>,  // 0x780C
    pub ev_pending: RW<UserIrqEventBits>, // 0x7810
    pub ev_enable: RW<UserIrqEventBits>,  // 0x7814
}

/// USER_IRQ_3 registers
#[register_block(0xf000_8000)]
pub struct UserIrq3Registers {
    pub input: RO<u32>,                   // 0x8000
    pub mode: RW<u32>,                    // 0x8004
    pub edge: RW<u32>,                    // 0x8008
    pub ev_status: RO<UserIrqEventBits>,  // 0x800C
    pub ev_pending: RW<UserIrqEventBits>, // 0x8010
    pub ev_enable: RW<UserIrqEventBits>,  // 0x8014
}

/// USER_IRQ_4 registers
#[register_block(0xf000_8800)]
pub struct UserIrq4Registers {
    pub input: RO<u32>,                   // 0x7000
    pub mode: RW<u32>,                    // 0x7004
    pub edge: RW<u32>,                    // 0x7008
    pub ev_status: RO<UserIrqEventBits>,  // 0x700C
    pub ev_pending: RW<UserIrqEventBits>, // 0x7010
    pub ev_enable: RW<UserIrqEventBits>,  // 0x7014
}

/// USER_IRQ_5 registers
#[register_block(0xf000_9000)]
pub struct UserIrq5Registers {
    pub input: RO<u32>,                   // 0x7000
    pub mode: RW<u32>,                    // 0x7004
    pub edge: RW<u32>,                    // 0x7008
    pub ev_status: RO<UserIrqEventBits>,  // 0x700C
    pub ev_pending: RW<UserIrqEventBits>, // 0x7010
    pub ev_enable: RW<UserIrqEventBits>,  // 0x7014
}

/// User project control registers
#[register_block(0x2600_0000)]
pub struct UserProjectRegisters {
    pub xfer: RW<UserIoXferBits>, // 0x2600_0000
    pub pwr: RW<u32>,             // 0x2600_0004
    _pad0: u32,
    pub datal: RW<u32>,           // 0x2600_000C
    pub datah: RW<u32>,           // 0x2600_0010
    _pad1: [u32; 4],              // 0x2614-0x2620
    pub io: [RW<UserIOBits>; 38], // 0x2600_0024 to 0x2600_00B8
}

/// Housekeeping SPI registers
#[register_block(0x2610_0000)]
pub struct HousekeepingRegisters {
    pub status: RO<u32>,      // 0x2610_0000
    pub chip_id: RO<u32>,     // 0x2610_0004
    pub user_id: RO<u32>,     // 0x2610_0008
    pub pll_ena: RW<u32>,     // 0x2610_000C // FIXME define bitfield
    pub pll_bypass: RW<u32>,  // 0x2610_0010 // FIXME define bitfield
    pub irq: RW<u32>,         // 0x2610_0014 // FIXME define bitfield
    pub reset: RW<u32>,       // 0x2610_0018 // FIXME define bitfield
    pub pll_trim: RW<u32>,    // 0x2610_001C
    pub pll_source: RW<u32>,  // 0x2610_0020 // FIXME define bitfield
    pub pll_divider: RW<u32>, // 0x2610_0024
}

/// System area registers
#[register_block(0x2620_0000)]
pub struct SystemRegisters {
    pub power_good: RO<u32>,   // 0x2620_0000 // FIXME define bitfield
    pub clk_out_dest: RW<u32>, // 0x2620_0004 // FIXME define bitfield
    _pad0: u32,
    pub irq_source: RW<u32>,    // 0x2620_000C // FIXME define bitfield
    pub hkspi_disable: RW<u32>, // 0x2620_0010 // FIXME define bitfield
}

/// Register bitfield definitions using bitfield-struct
pub mod bitfields {
    use bitfield_struct::bitfield;

    /// Control Reset register bitfields
    #[bitfield(u32)]
    pub struct CtrlResetBits {
        /// Write 1 to this register to reset the full SoC (Pulse Reset)
        #[bits(1)]
        pub soc_rst: bool,
        /// Write 1 to this register to reset the CPU(s) of the SoC (Hold Reset)
        #[bits(1)]
        pub cpu_rst: bool,
        #[bits(30)]
        _reserved: u32,
    }

    /// UART Event Status/Pending/Enable register bitfields
    #[bitfield(u32)]
    pub struct UartEventBits {
        #[bits(1)]
        pub tx: bool,
        #[bits(1)]
        pub rx: bool,
        #[bits(30)]
        _reserved: u32,
    }

    /// User IRQ Event Status/Pending/Enable register bitfields
    #[bitfield(u32)]
    pub struct UserIrqEventBits {
        #[bits(1)]
        pub i0: bool,
        #[bits(31)]
        _reserved: u32,
    }

    /// Timer Enable register bitfields
    #[bitfield(u32)]
    pub struct TimerEnableBits {
        #[bits(1)]
        pub enable: bool,
        #[bits(1)]
        pub oneshot: bool,
        #[bits(1)]
        pub upcount: bool,
        #[bits(1)]
        pub chain: bool,
        #[bits(1)]
        pub irq_enable: bool,
        #[bits(27)]
        _reserved: u32,
    }

    /// Timer Event Status/Pending/Enable register bitfields
    #[bitfield(u32)]
    pub struct TimerEventBits {
        #[bits(1)]
        pub zero: bool,
        #[bits(31)]
        _reserved: u32,
    }

    /// SPI Master Control register bitfields
    #[bitfield(u32)]
    pub struct SpiControlBits {
        /// SPI Xfer Start (Write 1 to start Xfer).
        #[bits(1)]
        pub start: bool,
        #[bits(7)]
        _reserved1: u8,
        /// SPI Xfer Length (in bits).
        #[bits(8)]
        pub length: u8,
        #[bits(16)]
        _reserved2: u16,
    }

    /// SPI Master Status register bitfields
    #[bitfield(u32)]
    pub struct SpiStatusBits {
        /// SPI Xfer Done (when read as 1).
        #[bits(1)]
        pub done: bool,
        #[bits(31)]
        _reserved: u32,
    }

    /// SPI Master CS register bitfields
    #[bitfield(u32)]
    pub struct SpiCsBits {
        /// Chip 0 selected for SPI Xfer.
        #[bits(1)]
        pub sel: bool,
        #[bits(15)]
        _reserved1: u16,
        /// SPI CS mode:
        /// - `0`: Normal operation (CS handled by Core).
        /// - `1`: Manual operation (CS handled by User, direct recopy of sel), useful for Bulk transfers.
        #[bits(1)]
        pub mode: bool,
        #[bits(15)]
        _reserved2: u16,
    }

    /// SPI Master Loopback register bitfields
    #[bitfield(u32)]
    pub struct SpiLoopbackBits {
        /// SPI Loopback mode:
        /// - `0`: Normal operation.
        /// - `1`: Loopback operation (MOSI to MISO).
        #[bits(1)]
        pub mode: bool,
        #[bits(31)]
        _reserved: u32,
    }

    /// Flash PHY Config register bitfields
    #[bitfield(u32)]
    pub struct FlashPhyConfigBits {
        /// SPI Xfer length (in bits).
        #[bits(8)]
        #[allow(clippy::len_without_is_empty)]
        pub len: u8,
        /// SPI Xfer width (1/2/4/8).
        #[bits(4)]
        pub width: u8,
        #[bits(4)]
        _reserved1: u8,
        /// SPI DQ output enable mask (set bits to 1 to enable output drivers on DQ lines).
        #[bits(8)]
        pub mask: u8,
        #[bits(8)]
        _reserved2: u8,
    }

    /// Flash Master Status register bitfields
    #[bitfield(u32)]
    pub struct FlashStatusBits {
        /// TX FIFO is not full.
        #[bits(1)]
        pub tx_ready: bool,
        /// RX FIFO is not empty.
        #[bits(1)]
        pub rx_ready: bool,
        #[bits(30)]
        _reserved: u32,
    }

    /// Power Good register bitfields
    #[bitfield(u32)]
    pub struct PowerGoodBits {
        #[bits(1)]
        pub user1_vccd: bool,
        #[bits(1)]
        pub user2_vccd: bool,
        #[bits(1)]
        pub user1_vdda: bool,
        #[bits(1)]
        pub user2_vdda: bool,
        #[bits(28)]
        _reserved: u32,
    }

    /// Clock Output Destination register bitfields
    #[bitfield(u32)]
    pub struct ClkOutDestBits {
        #[bits(1)]
        pub clock1_monitor: bool,
        #[bits(1)]
        pub clock2_monitor: bool,
        #[bits(30)]
        _reserved: u32,
    }

    /// IRQ Source register bitfields
    #[bitfield(u32)]
    pub struct IrqSourceBits {
        #[bits(1)]
        pub irq7: bool,
        #[bits(1)]
        pub irq8: bool,
        #[bits(30)]
        _reserved: u32,
    }

    /// User I/O configuration transfer bitfields
    #[bitfield(u32)]
    pub struct UserIoXferBits {
        /// Serial xfer/busy
        /// - `0`: idle
        /// - `1`: start transfer (write) / transfer in progress (read)
        #[bits(1)]
        pub xfer_busy: bool,
        /// - `0`: bitbang mode disabled
        /// - `1`: serial transfer bitbang mode enabled
        #[bits(1)]
        pub bitbang_enable: bool,
        /// - `0`: bitbang mode reset
        /// - `1`: bitbang mode normal operation
        #[bits(1)]
        pub bitbang_resetn: bool,
        /// - `0`: bitbang mode normal operation
        /// - `1`: latch configuration values
        #[bits(1)]
        pub bitbang_load: bool,
        /// 0->1 transition: Advance data in serial shift register by 1 bit in bitbang mode
        #[bits(1)]
        pub bitbang_clock: bool,
        /// Data to apply to serial data right side shift register (GPIO 0 to 18) on next bitbang clock
        #[bits(1)]
        pub bitbang_data_right: bool,
        /// Data to apply to serial data left side shift register (GPIO 19 to 37) on next bitbang clock
        #[bits(1)]
        pub bitbang_data_left: bool,
        #[bits(25)]
        _reserved: u32,
    }

    /// User I/O Pad Control bitfields
    #[bitfield(u32)]
    pub struct UserIOBits {
        /// Management enable:
        /// - `0`: user project controls GPIO
        /// - `1`: management SoC controls GPIO
        #[bits(1)]
        pub mgmt_enable: bool,
        /// Output disable:
        /// - `0`: digital output driver enabled (management controlled mode only)
        /// - `1`: digital output driver disabled
        #[bits(1)]
        pub output_disable: bool,
        /// Value of GPIO when in low-power state.
        #[bits(1)]
        pub hold_state_value: bool,
        /// Input disable:
        /// - `0`: digital input driver enabled
        /// - `1`: digital input driver disabled
        #[bits(1)]
        pub input_disable: bool,
        #[bits(1)]
        pub ib_mode_select: bool,
        #[bits(1)]
        pub analog_enable: bool,
        #[bits(1)]
        pub analog_select: bool,
        #[bits(1)]
        pub analog_polarity: bool,
        #[bits(1)]
        pub slow_slew: bool,
        #[bits(1)]
        pub trip_point_select: bool,
        /// Digital mode configuration:
        /// - `000`: disabled - Both input and output digital buffers disabled. Use this when connecting an analog signal to the pad
        /// - `001`: input - Digital input only. Output buffer is disabled.
        /// - `010`: input pullup - Input mode with pull-up. User mode only: Output must be enabled and driven to value 1.
        /// - `011`: input pulldown - Input mode with pull-down. User mode only: Output must be enabled and driven to value 0.
        /// - `110`: output - Digital output. User mode only: Output must be enabled (OEB = 0)
        #[bits(3)]
        pub digital_mode: u8,
        #[bits(19)]
        _reserved: u32,
    }

    /// Associated constants for UserIOBits, defining standard I/O configurations
    impl UserIOBits {
        // Management interface standard modes
        pub const MGMT_STD_INPUT_NOPULL: Self = Self(0x0403);
        pub const MGMT_STD_INPUT_PULLDOWN: Self = Self(0x0c01);
        pub const MGMT_STD_INPUT_PULLUP: Self = Self(0x0801);
        pub const MGMT_STD_OUTPUT: Self = Self(0x1809);
        pub const MGMT_STD_BIDIRECTIONAL: Self = Self(0x1801);
        pub const MGMT_STD_OUT_MONITORED: Self = Self(0x1803);
        pub const MGMT_STD_ANALOG: Self = Self(0x000b);

        // User interface standard modes
        pub const USER_STD_INPUT_NOPULL: Self = Self(0x0402);
        pub const USER_STD_INPUT_PULLDOWN: Self = Self(0x0c00);
        pub const USER_STD_INPUT_PULLUP: Self = Self(0x0800);
        pub const USER_STD_OUTPUT: Self = Self(0x1808);
        pub const USER_STD_BIDIRECTIONAL: Self = Self(0x1800);
        pub const USER_STD_OUT_MONITORED: Self = Self(0x1802);
        pub const USER_STD_ANALOG: Self = Self(0x000a);
    }
} // mod bitfields

pub use bitfields::*;

// =============================================================================
// Interrupts
// =============================================================================

/// RISC-V interrupt numbers used by Caravel
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "interrupts", riscv::pac_enum(unsafe ExternalInterruptNumber))]
pub enum CaravelInterrupt {
    Timer0 = 0,
    Uart = 1,
    User0 = 2,
    User1 = 3,
    User2 = 4,
    User3 = 5,
    User4 = 6,
    User5 = 7,
}

#[cfg(feature = "interrupts")]
impl CaravelInterrupt {
    /// Enable this Caravel external interrupt
    pub fn enable(self) {
        let vmim = vexriscv::register::vmim::read();
        vexriscv::register::vmim::write(vmim | (1 << self as usize));
    }
    /// Disable this Caravel external interrupt
    pub fn disable(self) {
        let vmim = vexriscv::register::vmim::read();
        vexriscv::register::vmim::write(vmim & !(1 << self as usize));
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    /// Helper macro to get the absolute address of a register field
    macro_rules! addr {
        ($struct:ty, $field:ident) => {{
            // Get base address by casting the new() result to usize
            let base = <$struct>::new() as *const $struct as usize;
            base + offset_of!($struct, $field)
        }};
        ($struct:ty, $field:ident[$idx:expr]) => {{
            let base = <$struct>::new() as *const $struct as usize;
            base + offset_of!($struct, $field) + ($idx * core::mem::size_of::<u32>())
        }};
    }

    #[test]
    fn test_register_addresses() {
        assert_eq!(addr!(CtrlRegisters, reset), 0xf000_0000);
        assert_eq!(addr!(CtrlRegisters, scratch), 0xf000_0004);
        assert_eq!(addr!(CtrlRegisters, bus_errors), 0xf000_0008);

        assert_eq!(addr!(FlashCoreRegisters, mmap_dummy_bits), 0xf000_1800);
        assert_eq!(addr!(FlashCoreRegisters, master_cs), 0xf000_1804);
        assert_eq!(addr!(FlashCoreRegisters, master_phy_config), 0xf000_1808);
        assert_eq!(addr!(FlashCoreRegisters, master_rxtx), 0xf000_180C);
        assert_eq!(addr!(FlashCoreRegisters, master_status), 0xf000_1810);

        assert_eq!(addr!(GpioRegisters, mode1), 0xf000_2800);
        assert_eq!(addr!(GpioRegisters, mode0), 0xf000_2804);
        assert_eq!(addr!(GpioRegisters, ien), 0xf000_2808);
        assert_eq!(addr!(GpioRegisters, oe), 0xf000_280C);
        assert_eq!(addr!(GpioRegisters, input), 0xf000_2810);
        assert_eq!(addr!(GpioRegisters, output), 0xf000_2814);

        assert_eq!(addr!(SpiMasterRegisters, control), 0xf000_4800);
        assert_eq!(addr!(SpiMasterRegisters, status), 0xf000_4804);
        assert_eq!(addr!(SpiMasterRegisters, mosi), 0xf000_4808);
        assert_eq!(addr!(SpiMasterRegisters, miso), 0xf000_480C);
        assert_eq!(addr!(SpiMasterRegisters, cs), 0xf000_4810);
        assert_eq!(addr!(SpiMasterRegisters, loopback), 0xf000_4814);
        assert_eq!(addr!(SpiMasterRegisters, clk_divider), 0xf000_4818);

        assert_eq!(addr!(Timer0Registers, load), 0xf000_5000);
        assert_eq!(addr!(Timer0Registers, reload), 0xf000_5004);
        assert_eq!(addr!(Timer0Registers, en), 0xf000_5008);
        assert_eq!(addr!(Timer0Registers, update_value), 0xf000_500C);
        assert_eq!(addr!(Timer0Registers, value), 0xf000_5010);
        assert_eq!(addr!(Timer0Registers, ev_status), 0xf000_5014);
        assert_eq!(addr!(Timer0Registers, ev_pending), 0xf000_5018);
        assert_eq!(addr!(Timer0Registers, ev_enable), 0xf000_501C);

        assert_eq!(addr!(UartRegisters, rxtx), 0xf000_5800);
        assert_eq!(addr!(UartRegisters, txfull), 0xf000_5804);
        assert_eq!(addr!(UartRegisters, rxempty), 0xf000_5808);
        assert_eq!(addr!(UartRegisters, ev_status), 0xf000_580C);
        assert_eq!(addr!(UartRegisters, ev_pending), 0xf000_5810);
        assert_eq!(addr!(UartRegisters, ev_enable), 0xf000_5814);
        assert_eq!(addr!(UartRegisters, txempty), 0xf000_5818);
        assert_eq!(addr!(UartRegisters, rxfull), 0xf000_581C);

        assert_eq!(addr!(UserProjectRegisters, xfer), 0x2600_0000);
        assert_eq!(addr!(UserProjectRegisters, pwr), 0x2600_0004);
        assert_eq!(addr!(UserProjectRegisters, datal), 0x2600_000C);
        assert_eq!(addr!(UserProjectRegisters, datah), 0x2600_0010);
        assert_eq!(addr!(UserProjectRegisters, io[0]), 0x2600_0024);
        assert_eq!(addr!(UserProjectRegisters, io[1]), 0x2600_0028);
        assert_eq!(addr!(UserProjectRegisters, io[10]), 0x2600_004C);
        assert_eq!(addr!(UserProjectRegisters, io[37]), 0x2600_00B8);

        assert_eq!(addr!(HousekeepingRegisters, status), 0x2610_0000);
        assert_eq!(addr!(HousekeepingRegisters, chip_id), 0x2610_0004);
        assert_eq!(addr!(HousekeepingRegisters, user_id), 0x2610_0008);
        assert_eq!(addr!(HousekeepingRegisters, pll_ena), 0x2610_000C);
        assert_eq!(addr!(HousekeepingRegisters, pll_bypass), 0x2610_0010);
        assert_eq!(addr!(HousekeepingRegisters, irq), 0x2610_0014);
        assert_eq!(addr!(HousekeepingRegisters, reset), 0x2610_0018);
        assert_eq!(addr!(HousekeepingRegisters, pll_trim), 0x2610_001C);
        assert_eq!(addr!(HousekeepingRegisters, pll_source), 0x2610_0020);
        assert_eq!(addr!(HousekeepingRegisters, pll_divider), 0x2610_0024);

        assert_eq!(addr!(SystemRegisters, power_good), 0x2620_0000);
        assert_eq!(addr!(SystemRegisters, clk_out_dest), 0x2620_0004);
        assert_eq!(addr!(SystemRegisters, irq_source), 0x2620_000C);
        assert_eq!(addr!(SystemRegisters, hkspi_disable), 0x2620_0010);
    }

    #[test]
    fn test_user_register_block() {
        #[user_register_block]
        pub struct MyRegisters {
            pub a: RO<u32>,     // Address 0x3000_0000, read-only
            pub b: [RW<u8>; 8], // Address 0x3000_0004 to 0x3000_000B, read-write
            pub c: RW<u32>,     // Address 0x3000_000C, read-write
        }

        assert_eq!(addr!(MyRegisters, a), 0x3000_0000);
        assert_eq!(addr!(MyRegisters, b), 0x3000_0004);
        assert_eq!(addr!(MyRegisters, c), 0x3000_000C);

        #[user_register_block(0x100)]
        pub struct MyRegistersTwo {
            pub d: RW<u32>, // Address 0x3000_0100
            _pad: [u32; 4], // Padding - no hardware mapped to these addresses
            pub e: RW<u32>, // Address 0x3000_0114
        }

        assert_eq!(addr!(MyRegistersTwo, d), 0x3000_0100);
        assert_eq!(addr!(MyRegistersTwo, e), 0x3000_0114);
    }
}
