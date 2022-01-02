extern crate proc_macro;
use self::proc_macro::TokenStream;
use crate::builder::Builder;
use aorist_util::AoristError;
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
    fn new(variants: &Punctuated<Variant, Comma>) -> Result<Self, AoristError> {
        let variant_idents = variants
            .iter()
            .map(|x| (x.ident.clone()))
            .collect::<Vec<Ident>>();
        Ok(Self { variant_idents })
    }
    fn to_file(&self, enum_name: &Ident, file_name: &str) -> Result<(), AoristError> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)?;
        writeln!(
            file,
            "node [shape = box, fillcolor=gray, style=filled, fontname = Helvetica] '{}';",
            enum_name
        )?;

        for v in self.variant_idents.iter() {
            writeln!(file, "'{}'->'{}';", enum_name, v)?;
        }
        Ok(())
    }
    fn to_concept_children_token_stream(
        &self,
        enum_name: &Ident,
    ) -> Result<TokenStream, AoristError> {
        let variant = &self.variant_idents;
        Ok(TokenStream::from(quote! { paste! {
          impl #enum_name {
              pub fn convert<T>(&self, name: AString, field: AOption<AString>, ix: AOption<usize>, uuid: AOption<Uuid>) -> AoristRef<T> 
              where 
                  #(
                      T: [<CanBe #variant>],
                  )*
              T: Debug + Clone + Serialize + PartialEq,
                {
                    match &self {
                        #(
                            #enum_name::#variant(ref x) =>
                                T::[<construct_ #variant:snake:lower>](x.clone(), ix, AOption(ROption::RSome((uuid.unwrap(), name)))),
                        )*
                    }
                }
          }
          impl <T> std::convert::From<(
              // enum name
              AString,
              // field name
              AOption<AString>,
              // ix
              AOption<usize>,
              // uuid
              AOption<Uuid>,
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
                      AString,
                      AOption<AString>,
                      AOption<usize>,
                      AOption<Uuid>,
                      #enum_name,
                  )
              ) -> Self {
                  let (name, field, ix, uuid, children_enum) = tpl;
                  match children_enum {
                      #(
                          #enum_name::#variant(ref x) => WrappedConcept{
                              inner: T::[<construct_ #variant:snake:lower>](x.clone(), ix, AOption(ROption::RSome((uuid.unwrap(), name)))),
                          },
                      )*
                      _ => panic!("_phantom arm should not be activated"),
                  }
              }
          }

        }}))
    }
    fn to_concept_token_stream(&self, enum_name: &Ident) -> Result<TokenStream, AoristError> {
        let variant = &self.variant_idents;
        let py_class_name = format!("{}", enum_name);
        Ok(proc_macro::TokenStream::from(quote! { paste! {
          pub trait [<CanBe #enum_name>]: Debug + Clone + Serialize + PartialEq {
              fn [<construct_ #enum_name:snake:lower>] (
                  obj_ref: AoristRef<#enum_name>,
                  ix: AOption<usize>,
                  id: AOption<(Uuid, AString)>
              ) -> AoristRef<Self>;
          }
          #[cfg(feature = "python")]
          #[pyo3::prelude::pyclass(name=#py_class_name)]
          #[derive(Clone, PartialEq)]
          pub struct [<Py #enum_name>] {
              pub inner: AoristRef<#enum_name>,
          }
          /*#[cfg(feature = "python")]
          impl AoristRef<#enum_name> {
              pub fn py_object(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::PyObject> {
                  Ok(pyo3::PyObject::from(pyo3::PyCell::new(py, [<Py #enum_name>]{
                      inner: self.clone(),
                  })?))
              }
          }*/
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
          impl AoristConceptBase for #enum_name {
              type TChildrenEnum = #enum_name;
              #[cfg(feature = "python")]
              fn py_object(inner: AoristRef<Self>, py: pyo3::Python) -> pyo3::PyResult<pyo3::PyObject> {
                    Ok(pyo3::PyObject::from(pyo3::PyCell::new(py, [<Py #enum_name>]{
                        inner
                    })?))
              }
              fn get_uuid(&self) -> AOption<Uuid> {
                  match &self {
                      #(
                        #enum_name::#variant(x) => x.get_uuid(),
                      )*
                  }
              }
              fn set_uuid(&mut self, uuid: Uuid) {
                  match &self {
                      #(
                        #enum_name::#variant(x) => x.0.write().set_uuid(uuid),
                      )*
                  }
              }
              fn deep_clone(&self) -> Self {
                  match &self {
                      #(
                        #enum_name::#variant(x) => #enum_name::#variant(x.deep_clone()),
                      )*
                  }
              }
              fn get_tag(&self) -> AOption<AString> {
                  match self {
                      #(
                        #enum_name::#variant(x) => x.get_tag(),
                      )*
                  }
              }
              fn compute_uuids(&mut self) {
                  match self {
                      #(
                        #enum_name::#variant(x) => x.compute_uuids(),
                      )*
                  }
              }
              fn get_children(&self) -> AVec<(
                  // enum name
                  AString,
                  // field name
                  AOption<AString>,
                  // ix
                  AOption<usize>,
                  // uuid
                  AOption<Uuid>,
                  Self,
              )> {
                  vec![(
                      stringify!(#enum_name).into(),
                      AOption(ROption::RNone),
                      AOption(ROption::RNone),
                      self.get_uuid(),
                      self.clone(),
                  )].into_iter().collect()
              }
          }
          impl #enum_name {
              fn get_children_uuid(&self) -> AVec<Uuid> {
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
          impl ConceptEnum for [<#enum_name>] {
              fn uuid(&self) -> AOption<Uuid> {
                  match self {
                      #(
                          #enum_name::#variant(x) => x.get_uuid(),
                      )*
                  }
              }
          }
          /*impl AoristRef<#enum_name> {
              pub fn deep_clone(&self) -> Self {
                  AoristRef(abi_stable::std_types::RArc::new(abi_stable::external_types::parking_lot::rw_lock::RRwLock::new(self.0.read().deep_clone())))
              }
          }*/
        }}))
    }
}
