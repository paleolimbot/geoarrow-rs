use std::sync::Arc;

use geoarrow::array::metadata::ArrayMetadata;
use geoarrow::array::{
    LineStringArray, MultiLineStringArray, MultiPointArray, MultiPolygonArray, NativeArrayDyn,
    PointArray, PolygonArray,
};
use pyo3::prelude::*;
use pyo3_geoarrow::{PyCoordBuffer, PyGeoArrowResult, PyNativeArray, PyOffsetBuffer};

use crate::crs::CRS;

fn create_array_metadata(crs: Option<CRS>) -> Arc<ArrayMetadata> {
    Arc::new(ArrayMetadata {
        crs: crs.map(|c| c.into_inner()),
        ..Default::default()
    })
}

#[pyfunction]
#[pyo3(signature = (coords, *, crs = None))]
pub fn points(coords: PyCoordBuffer, crs: Option<CRS>) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    match coords {
        PyCoordBuffer::TwoD(coords) => {
            let array = PointArray::new(coords, None, metadata);
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
        PyCoordBuffer::ThreeD(coords) => {
            let array = PointArray::new(coords, None, metadata);
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
    }
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, *, crs = None))]
pub fn linestrings(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    crs: Option<CRS>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    match coords {
        PyCoordBuffer::TwoD(coords) => {
            let array = LineStringArray::new(coords, geom_offsets.into_inner(), None, metadata);
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
        PyCoordBuffer::ThreeD(coords) => {
            let array = LineStringArray::new(coords, geom_offsets.into_inner(), None, metadata);
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
    }
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, ring_offsets, *, crs = None))]
pub fn polygons(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    ring_offsets: PyOffsetBuffer,
    crs: Option<CRS>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    match coords {
        PyCoordBuffer::TwoD(coords) => {
            let array = PolygonArray::new(
                coords,
                geom_offsets.into_inner(),
                ring_offsets.into_inner(),
                None,
                metadata,
            );
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
        PyCoordBuffer::ThreeD(coords) => {
            let array = PolygonArray::new(
                coords,
                geom_offsets.into_inner(),
                ring_offsets.into_inner(),
                None,
                metadata,
            );
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
    }
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, *, crs = None))]
pub fn multipoints(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    crs: Option<CRS>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    match coords {
        PyCoordBuffer::TwoD(coords) => {
            let array = MultiPointArray::new(coords, geom_offsets.into_inner(), None, metadata);
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
        PyCoordBuffer::ThreeD(coords) => {
            let array = MultiPointArray::new(coords, geom_offsets.into_inner(), None, metadata);
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
    }
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, ring_offsets, *, crs = None))]
pub fn multilinestrings(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    ring_offsets: PyOffsetBuffer,
    crs: Option<CRS>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    match coords {
        PyCoordBuffer::TwoD(coords) => {
            let array = MultiLineStringArray::new(
                coords,
                geom_offsets.into_inner(),
                ring_offsets.into_inner(),
                None,
                metadata,
            );
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
        PyCoordBuffer::ThreeD(coords) => {
            let array = MultiLineStringArray::new(
                coords,
                geom_offsets.into_inner(),
                ring_offsets.into_inner(),
                None,
                metadata,
            );
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
    }
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, polygon_offsets, ring_offsets, *, crs = None))]
pub fn multipolygons(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    polygon_offsets: PyOffsetBuffer,
    ring_offsets: PyOffsetBuffer,
    crs: Option<CRS>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    match coords {
        PyCoordBuffer::TwoD(coords) => {
            let array = MultiPolygonArray::new(
                coords,
                geom_offsets.into_inner(),
                polygon_offsets.into_inner(),
                ring_offsets.into_inner(),
                None,
                metadata,
            );
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
        PyCoordBuffer::ThreeD(coords) => {
            let array = MultiPolygonArray::new(
                coords,
                geom_offsets.into_inner(),
                polygon_offsets.into_inner(),
                ring_offsets.into_inner(),
                None,
                metadata,
            );
            Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
        }
    }
}