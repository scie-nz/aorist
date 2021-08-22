extern crate proc_macro;
use self::proc_macro::TokenStream;
use crate::builder::Builder;
use aorist_util::{
    extract_type_from_aorist_ref, extract_type_from_map, extract_type_from_option,
    extract_type_from_vector,
};
use proc_macro2::Ident;
use quote::quote;
use std::fs::OpenOptions;
use std::io::prelude::*;
use syn::{Field, FieldsNamed, Meta, Type};
mod keyword {
    syn::custom_keyword!(path);
}
use linked_hash_set::LinkedHashSet;

fn extract_names_and_types(fields: &Vec<Field>) -> (
    Vec<Ident>, 
    Vec<Type>,
    Vec<Ident>,
    Vec<Type>,
    Vec<Ident>,
    Vec<Type>,
) {
    let mut names: Vec<Ident> = Vec::new();
    let mut types: Vec<Type> = Vec::new();
    let mut names_ref: Vec<Ident> = Vec::new();
    let mut types_ref: Vec<Type> = Vec::new();
    let mut names_vec_ref: Vec<Ident> = Vec::new();
    let mut types_vec_ref: Vec<Type> = Vec::new();
    for field in fields {
        if let Some(t) = extract_type_from_aorist_ref(&field.ty) {
            names_ref.push(field.ident.as_ref().unwrap().clone());
            types_ref.push(t.clone());
        } else if let Some(ref vt) = extract_type_from_vector(&field.ty) {
            if let Some(t) = extract_type_from_aorist_ref(vt) {
                names_vec_ref.push(field.ident.as_ref().unwrap().clone());
                types_vec_ref.push(t.clone());
            } else {
                names.push(field.ident.as_ref().unwrap().clone());
                types.push(field.ty.clone());
            }
        }
        else {
            names.push(field.ident.as_ref().unwrap().clone());
            types.push(field.ty.clone());
        }
    }
    (names, types, names_ref, types_ref, names_vec_ref, types_vec_ref)
}

fn field_is_constrainable(field: &Field) -> bool {
    for a in &field.attrs {
        if let Ok(Meta::Path(x)) = a.parse_meta() {
            if x.is_ident("constrainable") {
                return true;
            }
        }
    }
    return false;
}

pub fn get_constrainable_fields(fields: Vec<Field>) -> (Vec<Field>, Vec<Field>) {
    let mut constrainable_fields: Vec<Field> = Vec::new();
    let mut unconstrainable_fields: Vec<Field> = Vec::new();
    for field in fields {
        if field_is_constrainable(&field) {
            constrainable_fields.push(field);
        } else {
            unconstrainable_fields.push(field);
        }
    }
    (constrainable_fields, unconstrainable_fields)
}

