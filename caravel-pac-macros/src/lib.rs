use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed, LitInt, spanned::Spanned};

fn register_block_impl(
    mut input: DeriveInput,
    base_addr: LitInt,
    stride: usize,
    use_mock: bool,
) -> Result<proc_macro2::TokenStream, Error> {
    let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named: fields, .. }),
        ..
    }) = &mut input.data
    else {
        return Err(Error::new_spanned(
            &input,
            "register_block only supports structs with named fields",
        ));
    };

    // Add repr attribute
    input.attrs.push(syn::parse_quote! {
        #[repr(C, align(4))]
    });

    // Add padding fields to achieve the given stride between registers
    if stride != 0 {
        let mut new_fields = syn::punctuated::Punctuated::new();
        for (index, field) in fields.iter().enumerate() {
            let ty = &field.ty;
            let pad_name = format_ident!("_pad{}", index);

            new_fields.push(field.clone());
            new_fields.push(syn::parse_quote! {
                #pad_name: [u8; (#stride - (core::mem::size_of::<#ty>() % #stride)) % #stride]
            });
        }
        *fields = new_fields;
    }

    // Body for use in the generated new() function
    let struct_ident = input.ident.clone();

    let new_body = if use_mock {
        quote! {
            static MOCK: #struct_ident = unsafe { core::mem::zeroed() };
            &MOCK
        }
    } else {
        quote! {
            unsafe { &*(#base_addr as *const Self) }
        }
    };

    let sync_impl = if use_mock {
        quote! {
                unsafe impl Sync for #struct_ident {}
        }
    } else {
        TokenStream::new()
    };

    Ok(quote! {
        #input

        impl #struct_ident {
            pub const fn new() -> &'static Self {
                #new_body
            }
        }

        #sync_impl
    })
}

/// Attribute macro to define a memory-mapped register block with a base address
///
/// # Usage
/// ```rust,ignore
/// use volatile_register::{RO, RW};
///
/// #[register_block(base = 0x2000_0000)]
/// pub struct MyRegisters {
///     pub a: RO<u32>, // Address 0x2000_0000, read-only
/// }
///
/// #[register_block(base = 0x2000_0000, stride = 4)]
/// pub struct MySpacedRegisters {
///     pub a: RO<u8>, // Address 0x2000_0000
///     pub b: RO<u8>, // Address 0x2000_0004
/// }
/// ```
///
/// This will:
/// - Add `#[repr(C, align = 4)]` to the struct
/// - When `stride` is provided, align each register to that number of bytes
/// - e.g. `stride = 4` to align each register to a 32-bit boundary
/// - Generate a `new()` method that returns a static reference at the base address
/// - If the `mock-registers` feature of `caravel-pac` is enabled,
///   a safe static instance is created instead of mapping to the specified address
#[proc_macro_attribute]
pub fn register_block(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut base_addr = None;
    let mut stride = 0usize;

    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("base") {
            base_addr = Some(meta.value()?.parse::<syn::LitInt>()?);
        } else if meta.path.is_ident("stride") {
            stride = meta.value()?.parse::<syn::LitInt>()?.base10_parse()?;
        } else {
            return Err(meta.error("unexpected argument"));
        }

        Ok(())
    });

    syn::parse_macro_input!(args with parser);

    let base_addr = match base_addr {
        Some(value) => value,
        None => {
            return syn::Error::new(proc_macro2::Span::call_site(), "missing base address")
                .into_compile_error()
                .into();
        }
    };

    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let use_mock = cfg!(feature = "mock-registers");

    match register_block_impl(input, base_addr, stride, use_mock) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
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
/// #[user_register_block(offset = 0x100)]
/// pub struct MyRegistersTwo {
///     pub d: RW<u32>, // Address 0x3000_0100
///     _pad: [u32; 4], // Padding - no hardware mapped to these addresses
///     pub e: RW<u32>, // Address 0x3000_0114
/// }
///
/// #[user_register_block(offset = 0x200, stride = 4)]
/// pub struct MySpacedUserRegisters {
///     pub f: RW<u8>, // Address 0x3000_0200
///     pub g: RW<u8>, // Address 0x3000_0204
/// }
/// ```
///
/// This will:
/// - Calculate the memory-mapped base address by adding the offset to the user area base (0x3000_0000)
/// - Add `#[repr(C, align = 4)]` to the struct
/// - When `stride` is provided, align each register to that number of bytes
/// - e.g. `stride = 4` to align each register to a 32-bit boundary/// - Generate a `new()` method that returns a static reference at the base address
/// - If the `mock-registers` feature of `caravel-pac` is enabled,
///   a safe static instance is created instead of mapping to the specified address
#[proc_macro_attribute]
pub fn user_register_block(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    const MPRJ_BASE: u64 = 0x3000_0000;

    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let mut offset = 0u64;
    let mut stride = 0usize;

    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("offset") {
            offset = meta.value()?.parse::<syn::LitInt>()?.base10_parse()?;
        } else if meta.path.is_ident("stride") {
            stride = meta.value()?.parse::<syn::LitInt>()?.base10_parse()?;
        } else {
            return Err(meta.error("unexpected argument"));
        }
        Ok(())
    });
    syn::parse_macro_input!(args with parser);

    let base_addr = LitInt::new(&format!("{:#010x}", MPRJ_BASE + offset), input.span());

    let use_mock = cfg!(feature = "mock-registers") || cfg!(feature = "mock-user-registers");

    match register_block_impl(input, base_addr, stride, use_mock) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
