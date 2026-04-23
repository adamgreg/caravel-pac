use proc_macro::TokenStream;

/// Attribute macro to define a memory-mapped register block with a base address
///
/// # Usage
/// ```ignore
/// #[register_block(0x2000_0000)]
/// pub struct MyRegisters { ... }
/// ```
///
/// This will:
/// - Add `#[repr(C)]` to the struct
/// - Generate a `new()` method that returns a static reference at the base address
#[proc_macro_attribute]
pub fn register_block(args: TokenStream, input: TokenStream) -> TokenStream {
    let base_addr = args.to_string();
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

    format!(
        r#"
        #[repr(C, align(4))]
        {input_str}
        
        #[cfg(feature = "mock-registers")]
        unsafe impl Sync for {struct_name} {{}}

        impl {struct_name} {{
            pub const fn new() -> &'static Self {{
                #[cfg(not(feature = "mock-registers"))]
                unsafe {{ &*({base_addr} as *const Self) }}
                #[cfg(feature = "mock-registers")]
                {{ static MOCK: {struct_name} = unsafe {{ core::mem::zeroed() }}; &MOCK }}
            }}
        }}
        "#,
        input_str = input_str,
        struct_name = struct_name,
        base_addr = base_addr
    )
    .parse()
    .unwrap()
}
