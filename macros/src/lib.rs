use proc_macro::TokenStream;

mod raw_builder;

use raw_builder::BuilderContext;

mod raw_generate;

use raw_generate::GenerateContext;

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}

// #[proc_macro_derive(RawBuilder)]
// pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
//     println!("{:#?}", input);
//     TokenStream::default()
// }

#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    BuilderContext::render(input).unwrap().parse().unwrap()
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let s = GenerateContext::render(input).unwrap();
    println!("{}", s);
    s.parse().unwrap()
}
