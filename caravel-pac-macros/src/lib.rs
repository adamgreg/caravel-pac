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
