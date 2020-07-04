use ndarray::array;
use ndarray::Array1;
use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*;

#[pyclass]
pub struct Table {
    cols: Vec<Column>,
}

pub enum Column {
    IntColumn(Array1<i32>),
    UIntColumn(Array1<u32>),
    LongColumn(Array1<i64>),
    ULongColumn(Array1<u64>),
    FloatColumn(Array1<f32>),
    StrColumn(Array1<String>),
}

#[pymethods]
impl Table {
    #[new]
    pub fn new() -> Self {
        Table {
            cols: vec![Column::UIntColumn(array![1, 2, 2, 3, 4, 5, 5, 5])],
        }
    }

    pub fn get_col(&mut self, py: Python, i: usize) -> Py<PyArray1<u32>> {
        match &mut self.cols[i] {
            Column::UIntColumn(arr) => arr.clone().into_pyarray(py).to_owned(),
            _ => unimplemented!(),
        }
    }
}
