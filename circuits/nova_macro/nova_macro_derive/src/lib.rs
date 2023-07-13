use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(NovaRoundInput)]
pub fn nova_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_nova_macro(&ast)
}

fn impl_nova_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = ast.data.clone()
    {
        named
    } else {
        unimplemented!()
    };

    let insertions = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        quote! {
            map.insert(
                stringify!(#name).to_string(),
                ::serde_json::json!(self.#name)
            );
        }
    });

    let gen = quote! {
        impl NovaRoundInput for #name {
            fn circuit_input(&self) -> std::collections::HashMap<String, serde_json::Value> {
                let mut map = std::collections::HashMap::new();
                #(#insertions)*
                map
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(NovaInput)]
pub fn outer_input_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_outer_input_macro(&ast)
}

fn impl_outer_input_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        use pasta_curves::Fq;
        use serde_json::Value;

        impl NovaInput for #name {
            fn initial_inputs(&self) -> Vec<Fq> {
                self.step_in
                    .to_vec()
                    .iter()
                    .map(|x| F1::from_str_vartime(x).unwrap())
                    .collect()
            }

            fn round_inputs(&self) -> Vec<HashMap<String, Value>> {
                self.private_inputs
                    .iter()
                    .map(|v| v.circuit_input())
                    .collect()
            }
        }
    };
    gen.into()
}
