use itertools::Itertools;
use pyo3::{intern, prelude::*};

use crate::app::c_dictionary_record::CDictionaryRecord;
use crate::util::indexed_table::IndexedTableValue;

pub fn module(py: Python) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new_bound(py, "c_const")?;
    module.add_class::<CConst>()?;
    module.add_class::<CConstInt>()?;
    module.add_class::<CConstStr>()?;
    module.add_class::<CConstWStr>()?;
    module.add_class::<CConstChr>()?;
    Ok(module)
}

/// Constant expression.
#[derive(Clone)]
#[pyclass(extends = CDictionaryRecord, frozen, subclass)]
struct CConst {}

#[pymethods]
impl CConst {
    #[new]
    fn new(cd: Py<PyAny>, ixval: IndexedTableValue) -> PyClassInitializer<Self> {
        PyClassInitializer::from(CDictionaryRecord::new(cd, ixval)).add_subclass(CConst {})
    }

    fn get_strings(&self) -> Vec<String> {
        vec![]
    }

    #[getter]
    fn is_int(&self) -> bool {
        false
    }

    #[getter]
    fn is_str(&self) -> bool {
        false
    }

    #[getter]
    fn is_wstr(&self) -> bool {
        false
    }

    #[getter]
    fn is_chr(&self) -> bool {
        false
    }

    #[getter]
    fn is_real(&self) -> bool {
        false
    }

    #[getter]
    fn is_enum(&self) -> bool {
        false
    }

    #[pyo3(name = "__str__")]
    fn str(slf: PyRef<Self>) -> String {
        slf.into_super().into_super().tags()[0].clone()
    }
}

/// Constant integer.
///
/// - tags[1]: string representation of value
/// - tags[2]: ikind
#[derive(Clone)]
#[pyclass(extends = CConst, frozen, subclass)]
struct CConstInt {}

#[pymethods]
impl CConstInt {
    #[new]
    fn new(cd: Py<PyAny>, ixval: IndexedTableValue) -> PyClassInitializer<Self> {
        PyClassInitializer::from(CConst::new(cd, ixval)).add_subclass(CConstInt {})
    }

    #[getter]
    fn intvalue(slf: PyRef<Self>) -> PyResult<isize> {
        Ok(slf.into_super().into_super().into_super().tags()[1].parse()?)
    }

    #[getter]
    fn ikind(slf: PyRef<Self>) -> String {
        slf.into_super().into_super().into_super().tags()[2].clone()
    }

    #[getter]
    fn is_int(&self) -> bool {
        true
    }

    #[pyo3(name = "__str__")]
    fn str(slf: PyRef<Self>) -> PyResult<String> {
        // TODO reference the value directly
        Ok(format!("{}", CConstInt::intvalue(slf)?))
    }
}

/// Constant string
///
/// - args[0]: string index
#[derive(Clone)]
#[pyclass(extends = CConst, frozen, subclass)]
struct CConstStr {}

#[pymethods]
impl CConstStr {
    #[new]
    fn new(cd: Py<PyAny>, ixval: IndexedTableValue) -> PyClassInitializer<Self> {
        PyClassInitializer::from(CConst::new(cd, ixval)).add_subclass(CConstStr {})
    }

    #[getter]
    fn stringvalue(slf: PyRef<Self>, py: Python) -> PyResult<String> {
        let dict_record = slf.into_super().into_super();
        let decls = dict_record.cd();
        let arg0 = dict_record.into_super().args()[0];
        decls
            .call_method1(py, intern!(py, "get_string"), (arg0,))?
            .extract(py)
    }

    fn get_strings(slf: PyRef<Self>, py: Python) -> PyResult<Vec<String>> {
        Ok(vec![CConstStr::stringvalue(slf, py)?])
    }

    #[getter]
    fn is_str(&self) -> bool {
        true
    }

    #[pyo3(name = "__str__")]
    fn str(slf: PyRef<Self>, py: Python) -> PyResult<String> {
        let strg = CConstStr::stringvalue(slf, py)?;
        if strg.len() > 25 {
            Ok(format!("{}-char string", strg.len()))
        } else {
            Ok(format!("str({})", strg))
        }
    }
}

/// Constant wide string (represented as a sequence of int64 integers)
///
/// - tags[1..]: string representation of int64 integers
#[derive(Clone)]
#[pyclass(extends = CConst, frozen, subclass)]
struct CConstWStr {}

#[pymethods]
impl CConstWStr {
    #[new]
    fn new(cd: Py<PyAny>, ixval: IndexedTableValue) -> PyClassInitializer<Self> {
        PyClassInitializer::from(CConst::new(cd, ixval)).add_subclass(CConstWStr {})
    }

    #[getter]
    fn stringvalue(slf: PyRef<Self>) -> PyResult<String> {
        Ok(slf.into_super().into_super().into_super().tags()[1..]
            .iter()
            .join("-"))
    }

    #[getter]
    fn is_wstr(&self) -> bool {
        true
    }

    #[pyo3(name = "__str__")]
    fn str(slf: PyRef<Self>) -> PyResult<String> {
        Ok(format!("wstr({})", CConstWStr::stringvalue(slf)?))
    }
}

/// Constant character.
///
/// - args[0]: char code
#[derive(Clone)]
#[pyclass(extends = CConst, frozen, subclass)]
struct CConstChr {}

#[pymethods]
impl CConstChr {
    #[new]
    fn new(cd: Py<PyAny>, ixval: IndexedTableValue) -> PyClassInitializer<Self> {
        PyClassInitializer::from(CConst::new(cd, ixval)).add_subclass(CConstChr {})
    }

    #[getter]
    fn chrvalue(slf: PyRef<Self>) -> PyResult<isize> {
        Ok(slf.into_super().into_super().into_super().args()[0])
    }

    #[getter]
    fn is_char(&self) -> bool {
        true
    }

    #[pyo3(name = "__str__")]
    fn str(slf: PyRef<Self>) -> PyResult<String> {
        Ok(format!("wstr({})", CConstChr::chrvalue(slf)?))
    }
}