pub struct StructBuilder {
    pub bare_types: Vec<Type>,
    pub vec_types: Vec<Type>,
    pub option_vec_types: Vec<Type>,
    pub option_types: Vec<Type>,
    pub map_key_types: Vec<Type>,
    pub map_value_types: Vec<Type>,
    pub bare_idents: Vec<Ident>,
    pub vec_idents: Vec<Ident>,
    pub option_vec_idents: Vec<Ident>,
    pub option_idents: Vec<Ident>,
    pub map_idents: Vec<Ident>,
    pub fields_with_default: syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
    pub unconstrainable: Vec<Field>,
}
impl StructBuilder {
    pub fn get_all_types(&self) -> Vec<&Type> {
        self.bare_types
            .iter()
            .chain(self.vec_types.iter())
            .chain(self.option_types.iter())
            .chain(self.option_vec_types.iter())
            .chain(self.map_value_types.iter())
            .map(|x| extract_type_from_aorist_ref(x).unwrap())
            .collect::<LinkedHashSet<_>>()
            .into_iter()
            .collect()
    }
}
impl Builder for StructBuilder {
    type TInput = FieldsNamed;
    fn new(fields: &FieldsNamed) -> StructBuilder {
        let fields_filtered = fields
            .named
            .clone()
            .into_iter()
            .filter(|x| {
                let ident = x.ident.as_ref().unwrap().to_string();
                !(ident == "tag" || ident == "uuid" || ident == "constraints")
            })
            .collect::<Vec<_>>();
        let fields_with_default = fields_filtered
            .clone()
            .into_iter()
            .map(|x| {
                let mut it = x
                    .attrs
                    .iter()
                    .map(|attr| {
                        let meta = attr.parse_meta().unwrap();
                        if let syn::Meta::NameValue(ref nv) = meta {
                            if nv.path.is_ident("py_default") {
                                if let syn::Lit::Str(_) = nv.lit {
                                    let field_name = x.ident.as_ref().unwrap().clone();
                                    return Some(syn::MetaNameValue {
                                        path: syn::Path::from(field_name),
                                        eq_token: nv.eq_token.clone(),
                                        lit: nv.lit.clone(),
                                    });
                                } else {
                                    panic!("py_default values should only be strings");
                                }
                            }
                        }
                        None
                    })
                    .filter(|x| x.is_some());
                let default_val = it.next();
                if let Some(x) = default_val {
                    assert!(it.next().is_none());
                    return x;
                }
                None
            })
            .filter(|x| x.is_some())
            .map(|x| syn::NestedMeta::Meta(syn::Meta::NameValue(x.unwrap())))
            .collect::<syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>>();

        let (constrainable, unconstrainable) = get_constrainable_fields(fields_filtered.clone());

        let mut bare_types: Vec<Type> = Vec::new();
        let mut vec_types: Vec<Type> = Vec::new();
        let mut option_vec_types: Vec<Type> = Vec::new();
        let mut option_types: Vec<Type> = Vec::new();
        let mut map_key_types: Vec<Type> = Vec::new();
        let mut map_value_types: Vec<Type> = Vec::new();

        let mut bare_idents: Vec<Ident> = Vec::new();
        let mut vec_idents: Vec<Ident> = Vec::new();
        let mut option_vec_idents: Vec<Ident> = Vec::new();
        let mut option_idents: Vec<Ident> = Vec::new();
        let mut map_idents: Vec<Ident> = Vec::new();

        for field in constrainable {
            let tt = &field.ty;
            let ident = field.ident.as_ref().unwrap().clone();

            if let Some(vec_type) = extract_type_from_vector(tt) {
                vec_types.push(vec_type.clone());
                vec_idents.push(ident.clone());
            } else if let Some(option_type) = extract_type_from_option(tt) {
                if let Some(option_vec_type) = extract_type_from_vector(option_type) {
                    option_vec_types.push(option_vec_type.clone());
                    option_vec_idents.push(ident.clone());
                } else {
                    option_types.push(option_type.clone());
                    option_idents.push(ident.clone());
                }
            } else if let Some((map_key_type, map_value_type)) = extract_type_from_map(tt) {
                map_key_types.push(map_key_type.clone());
                map_value_types.push(map_value_type.clone());
                map_idents.push(ident.clone());
            } else {
                bare_types.push(tt.clone());
                bare_idents.push(ident.clone());
            }
        }
        Self {
            bare_types,
            vec_types,
            option_types,
            option_vec_types,
            map_key_types,
            map_value_types,
            bare_idents,
            vec_idents,
            option_idents,
            option_vec_idents,
            map_idents,
            fields_with_default,
            unconstrainable,
        }
    }
    fn to_file(&self, struct_name: &Ident, file_name: &str) {
        let types = self
            .bare_idents
            .iter()
            .chain(self.option_vec_idents.iter())
            .chain(self.option_idents.iter())
            .chain(self.vec_idents.iter())
            .zip(
                self.bare_types
                    .iter()
                    .chain(self.option_vec_types.iter())
                    .chain(self.option_types.iter())
                    .chain(self.vec_types.iter()),
            );
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
            .unwrap();
        writeln!(
            file,
            "node [shape = oval, fillcolor=white, style=filled, fontname = Helvetica] '{}';",
            struct_name
        )
        .unwrap();
        for (ident, t) in types {
            let tp = match t {
                Type::Path(x) => &x.path,
                _ => panic!("Something other than a type path found."),
            };
            let type_val = tp
                .segments
                .iter()
                .map(|x| x.ident.to_string())
                .collect::<Vec<_>>()
                .join("|");
            writeln!(
                file,
                "'{}'->'{}' [label='{}'];",
                struct_name, type_val, ident,
            )
            .unwrap();
        }
    }
    fn to_concept_children_token_stream(&self, struct_name: &Ident) -> TokenStream {
        let types = self.get_all_types();

        TokenStream::from(quote! { paste! {

            impl <T> std::convert::From<(
                // struct name
                &str,
                // field name
                Option<&str>,
                // ix
                Option<usize>,
                // uuid
                Option<Uuid>,
                // wrapped reference
                [<#struct_name Children>]
            )> for WrappedConcept<T> where
            #(
                T: [<CanBe #types>],
            )*
                T: Debug + Clone + Serialize + PartialEq,
            {
                fn from(
                    tpl: (
                        &str,
                        Option<&str>,
                        Option<usize>,
                        Option<Uuid>,
                        [<#struct_name Children>]
                    )
                ) -> Self {
                    let (name, field, ix, uuid, children_enum) = tpl;
                    match children_enum {
                        #(
                            [<#struct_name Children>]::#types(x) => WrappedConcept{
                                inner: T::[<construct_ #types:snake:lower>](x, ix, Some((uuid.unwrap(), name.to_string()))),
                            },
                        )*
                        _ => panic!("_phantom arm should not be activated"),
                    }
                }
            }
        }})
    }
    fn to_concept_token_stream(&self, struct_name: &Ident) -> TokenStream {
        let (
            bare_ident,
            vec_ident,
            option_ident,
            option_vec_ident,
            map_ident,
            bare_type,
            vec_type,
            option_type,
            option_vec_type,
            map_value_type,
        ) = (
            &self.bare_idents,
            &self.vec_idents,
            &self.option_idents,
            &self.option_vec_idents,
            &self.map_idents,
            &self.bare_types,
            &self.vec_types,
            &self.option_types,
            &self.option_vec_types,
            &self.map_value_types,
        );
        let (
            unconstrainable_name,
            unconstrainable_type,
            unconstrainable_name_ref,
            unconstrainable_type_ref,
            unconstrainable_name_vec_ref,
            unconstrainable_type_vec_ref,
        ) =
            extract_names_and_types(&self.unconstrainable);
        let bare_type_deref = bare_type
            .iter()
            .map(|x| extract_type_from_aorist_ref(x))
            .collect::<Vec<_>>();
        let vec_type_deref = vec_type
            .iter()
            .map(|x| extract_type_from_aorist_ref(x))
            .collect::<Vec<_>>();
        let option_vec_type_deref = option_vec_type
            .iter()
            .map(|x| extract_type_from_aorist_ref(x))
            .collect::<Vec<_>>();
        let option_type_deref = option_type
            .iter()
            .map(|x| extract_type_from_aorist_ref(x))
            .collect::<Vec<_>>();
        let map_value_type_deref = map_value_type
            .iter()
            .map(|x| extract_type_from_aorist_ref(x))
            .collect::<Vec<_>>();
        let py_class_name = format!("{}", struct_name);
        let types = self.get_all_types();
        TokenStream::from(quote! { paste! {
            pub enum [<#struct_name Children>] {
                #(
                    #types(AoristRef<#types>),
                )*
            }

            #[cfg(feature = "python")]
            #[pyo3::prelude::pyclass(name=#py_class_name)]
            #[derive(Clone, PartialEq)]
            pub struct [<Py #struct_name>] {
                pub inner: AoristRef<#struct_name>,
            }
            #[cfg(feature = "python")]
            #[pyo3::prelude::pymethods]
            impl [<Py #struct_name>] {
                pub fn compute_uuids(&self) {
                    self.inner.compute_uuids()
                }
                #[staticmethod]
                pub fn is_enum_type() -> bool {
                    false
                }
                #[staticmethod]
                pub fn required_unique_children_type_names() -> Vec<String> {
                    vec![#(
                        stringify!(#bare_type_deref).into(),
                    )*]
                }
                #[staticmethod]
                pub fn optional_unique_children_type_names() -> Vec<String> {
                    vec![#(
                        stringify!(#option_type_deref).into(),
                    )*]
                }
                #[staticmethod]
                pub fn required_list_children_type_names() -> Vec<String> {
                    vec![#(
                        stringify!(#vec_type_deref).into(),
                    )*]
                }
                #[staticmethod]
                pub fn optional_list_children_type_names() -> Vec<String> {
                    vec![#(
                        stringify!(#option_vec_type_deref).into(),
                    )*]
                }
                #[staticmethod]
                pub fn children_dict_type_names() -> Vec<String> {
                    vec![#(
                        stringify!(#map_value_type_deref).into(),
                    )*]
                }
                #[new]
                pub fn new(
                    #(
                        #bare_ident: [<Py #bare_type_deref>],
                    )*
                    #(#vec_ident: Vec<[<Py #vec_type_deref>]> ,)*
                    #(#option_ident: Option<[<Py #option_type_deref>]> ,)*
                    #(#option_vec_ident: Option<Vec<[<Py #option_vec_type_deref>]>> ,)*
                    #(
                      #map_ident: std::collections::BTreeMap<
                        String, [<Py #map_value_type_deref>]
                      >,
                    )*
                    #(
                        #unconstrainable_name_ref: [<Py #unconstrainable_type_ref>],
                    )*
                    #(
                        #unconstrainable_name_vec_ref: Vec<[<Py #unconstrainable_type_vec_ref>]>,
                    )*
                    #(
                        #unconstrainable_name: #unconstrainable_type,
                    )*
                    tag: Option<String>,
                ) -> Self {
                    let obj = #struct_name {
                        #(
                            #bare_ident: #bare_ident.inner.clone(),
                        )*
                        #(
                            #vec_ident: #vec_ident.iter().map(
                                |x| x.inner.clone()
                            ).collect(),
                        )*
                        #(
                            #option_ident: #option_ident.and_then(
                                |x| Some(x.inner.clone())
                            ),
                        )*
                        #(
                            #option_vec_ident: #option_vec_ident.and_then(
                                |x| Some(x.iter().map(
                                    |y| y.inner.clone()
                                ).collect())
                            ),
                        )*
                        #(
                            #map_ident: #map_ident.iter().map(
                                |(k, v)| (k.clone(), v.inner.clone())
                            ).collect(),
                        )*
                        #(
                            #unconstrainable_name,
                        )*
                        #(
                            #unconstrainable_name_ref: #unconstrainable_name_ref.inner.clone(),
                        )*
                        #(
                            #unconstrainable_name_vec_ref: #unconstrainable_name_vec_ref.iter().map(
                                |x| x.inner.clone()
                            ).collect(),
                        )*
                        tag,
                        uuid: None,
                    };
                    let inner = AoristRef(std::sync::Arc::new(std::sync::RwLock::new(
                        obj
                    )));
                    Self { inner }
                }
                #[getter]
                pub fn tag(&self) -> pyo3::prelude::PyResult<Option<String>> {
                    Ok(self.inner.0.read().unwrap().tag.clone())
                }
                #(
                    #[getter]
                    pub fn #bare_ident(&self) -> pyo3::prelude::PyResult<[<Py #bare_type_deref>]> {
                        Ok(
                            [<Py #bare_type_deref>] {
                                inner: self.inner.0.read().unwrap().#bare_ident.clone(),
                            }
                        )
                    }
                    #[setter]
                    pub fn [<set_#bare_ident>](&self, val: [<Py #bare_type_deref>]) -> pyo3::prelude::PyResult<()> {
                        Ok(
                            (*self.inner.0.write().unwrap()).#bare_ident = val.inner.clone()
                        )
                    }
                )*
                #(
                    #[getter]
                    pub fn #option_ident(&self) -> pyo3::prelude::PyResult<Option<[<Py #option_type_deref>]>> {
                        Ok(
                            self.inner.0.read().unwrap().#option_ident.as_ref().and_then(|x|
                                Some([<Py #option_type_deref>] {
                                    inner: x.clone()
                                })
                            )
                        )
                    }
                    #[setter]
                    pub fn [<set_#option_ident>](&self, val: Option<[<Py #option_type_deref>]>) -> pyo3::prelude::PyResult<()> {
                        Ok(
                            (*self.inner.0.write().unwrap()).#option_ident = val.and_then(|x| Some(x.inner.clone()))
                        )
                    }
                )*
                #(
                    #[getter]
                    pub fn #vec_ident(&self) -> pyo3::prelude::PyResult<Vec<[<Py #vec_type_deref>]>> {
                        Ok(
                            self.inner.0.read().unwrap().#vec_ident.iter().map(|x| {
                                [<Py #vec_type_deref>] {
                                    inner: x.clone(),
                                }
                            }).collect::<Vec<_>>()
                        )
                    }
                    #[setter]
                    pub fn [<set_#vec_ident>](&self, val: Vec<[<Py #vec_type_deref>]>) -> pyo3::prelude::PyResult<()> {
                        Ok(
                            (*self.inner.0.write().unwrap()).#vec_ident = val.iter().map(|x| x.inner.clone()).collect()
                        )
                    }
                )*
                #(
                    #[getter]
                    pub fn #option_vec_ident(&self) -> pyo3::prelude::PyResult<Option<
                        Vec<[<Py #option_vec_type_deref>]>
                    >> {
                        Ok(
                            self.inner.0.read().unwrap().#option_vec_ident.as_ref().and_then(|x|
                                Some(
                                    x.iter().map(|y| {
                                        [<Py #option_vec_type_deref>] {
                                            inner: y.clone()
                                        }
                                    }).collect()
                                )
                            )
                        )
                    }
                    #[setter]
                    pub fn [<set_#option_vec_ident>](
                        &self,
                        val: Option<Vec<[<Py #option_vec_type_deref>]>>
                    ) -> pyo3::prelude::PyResult<()> {
                        Ok(
                            (*self.inner.0.write().unwrap()).#option_vec_ident = val.and_then(
                                |x| Some(
                                    x.iter().map(|y| y.inner.clone()).collect()
                                )
                            )
                        )
                    }
                )*
                #(
                    #[getter]
                    pub fn #map_ident(&self) -> pyo3::prelude::PyResult<BTreeMap<
                        String, [<Py #map_value_type_deref>]>
                    > {
                        Ok(
                            self.inner.0.read().unwrap().#map_ident.iter().map(|(k, v)| {(
                                k.clone(),
                                [<Py #map_value_type_deref>] {
                                    inner: v.clone(),
                                }
                            )}).collect()
                        )
                    }
                    #[setter]
                    pub fn [<set_#map_ident>](
                        &self,
                        val: BTreeMap<String, [<Py #map_value_type_deref>]>
                    ) -> pyo3::prelude::PyResult<()> {
                        Ok(
                            (*self.inner.0.write().unwrap()).#map_ident = val.iter().map(
                                |(k, v)| (k.clone(), v.inner.clone())
                            ).collect()
                        )
                    }
                )*
                #(
                    #[getter]
                    pub fn #unconstrainable_name(&self) -> pyo3::prelude::PyResult<#unconstrainable_type> {
                        Ok(self.inner.0.read().unwrap().#unconstrainable_name.clone())
                    }
                )*
                #(
                    #[setter]
                    pub fn [<set_#unconstrainable_name>](&mut self, val: #unconstrainable_type) -> pyo3::prelude::PyResult<()> {
                        Ok((*self.inner.0.write().unwrap()).#unconstrainable_name = val)
                    }
                )*
            }
            #[cfg(feature = "python")]
            #[pyo3::prelude::pyproto]
            impl pyo3::PyObjectProtocol for [<Py #struct_name>] {
                fn __repr__(&self) -> pyo3::PyResult<String> {
                    Ok(format!(
                        "{} {}",
                        stringify!(#struct_name),
                        serde_json::to_string_pretty(&*self.inner.0.read().unwrap()).unwrap()
                    ))
                }
                fn __str__(&self) -> pyo3::PyResult<String> {
                    Ok(format!(
                        "{} {}",
                        stringify!(#struct_name),
                        serde_json::to_string_pretty(&*self.inner.0.read().unwrap()).unwrap()
                    ))
                }
            }
            #[cfg(feature = "python")]
            impl AoristRef<#struct_name> {
                pub fn py_object(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::PyObject> {
                    Ok(pyo3::PyObject::from(pyo3::PyCell::new(py, [<Py #struct_name>]{
                        inner: self.clone(),
                    })?))
                }
            }
            impl AoristRef<#struct_name> {
                pub fn deep_clone(&self) -> Self {
                    AoristRef(std::sync::Arc::new(std::sync::RwLock::new(self.0.read().unwrap().deep_clone())))
                }
            }
            impl #struct_name {
                pub fn get_uuid(&self) -> Option<Uuid> {
                    self.uuid.clone()
                }
                fn deep_clone(&self) -> Self {
                    assert!(self.uuid.is_none());
                    Self {
                        #(
                            #bare_ident: self.#bare_ident.deep_clone(),
                        )*
                        #(
                            #option_ident: self.#option_ident.as_ref().and_then(|x| Some(x.deep_clone())),
                        )*
                        #(
                            #vec_ident: self.#vec_ident.iter().map(|x| x.deep_clone()).collect(),
                        )*
                        #(
                            #option_vec_ident: self.#option_vec_ident.as_ref().and_then(|x| Some(
                                x.iter().map(|x| x.deep_clone()).collect()
                            )),
                        )*
                        #(
                            #map_ident: self.#map_ident.iter().map(|(k, v)| (k.clone(), v.deep_clone())).collect(),
                        )*
                        #(
                            #unconstrainable_name: self.#unconstrainable_name.clone(),
                        )*
                        #(
                            #unconstrainable_name_ref: self.#unconstrainable_name_ref.clone(),
                        )*
                        #(
                            #unconstrainable_name_vec_ref: self.#unconstrainable_name_vec_ref.clone(),
                        )*
                        tag: self.tag.clone(),
                        uuid: None,
                    }
                }
                fn compute_uuids(&mut self) {
                    #(
                        self.#bare_ident.compute_uuids();
                    )*
                    #(
                        if let Some(ref c) = self.#option_ident {
                            c.compute_uuids();
                        }
                    )*
                    #(
                        for elem in self.#vec_ident.iter() {
                            elem.compute_uuids();
                        }
                    )*
                    #(
                        if let Some(ref mut v) = self.#option_vec_ident {
                            for elem in v.iter() {
                                elem.compute_uuids();
                            }
                        }
                    )*
                    #(
                        for elem in self.#map_ident.values() {
                            elem.compute_uuids();
                        }
                    )*
                }
                fn set_uuid(&mut self, uuid: Uuid) {
                    self.uuid = Some(uuid);
                }
                fn get_tag(&self) -> Option<String> {
                    self.tag.clone()
                }
                #(
                    pub fn #bare_ident(&self) -> #bare_type {
                        self.#bare_ident.clone()
                    }
                )*
                #(
                    pub fn #option_ident(&self) -> Option<#option_type> {
                        self.#option_ident.clone()
                    }
                )*
                #(
                    pub fn #vec_ident(&self) -> Vec<#vec_type> {
                        self.#vec_ident.clone()
                    }
                )*
                #(
                    pub fn #option_vec_ident(&self) -> Option<Vec<#option_vec_type>> {
                        self.#option_vec_ident.clone()
                    }
                )*
                #(
                    pub fn #map_ident(&self) -> BTreeMap<String, #map_value_type> {
                        self.#map_ident.clone()
                    }
                )*
            }
            impl [<#struct_name Children>] {
                pub fn get_uuid(&self) -> Option<Uuid> {
                    match &self {
                        #(
                            Self::#types(x) => x.get_uuid(),
                        )*
                        _ => panic!("phantom arm was activated.")
                    }
                }
            }
            impl ConceptEnum for [<#struct_name Children>] {}
            pub trait [<CanBe #struct_name>]: Debug + Clone + Serialize + PartialEq {
                fn [<construct_ #struct_name:snake:lower>](
                    obj_ref: AoristRef<#struct_name>,
                    ix: Option<usize>,
                    id: Option<(Uuid, String)>
                ) -> AoristRef<Self>;
            }

            impl AoristConcept for AoristRef<#struct_name> {
                type TChildrenEnum = [<#struct_name Children>];
                fn get_uuid(&self) -> Option<Uuid> {
                    let read_lock = self.0.read().unwrap();
                    if let Ok(ref x) = self.0.read() {
                        return x.get_uuid();
                    }
                    panic!("Could not open object {} for reading.", stringify!(#struct_name));
                }
                fn compute_uuids(&self) {
                    if let Ok(ref mut x) = self.0.write() {
                        x.compute_uuids();
                    } else {
                        panic!("Could not open object {} for writing.", stringify!(#struct_name));
                    }
                    let uuid;
                    if let Ok(ref x) = self.0.read() {
                        uuid = self.get_uuid_from_children_uuid();
                    } else {
                        panic!("Could not open object {} for reading.", stringify!(#struct_name));
                    }
                    if let Ok(ref mut x) = self.0.write() {
                        x.set_uuid(uuid);
                    } else {
                        panic!("Could not open object {} for writing.", stringify!(#struct_name));
                    }
                }
                fn get_children_uuid(&self) -> Vec<Uuid> {
                    self.get_children().iter().map(|x| x.4.get_uuid().unwrap()).collect()
                }
                fn get_tag(&self) -> Option<String> {
                    let read_lock = self.0.read().unwrap();
                    if let Ok(ref x) = self.0.read() {
                        return x.get_tag();
                    }
                    panic!("Could not open object {} for reading.", stringify!(#struct_name));
                }
                fn get_children(&self) -> Vec<(
                    // struct name
                    &str,
                    // field name
                    Option<&str>,
                    // ix
                    Option<usize>,
                    // uuid
                    Option<Uuid>,
                    // wrapped reference
                    [<#struct_name Children>],
                )> {
                    let mut children: Vec<_> = Vec::new();
                    let read = self.0.read().unwrap();
                    #(
                        children.push((
                            stringify!(#struct_name),
                            Some(stringify!(#bare_ident)),
                            None,
                            self.get_uuid(),
                            [<#struct_name Children>]::#bare_type_deref(read.#bare_ident())
                        ));
                    )*
                    #(
                        if let Some(c) = read.#option_ident() {
                            children.push((
                                stringify!(#struct_name),
                                Some(stringify!(#option_ident)),
                                None,
                                self.get_uuid(),
                                [<#struct_name Children>]::#option_type_deref(c)
                            ));
                        }
                    )*
                    #(
                        for (ix, elem) in read.#vec_ident().into_iter().enumerate() {
                            children.push((
                                stringify!(#struct_name),
                                Some(stringify!(#vec_ident)),
                                Some(ix),
                                self.get_uuid(),
                                [<#struct_name Children>]::#vec_type_deref(elem)
                            ));
                        }
                    )*
                    #(
                        if let Some(v) = read.#option_vec_ident() {
                            for (ix, elem) in v.into_iter().enumerate() {
                                children.push((
                                    stringify!(#struct_name),
                                    Some(stringify!(#option_vec_ident)),
                                    Some(ix),
                                    read.get_uuid(),
                                    [<#struct_name Children>]::#option_vec_type_deref(elem)
                                ));
                            }
                        }
                    )*
                    #(
                        for elem in read.#map_ident().values() {
                            children.push((
                                stringify!(#struct_name),
                                Some(stringify!(#map_ident)),
                                None,
                                read.get_uuid(),
                                [<#struct_name Children>]::#map_value_type_deref(elem.clone())
                            ));
                        }
                    )*
                    children
                }
            }
        }})
    }
}
