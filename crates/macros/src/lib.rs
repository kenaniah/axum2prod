extern crate proc_macro;

#[proc_macro_attribute]
pub fn isolated_test(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemFn);
    //let name = item.sig.ident.clone();
    let block = item.block.clone();
    let attrs = item.attrs.clone();
    let vis = item.vis.clone();
    let sig = item.sig.clone();
    let output = quote::quote! {

        #(#attrs)*
        #[tokio::test]
        #vis #sig {
            axum2prod::test_helpers::run_test(|ctx| {
                Box::pin(async move {
                    #block
                })
            })
            .await;
        }
    };
    output.into()
}
