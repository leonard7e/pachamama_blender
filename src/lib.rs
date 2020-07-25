use pyo3::prelude::*;
use pyo3::{
    wrap_pyfunction
};

#[pyfunction]
fn register() {
}

#[pyfunction]
fn unregister() {
}

#[pymodule]
fn blender_test(py: Python, m: &PyModule) -> PyResult<()> {
    // Define bl_info dictionary
    // let dict_bl_info = PyDict::new(py);
    // dict_bl_info.set_item("name", "Pachamama")?;
    // dict_bl_info.set_item("blender", (2,83,3))?;
    // dict_bl_info.set_item("category", "Object")?;
    // m.add("bl_info", dict_bl_info)?;

    m.add_wrapped(wrap_pyfunction!(register))?;
    m.add_wrapped(wrap_pyfunction!(unregister))?;
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
