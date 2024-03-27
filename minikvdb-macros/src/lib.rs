use proc_macro::TokenStream;

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(KVDBEntity)]
pub fn kvdb_entity(i: TokenStream) -> TokenStream {
    let s = i.to_string();
    let ast = syn::parse_str(&s).unwrap();
    gen_into_hashmap(&ast)
}

fn gen_into_hashmap(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let data = if let syn::Data::Struct(data) = &ast.data {
        data
    } else {
        unimplemented!();
    };

    let fields = data.fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            (stringify!(#name).to_string(), v.#name.into())
        }
    });

    let fields_get = data.fields.iter().map(|f| {
        let name = &f.ident;
        quote!{
            #name: v.get(stringify!(#name)).ok_or(lib_minikvdb::error::MiniKVDBError::MissingField(stringify!(#name).to_string()))?.try_into()?
        }
    });

    let gen = quote! {
        #[automatically_derived]
        impl From<#name> for lib_minikvdb::prelude::KVDBObject {
            fn from(v: #name) -> lib_minikvdb::prelude::KVDBObject {
                [
                    #(#fields,)*
                ].into()
            }
        }
        #[automatically_derived]
        impl TryFrom<lib_minikvdb::prelude::KVDBObject> for #name {
            type Error = lib_minikvdb::error::MiniKVDBError;
            fn try_from(v: lib_minikvdb::prelude::KVDBObject) -> Result<Self, Self::Error> {
                    Ok(Self {
                        #(#fields_get,)*
                    })
            }
        }
    };

    gen.into()
}
