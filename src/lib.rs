//! A demo python module in Rust that can extract words
//! from a comma separated string of words that ends with the given suffix

use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_rust, m)?)?;
    m.add_function(wrap_pyfunction!(find_words, m)?)?;
    Ok(())
}

#[pyfunction]
fn hello_from_rust() -> String {
    "Hello from Rust!".to_string()
}

#[pyfunction]
fn find_words(src: &str, suffix: &str) -> PyResult<Vec<String>> {
    let result = find_words_impl(src, suffix);
    Ok(result)
}

fn find_words_impl(src: &str, suffix: &str) -> Vec<String> {
    let mut v = vec![];
    let filtered = src.split(",").filter_map(|s| {
        let trimmed = s.trim();
        if trimmed.ends_with(&suffix) {
            Some(trimmed.to_owned())
        } else {
            None
        }
    });
    for s in filtered {
        v.push(s);
    }
    v
}

#[cfg(test)]
mod tests {
    use crate::find_words_impl;

    #[test]
    fn it_works() {
        let result = find_words_impl("Baz,Jazz,Mash,Splash,Squash", "sh");
        println!("{:#?}", result);
    }
}
