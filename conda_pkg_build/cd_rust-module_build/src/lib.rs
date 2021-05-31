use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    // There is an improved version later on this post
    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }

    Ok(total)
}

#[pymodule]
fn myrustlib(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(count_doubles))?;
    Ok(())
}
