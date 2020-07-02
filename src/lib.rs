use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn tanuki(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_class::<MyClass>()?;

    Ok(())
}

use pyo3::types::PyType;

// it works even if the item is not documented:

#[pyclass]
#[text_signature = "(c, d, /)"]
struct MyClass {
    c: i32,
    d: String,
}

#[pymethods]
impl MyClass {
    // the signature for the constructor is attached
    // to the struct definition instead.
    #[new]
    fn new(c: i32, d: &str) -> Self {
        Self {
            c,
            d: d.to_string(),
        }
    }
    // the self argument should be written $self
    #[text_signature = "($self, e, f)"]
    fn my_method(&self, e: i32, f: i32) -> i32 {
        println!("{}", self.d);
        self.c + e + f
    }
    #[classmethod]
    #[text_signature = "(cls, e, f)"]
    fn my_class_method(cls: &PyType, e: i32, f: i32) -> i32 {
        e + f
    }
    #[staticmethod]
    #[text_signature = "(e, f)"]
    fn my_static_method(e: i32, f: i32) -> i32 {
        e + f
    }
}
