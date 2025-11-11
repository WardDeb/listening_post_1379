use pyo3::prelude::*;

#[pymodule]
fn notify(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(keep_cool::parse_cools, m)?)?;
    Ok(())
}