use proc_macro::TokenStream;

/// Attribute macro to define a memory-mapped register block with a base address
///
/// # Usage
/// ```rust,ignore
/// use volatile_register::{RO, RW};
///
/// #[register_block(0x2000_0000)]
/// pub struct MyRegisters {
///     pub a: RO<u32>, // Address 0x2000_0000, read-only
/// }
/// ```
///
/// This will:
/// - Add `#[repr(C)]` to the struct
/// - Generate a `new()` method that returns a static reference at the base address
/// - If the `mock-registers` feature of `caravel-pac` is enabled,
///   a safe static instance is created instead of mapping to the specified address
#[proc_macro_attribute]
pub fn register_block(#[allow(unused)] args: TokenStream, input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    // Find the struct name by looking for "struct" keyword
    let struct_pos = input_str
        .find("struct")
        .expect("Expected struct definition");
    let after_struct = &input_str[struct_pos + 6..].trim_start();
    let struct_name = after_struct
        .split(|c: char| c.is_whitespace() || c == '{')
        .next()
        .expect("Expected struct name");

    let new_body = cfg_select! {
        feature = "mock-registers" => {
            format!("static MOCK: {struct_name} = unsafe {{ core::mem::zeroed() }}; &MOCK")
        }
        _ => {{
            let base_addr = args.to_string();
            format!("unsafe {{ &*({base_addr} as *const Self) }}")
        }}
    };

    let sync_impl = cfg_select! {
        feature = "mock-registers" => {format!("unsafe impl Sync for {struct_name} {{}}")}
        _ => ""
    };

    format!(
        r#"
        #[repr(C, align(4))]
        {input_str}
        
        {sync_impl}

        impl {struct_name} {{
            pub const fn new() -> &'static Self {{
                {new_body}
            }}
        }}
        "#,
        input_str = input_str,
        struct_name = struct_name,
        new_body = new_body
    )
    .parse()
    .unwrap()
}

/// Attribute macro to define a memory-mapped register block at the given address offset
/// in the user project area (i.e. on the Wishbone bus).
///
/// It is recommended to use
/// [volatile_register](https://docs.rs/volatile-register/latest/volatile_register/)
/// to handle access to the individual registers.
///
/// # Usage
/// ```rust,ignore
/// use volatile_register::{RO, RW};
///
/// #[user_register_block]
/// pub struct MyRegisters {
///     pub a: RO<u32>, // Address 0x3000_0000, read-only
///     pub b: [RW<u8>; 8], // Address 0x3000_0004 to 0x3000_000B, read-write
///     pub c: RW<u32>, // Address 0x3000_000C, read-write
/// }
///
/// #[user_register_block(0x100)]
/// pub struct MyRegistersTwo {
///     pub d: RW<u32>, // Address 0x3000_0100
///     _pad: [u32; 4], // Padding - no hardware mapped to these addresses
///     pub e: RW<u32>, // Address 0x3000_0114
/// }
/// ```
///
/// This will:
/// - Calculate the memory-mapped base address, by adding the offset to the base of the user area (0x3000_0000)
/// - Add `#[repr(C)]` to the struct
/// - Generate a `new()` method that returns a static reference at the base address
/// - If the `mock-registers` feature of `caravel-pac` is enabled,
///   a safe static instance is created instead of mapping to the specified address
#[proc_macro_attribute]
pub fn user_register_block(#[allow(unused)] args: TokenStream, input: TokenStream) -> TokenStream {
    // Add any offset to the user project base address
    let new_args: TokenStream = if args.is_empty() {
        "0x3000_0000".parse().unwrap()
    } else {
        format!("(0x3000_0000 + {args})").parse().unwrap()
    };
    register_block(new_args, input)
}
