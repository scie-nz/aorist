use crate::code::Preamble;
use crate::r::r_import::RImport;
use extendr_api::prelude::*;
use std::hash::Hash;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct RPreamble {
    pub libraries: Vec<RImport>,
    pub body: String,
}
impl Preamble for RPreamble {
    type ImportType = RImport;
    fn get_imports(&self) -> Vec<Self::ImportType> {
        self.libraries.clone()
    }
}
impl<'a> RPreamble {
    // Assumes R has already been started
    pub fn new(body: String) -> RPreamble {
        eval_string(
            r#"
            to.preamble <- function(body) {
                x <- as.list(parse(text=body))
                is.library <- sapply(x, function(y) {
                    if (class(y) == "call") {
                      return(y[[1]] == "library")
                    }
                    return(FALSE)
                })
                call.idx <- which(is.library)
                calls <- x[call.idx]
                not.calls <- x[which(!is.library)]

                body <- paste(sapply(
                       not.calls,
                       function(x) paste(deparse(x), collapse="\n")
                ), collapse="\n\n")

                libraries <- sapply(calls, function(x) x[[2]])
                list(body=body, libraries=libraries)
            }
        "#,
        )
        .unwrap();

        let res = call!("to.preamble", body).unwrap();
        let body_no_imports = res.index(1).unwrap();
        let libraries = res.index(2).unwrap();
        Self {
            libraries: libraries
                .as_string_vector()
                .unwrap()
                .into_iter()
                .map(|x| RImport::new(x))
                .collect(),
            body: body_no_imports.as_str().unwrap().to_string(),
        }
    }
    pub fn get_body(&self) -> String {
        self.body.clone()
    }
}

#[allow(unused_imports)]
mod r_test_preamble {
    use crate::r::preamble::RPreamble;
    use extendr_api::prelude::*;
    #[test]
    fn test_basic_preamble() {
        test! {
            let body = r#"
            library('ggplot2')
            library('igraph')
            c(1)
            f <- function(a, b) {
              a + b
            }
            "#;
            let preamble = RPreamble::new(body.to_string());
            assert_eq!(preamble.libraries.get(0).unwrap(), "ggplot2");
            assert_eq!(preamble.libraries.get(1).unwrap(), "igraph");
            assert_eq!(preamble.body, r#"c(1)

f <- function(a, b) {
    a + b
}"#);
        }
    }
}
