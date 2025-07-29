use pyo3::prelude::*;


#[pyfunction]
fn imprimir_mensajes() -> PyResult<()> {
    println!("✅ Hola desde Rust");
    Ok(())
}

#[pymodule]
fn rust_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(imprimir_mensajes, m)?)?;
    Ok(())
}
