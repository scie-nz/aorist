body <- "
library('ggplot2')
library('igraph')
c(1)
f <- function(a, b) {
  a + b
}
"

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
print(to.preamble(body))
