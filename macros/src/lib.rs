use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{
    Error,
    parse_macro_input,
    ItemFn,
    Pat,
    FnArg,
    ReturnType,
};
use quote::{ToTokens, quote, format_ident};

#[proc_macro_attribute]
pub fn door(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {

    // parse the function this attribute was applied to
    let input = parse_macro_input!(item as ItemFn);

    // extract the function name
    let name = format_ident!("{}", input.sig.ident.to_string());

    // check number of arguments, we only support a single argument
    if input.sig.inputs.len() != 1 {
        return Error::new(
            input.sig.inputs.span(), 
            "only single argument doors supported",
        ).to_compile_error().into();
    }

    // extract the single argument and it's type
    let arg = &input.sig.inputs[0];
    let (arg_ident, arg_type) = match arg {

        FnArg::Receiver(_) => {
            return Error::new(
                arg.span(), 
                "only standalone functions supported",
            ).to_compile_error().into();
        },

        FnArg::Typed(pt) => {
            let p = match &*pt.pat {

                Pat::Ident(i) => i.ident.to_string(),

                _ => {
                    return Error::new(
                        arg.span(),
                        "only identifier arguments supported",
                    ).to_compile_error().into()
                }

            };
            (
                format_ident!("{}", p), 
                format_ident!("{}", (*pt.ty).to_token_stream().to_string()),
            )
        }
    };

    //extract the return type
    let return_type = match input.sig.output {
        ReturnType::Default => ReturnType::Default.to_token_stream(),
        ReturnType::Type(_, t) => (*t).to_token_stream(),
    };

    // extract the body of the function
    let blk = input.block;

    // generate the output function
    let q = quote! {

        unsafe extern "C" fn #name(
            _cookie: *mut std::os::raw::c_void,
            dataptr: *mut std::os::raw::c_char, 
            _datasize: rusty_doors::sys::size_t,
            _descptr: *mut rusty_doors::sys::door_desc_t,
            _ndesc: rusty_doors::sys::uint_t,
         ) {

            let f = || -> #return_type {
                let #arg_ident = *(dataptr as *mut #arg_type);
                #blk
            };

            let mut result = f();
            rusty_doors::sys::door_return(
                (&mut result as *mut #return_type) as *mut std::os::raw::c_char,
                std::mem::size_of::<#return_type>() as u64,
                std::ptr::null_mut(),
                0,
            );

        }

    };

    TokenStream::from(q)

}
