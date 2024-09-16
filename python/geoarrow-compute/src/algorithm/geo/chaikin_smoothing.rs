use crate::ffi::from_python::AnyGeometryInput;
use crate::util::{return_chunked_geometry_array, return_geometry_array};
use geoarrow::algorithm::geo::ChaikinSmoothing;
use pyo3::prelude::*;
use pyo3_geoarrow::PyGeoArrowResult;

#[pyfunction]
pub fn chaikin_smoothing(
    py: Python,
    input: AnyGeometryInput,
    n_iterations: u32,
) -> PyGeoArrowResult<PyObject> {
    match input {
        AnyGeometryInput::Array(arr) => {
            let out = arr.as_ref().chaikin_smoothing(n_iterations)?;
            return_geometry_array(py, out)
        }
        AnyGeometryInput::Chunked(arr) => {
            let out = arr.as_ref().chaikin_smoothing(n_iterations)?;
            return_chunked_geometry_array(py, out)
        }
    }
}