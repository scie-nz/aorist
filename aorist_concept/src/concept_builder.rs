use aorist_util::AoristError;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{
    parse_quote, Data, DataEnum, DeriveInput, Field, Fields, LitStr, Meta, NestedMeta, Path, Token,
};
mod keyword {
    syn::custom_keyword!(path);
}

pub trait TConceptBuilder {
    fn new(extra_derives: Vec<&str>) -> Result<Self, AoristError> where Self: Sized;
    fn get_derives(&self, attrs: Vec<NestedMeta>) -> (Vec<NestedMeta>, Vec<NestedMeta>) {
        let mut derivatives: Vec<NestedMeta> = Vec::new();
        let mut derives: Vec<NestedMeta> = Vec::new();
        for attr in attrs {
            if let NestedMeta::Meta(Meta::List(x)) = attr {
                if x.path.is_ident("derivative") {
                    derivatives = x.nested.into_iter().collect();
                } else if x.path.is_ident("derive") {
                    derives = x.nested.into_iter().collect();
                } else {
                    panic!("An attribute other than derive or derivative was specified");
                }
            } else {
                panic!("An attribute other than a MetaList was specified.");
            }
        }
        (derives, derivatives)
    }

    fn extend_metas(&self, ast: &mut DeriveInput, extra_metas: Vec<NestedMeta>, ident: &str) -> Result<(), AoristError> {
        let (attr, mut metas) = ast
            .attrs
            .iter_mut()
            .filter(|attr| attr.path.is_ident(ident))
            .filter_map(|attr| match attr.parse_meta() {
                Ok(Meta::List(meta_list)) => Some((attr, meta_list)),
                _ => None, // kcov-ignore
            })
            .next()
            .ok_or_else(|| AoristError::OtherError("Cannot extend metas".into()))?;
        metas
            .nested
            .extend::<Punctuated<NestedMeta, Token![,]>>(extra_metas.into_iter().collect());
        *attr = parse_quote!(#[#metas]);
        Ok(())
    }
    fn extend_derivatives(
        &self,
        ast: &mut DeriveInput,
        extra_derivatives: Vec<NestedMeta>,
    ) -> Result<(), AoristError> {
        self.extend_metas(ast, extra_derivatives, "derivative")
    }
    fn extend_derives(
        &self,
        ast: &mut DeriveInput,
        extra_derives: Vec<NestedMeta>,
    ) -> Result<(), AoristError> {
        self.extend_metas(ast, extra_derives, "derive")
    }
    fn get_extra_derives(&self) -> Vec<NestedMeta>;
    fn gen(
        &self,
        input_attrs: Vec<NestedMeta>,
        mut ast: DeriveInput,
    ) -> Result<TokenStream, AoristError> {
        let (mut extra_derives, extra_derivatives) = self.get_derives(input_attrs);
        for derive in self.get_extra_derives() {
            extra_derives.push(derive);
        }
        let quoted2 = match &mut ast.data {
            syn::Data::Struct(ref mut struct_data) => {
                self.add_aorist_fields(struct_data)?;
                let quoted = quote! {
                    #[repr(C)]
                    #[derive(
                        Derivative, Serialize, Deserialize, Clone, Hash,
                    )]
                    #[derivative(PartialEq, Debug, Eq)]
                    #ast
                };
                let mut final_ast: DeriveInput = syn::parse2(quoted)?;

                self.extend_derivatives(&mut final_ast, extra_derivatives)?;
                self.extend_derives(&mut final_ast, extra_derives)?;

                quote! { #final_ast }
            }
            Data::Enum(DataEnum { variants, .. }) => {
                let enum_name = &ast.ident;
                let variant = variants.iter().map(|x| (&x.ident)).collect::<Vec<_>>();
                let variant_type = variants.iter().map(|x| (&x.fields)).collect::<Vec<_>>();
                let quoted = quote! {
                    #[repr(C)]
                    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Hash, Eq)]
                    #[serde(tag = "type")]
                    pub enum #enum_name {
                        #(#variant(#variant_type)),*
                    }
                };
                let mut final_ast: DeriveInput = syn::parse2(quoted)?;
                self.extend_derives(&mut final_ast, extra_derives)?;
                self.extend_derives(&mut final_ast, extra_derivatives)?;

                quote! {
                    #final_ast
                    impl #enum_name {
                        pub fn is_same_variant_in_enum_as(&self, other: &Self) -> bool {
                            match (self, other) {
                                #(
                                    (#enum_name::#variant(_), #enum_name::#variant(_)) => true,
                                )*
                                _ => false,
                            }
                        }
                    }
                }
            }
            _ => panic!("expected a struct with named fields or an enum"),
        };
        Ok(proc_macro::TokenStream::from(quoted2))
    }
    fn add_aorist_fields(&self, struct_data: &mut syn::DataStruct) -> Result<(), AoristError>;
    fn parse_extra_derives(extra_derives_v: Vec<&str>) -> Result<Vec<NestedMeta>, AoristError> {
        let mut extra_derives = Vec::new();
        for t in extra_derives_v {
            let path = LitStr::new(t, Span::call_site()).parse_with(Path::parse_mod_style)?;
            let derive = NestedMeta::Meta(Meta::Path(path));
            extra_derives.push(derive);
        }
        Ok(extra_derives)
    }
}
pub struct RawConceptBuilder {
    extra_derives: Vec<NestedMeta>,
}
impl TConceptBuilder for RawConceptBuilder {
    fn new(extra_derives_v: Vec<&str>) -> Result<Self, AoristError> {
        Ok(Self {
            extra_derives: Self::parse_extra_derives(extra_derives_v)?,
        })
    }
    fn get_extra_derives(&self) -> Vec<NestedMeta> {
        self.extra_derives.clone()
    }
    fn add_aorist_fields(&self, struct_data: &mut syn::DataStruct) -> Result<(), AoristError> {
        match &mut struct_data.fields {
            Fields::Named(fields) => {
                fields.named.push(Field::parse_named.parse2(quote! {
                    pub uuid: Option<Uuid>
                })?);
                fields.named.push(Field::parse_named.parse2(quote! {
                    pub tag: Option<AString>
                })?);
            }
            _ => (),
        }
        Ok(())
    }
}
