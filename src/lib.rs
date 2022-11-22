use indexmap::IndexMap;

use pyo3::prelude::*;
use std::fmt::Write;

extern crate cjval;

#[pymodule]
fn cjvalpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CJValidator>()?;
    Ok(())
}

#[pyclass(unsendable)]
pub struct CJValidator {
    val: cjval::CJValidator,
    valsumm: IndexMap<String, cjval::ValSummary>,
}

#[pymethods]
impl CJValidator {
    #[new]
    fn new(js: Vec<String>) -> PyResult<Self> {
        let mut v: cjval::CJValidator;
        if js.is_empty() {
            v = cjval::CJValidator::from_str(&"{}".to_string());
        } else {
            v = cjval::CJValidator::from_str(&js[0]);
        }
        for i in 1..js.len() {
            let _re = v.add_one_extension_from_str(&js[i]);
        }
        let tmp: IndexMap<String, cjval::ValSummary> = IndexMap::new();
        Ok(CJValidator {
            val: v,
            valsumm: tmp,
        })
    }

    fn get_report(&self) -> PyResult<String> {
        let mut s = String::new();
        let mut has_errors = false;
        let mut has_warnings = false;
        for (criterion, summ) in self.valsumm.iter() {
            write!(&mut s, "=== {} ===\n", criterion).expect("Problem writing String");
            write!(&mut s, "{}\n", summ).expect("Problem writing String");
            if summ.has_errors() == true {
                if summ.is_warning() == true {
                    has_warnings = true;
                } else {
                    has_errors = true;
                }
            }
        }
        write!(&mut s, "\n").expect("Problem writing String");
        write!(&mut s, "============= SUMMARY =============\n").expect("Problem writing String");
        if has_errors == false && has_warnings == false {
            write!(&mut s, "✅ File is valid\n").expect("Problem writing String");
        } else if has_errors == false && has_warnings == true {
            write!(&mut s, "🟡  File is valid but has warnings\n").expect("Problem writing String");
        } else {
            write!(&mut s, "❌ File is invalid\n").expect("Problem writing String");
        }
        write!(&mut s, "===================================\n").expect("Problem writing String");
        Ok(s)
    }

    fn validate(&mut self) -> PyResult<bool> {
        self.valsumm = self.val.validate();
        if self.valsumm["json_syntax"].has_errors() {
            return Ok(false);
        }
        if self.valsumm["schema"].has_errors() {
            return Ok(false);
        }
        if self.valsumm["extensions"].has_errors() {
            return Ok(false);
        }
        if self.valsumm["parents_children_consistency"].has_errors() {
            return Ok(false);
        }
        if self.valsumm["wrong_vertex_index"].has_errors() {
            return Ok(false);
        }
        if self.valsumm["semantics_arrays"].has_errors() {
            return Ok(false);
        }
        Ok(true)
    }
}
