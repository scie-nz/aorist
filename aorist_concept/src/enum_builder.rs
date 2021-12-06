extern crate proc_macro;
use self::proc_macro::TokenStream;
use crate::builder::Builder;
use proc_macro2::Ident;
use quote::quote;
use std::fs::OpenOptions;
use std::io::prelude::*;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Variant;
mod keyword {
    syn::custom_keyword!(path);
}

pub struct EnumBuilder {
    pub variant_idents: Vec<Ident>,
}
impl Builder for EnumBuilder {
    type TInput = syn::punctuated::Punctuated<Variant, Comma>;
    fn new(variants: &Punctuated<Variant, Comma>) -> Self {
        let variant_idents = variants
            .iter()
            .map(|x| (x.ident.clone()))
            .collect::<Vec<Ident>>();
        Self { variant_idents }
    }
    fn to_file(&self, enum_name: &Ident, file_name: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
            .unwrap();
        writeln!(
            file,
            "node [shape = box, fillcolor=gray, style=filled, fontname = Helvetica] '{}';",
            enum_name
        )
        .unwrap();

        for v in &self.variant_idents {
            writeln!(file, "'{}'->'{}';", enum_name, v).unwrap();
        }
    }
    fn to_concept_children_token_stream(&self, enum_name: &Ident) -> TokenStream {
        let variant = &self.variant_idents;
        TokenStream::from(quote! { paste! {
          impl <T> std::convert::From<(
              // enum name
              &str,
              // field name
              Option<&str>,
              // ix
              Option<usize>,
              // uuid
              Option<Uuid>,
              // wrapped reference
              #enum_name,
          )> for WrappedConcept<T> where
          #(
              T: [<CanBe #variant>],
          )*
              T: Debug + Clone + Serialize + PartialEq,
          {
              fn from(
                  tpl: (
                      &str,
                      Option<&str>,
                      Option<usize>,
                      Option<Uuid>,
                      #enum_name,
                  )
              ) -> Self {
                  let (name, field, ix, uuid, children_enum) = tpl;
                  match children_enum {
                      #(
                          #enum_name::#variant(ref x) => WrappedConcept{
                              inner: T::[<construct_ #variant:snake:lower>](x.clone(), ix, Some((uuid.unwrap(), name.into()))),
                          },
                      )*
                      _ => panic!("_phantom arm should not be activated"),
                  }
              }
          }
          impl <T> std::convert::From<(
              // enum name
              &str,
              // field name
              Option<&str>,
              // ix
              Option<usize>,
              // uuid
              Option<Uuid>,
              // wrapped reference
              AoristRef<#enum_name>,
          )> for WrappedConcept<T> where
          #(
              T: [<CanBe #variant>],
          )*
              T: Debug + Clone + Serialize + PartialEq,
          {
              fn from(
                  tpl: (
                      &str,
                      Option<&str>,
                      Option<usize>,
                      Option<Uuid>,
                      AoristRef<#enum_name>,
                  )
              ) -> Self {
                  let (name, field, ix, uuid, children_enum) = tpl;
                  let read = children_enum.0.read();
                  match &*read {
                      #(
                          #enum_name::#variant(ref x) => WrappedConcept{
                              inner: T::[<construct_ #variant:snake:lower>](
                                  x.clone(), ix, Some((uuid.unwrap(), name.into()))
                              ),
                          },
                      )*
                      _ => panic!("_phantom arm should not be activated"),
                  }
              }
          }
        }})
    }
    fn to_concept_token_stream(&self, enum_name: &Ident) -> TokenStream {
        let variant = &self.variant_idents;
        let py_class_name = format!("{}", enum_name);
        proc_macro::TokenStream::from(quote! { paste! {
          impl ConceptEnum for AoristRef<#enum_name> {}
          pub trait [<CanBe #enum_name>]: Debug + Clone + Serialize + PartialEq {
              fn [<construct_ #enum_name:snake:lower>] (
                  obj_ref: AoristRef<#enum_name>,
                  ix: Option<usize>,
                  id: Option<(Uuid, AString)>
              ) -> AoristRef<Self>;
          }
          #[cfg(feature = "python")]
          #[pyo3::prelude::pyclass(name=#py_class_name)]
          #[derive(Clone, PartialEq)]
          pub struct [<Py #enum_name>] {
              pub inner: AoristRef<#enum_name>,
          }
          #[cfg(feature = "python")]
          impl AoristRef<#enum_name> {
              pub fn py_object(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::PyObject> {
                  Ok(pyo3::PyObject::from(pyo3::PyCell::new(py, [<Py #enum_name>]{
                      inner: self.clone(),
                  })?))
              }
          }
          #[cfg(feature = "python")]
          #[derive(Clone, PartialEq, pyo3::prelude::FromPyObject)]
          pub enum [<Py #enum_name Input>] {
              #(
                  #variant([<Py #variant>]),
              )*
          }
          #[cfg(feature = "python")]
          #[pyo3::prelude::pymethods]
          impl [<Py #enum_name>] {
              pub fn deep_clone(&self) -> Self {
                  Self { inner: self.inner.deep_clone() }
              }
              #[staticmethod]
              pub fn child_concept_types() -> Vec<pyo3::prelude::PyObject> {
                  let gil_guard = pyo3::prelude::Python::acquire_gil();
                  let py = gil_guard.python();
                  vec![#(
                      pyo3::prelude::ToPyObject::to_object(
                          pyo3::types::PyType::new::<[<Py #variant>]>(py),
                          py
                      ),
                  )*]

              }
              #[staticmethod]
              pub fn is_enum_type() -> bool {
                  true
              }
              #[staticmethod]
              pub fn concrete_type_names() -> Vec<String> {
                  vec![#(
                      stringify!(#variant).into(),
                  )*]
              }

              #[new]
              pub fn new(
                  input: [<Py #enum_name Input>],
              ) -> Self {
                  match input {
                      #(
                          [<Py #enum_name Input>]::#variant(x) => {
                              let obj = #enum_name::#variant(x.inner.clone());
                              let inner = AoristRef(abi_stable::std_types::RArc::new(abi_stable::external_types::parking_lot::rw_lock::RRwLock::new(obj)));
                              Self { inner }
                          }
                      )*
                  }
              }
                #(
                    #[getter]
                    pub fn [<#variant:snake:lower>](&self) -> pyo3::prelude::PyResult<Option<[<Py #variant>]>> {
                        Ok(
                            match &*self.inner.0.read() {
                                #enum_name::#variant(x) => Some([<Py #variant>]{ inner: x.clone() }),
                                _ => None,
                            }
                        )
                    }
                )*
          }
          impl #enum_name {

              pub fn get_uuid(&self) -> Option<Uuid> {
                  match &self {
                      #(
                        #enum_name::#variant(x) => x.get_uuid(),
                      )*
                  }
              }
              pub fn deep_clone(&self) -> Self {
                  match &self {
                      #(
                        #enum_name::#variant(x) => #enum_name::#variant(x.deep_clone()),
                      )*
                  }
              }
              fn get_tag(&self) -> Option<AString> {
                  match self {
                      #(
                        #enum_name::#variant(x) => x.get_tag(),
                      )*
                  }
              }
              fn compute_uuids(&self) {
                  match self {
                      #(
                        #enum_name::#variant(x) => x.compute_uuids(),
                      )*
                  }
              }
              fn get_children_uuid(&self) -> Vec<Uuid> {
                  match self {
                      #(
                          #enum_name::#variant(x) => {
                              let t: AoristRef<#variant> = x.clone();
                              t.get_children_uuid()
                          },
                      )*
                  }
              }
          }
          impl AoristRef<#enum_name> {
              pub fn deep_clone(&self) -> Self {
                  AoristRef(abi_stable::std_types::RArc::new(abi_stable::external_types::parking_lot::rw_lock::RRwLock::new(self.0.read().deep_clone())))
              }
          }
          impl AoristConcept for AoristRef<#enum_name> {
              type TChildrenEnum = AoristRef<#enum_name>;
              fn get_children(&self) -> Vec<(
                  // enum name
                  &str,
                  // field name
                  Option<&str>,
                  // ix
                  Option<usize>,
                  // uuid
                  Option<Uuid>,
                  AoristRef<#enum_name>,
              )> {
                  vec![(
                      stringify!(#enum_name),
                      None,
                      None,
                      self.get_uuid(),
                      // clone of RArc<RRwLock
                      Self(self.0.clone()),
                  )]
              }
              fn get_uuid(&self) -> Option<Uuid> {
                  self.0.read().get_uuid()
              }
              fn get_tag(&self) -> Option<AString> {
                  self.0.read().get_tag()
              }
              fn get_children_uuid(&self) -> Vec<Uuid> {
                  self.0.read().get_children_uuid()
              }
              fn compute_uuids(&self) {
                  self.0.read().compute_uuids()
              }
          }
        }})
    }
}
