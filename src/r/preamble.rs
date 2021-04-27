use crate::code::Preamble;
use extendr_api::prelude::*;
use std::hash::Hash;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct RPreamble {
    pub libraries: Vec<String>,
    pub body: String,
}
impl Preamble for RPreamble {}
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
            libraries: libraries.as_list_iter().unwrap().map(|x| x.as_str().unwrap().to_string()).collect(),
            body: body_no_imports.as_str().unwrap().to_string(),
        }
    }
}
