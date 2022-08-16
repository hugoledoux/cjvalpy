use pyo3::exceptions;
use pyo3::prelude::*;

extern crate cjval;

#[pymodule]
fn cjvalpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CJValidator>()?;
    Ok(())
}

#[pyclass(unsendable)]
pub struct CJValidator {
    val: cjval::CJValidator,
    serrors: String,
    isvalid: bool,
}

#[pymethods]
impl CJValidator {
    #[new]
    fn new(j: Vec<String>) -> PyResult<Self> {
        let mut isvalid = true;
        let re = cjval::CJValidator::from_str(&j[0]);
        if re.is_err() {
            let s = format!("Invalid JSON file: {:?}", re.as_ref().err().unwrap());
            return Err(PyErr::new::<exceptions::PyIOError, _>(s));
        } else {
            let mut val = re.unwrap();
            let mut s = String::from("=== CityJSON syntax ===\n");
            s.push_str("CityJSON schemas used: v");
            s.push_str(&val.get_cityjson_schema_version());
            s.push_str(" (builtin)\n\n");
            let rev = val.validate_schema();
            print_errors(&mut s, &rev);
            if rev.is_empty() == true {
                s.push_str("=== Extensions schemas ===\n");
                for i in 1..j.len() {
                    let re = val.add_one_extension_from_str(&i.to_string(), &j[i]);
                    match re {
                        Ok(()) => {
                            let a = format!("{}. ok", i);
                            s.push_str(&a);
                        }
                        Err(e) => {
                            let a = format!("{}. ERROR\n({})", i, e);
                            s.push_str(&a);
                            isvalid = false;
                        }
                    }
                }
            } else {
                isvalid = false;
            }

            Ok(CJValidator {
                val: val,
                serrors: s,
                isvalid: isvalid,
            })
        }
    }

    fn get_report(&mut self) -> PyResult<String> {
        return Ok(self.serrors.clone());
    }

    fn validate(&mut self) -> PyResult<bool> {
        if self.isvalid == false {
            print_summary(&mut self.serrors, -1);
            return Ok(self.isvalid);
        }
        //-- validate Extensions, if any
        if self.val.get_input_cityjson_version() == 10 {
            self.serrors
                .push_str("(validation of Extensions is not supported in v1.0, upgrade to v1.1)");
            print_summary(&mut self.serrors, -1);
            return Ok(self.isvalid);
        }
        let mut rev = self.val.validate_extensions();
        print_errors(&mut self.serrors, &rev);
        if rev.is_empty() == false {
            print_summary(&mut self.serrors, -1);
            return Ok(self.isvalid);
        }

        //-- parent_children_consistency
        self.serrors
            .push_str("=== parent_children_consistency ===\n");
        rev = self.val.parent_children_consistency();
        print_errors(&mut self.serrors, &rev);
        if rev.is_empty() == false {
            self.isvalid = false;
        }
        //-- wrong_vertex_index
        self.serrors.push_str("=== wrong_vertex_index ===\n");
        rev = self.val.wrong_vertex_index();
        print_errors(&mut self.serrors, &rev);
        if rev.is_empty() == false {
            self.isvalid = false;
        }
        //-- semantics_arrays
        self.serrors.push_str("=== semantics_arrays ===\n");
        rev = self.val.semantics_arrays();
        print_errors(&mut self.serrors, &rev);
        if rev.is_empty() == false {
            self.isvalid = false;
        }

        if self.isvalid == false {
            print_summary(&mut self.serrors, -1);
            return Ok(self.isvalid);
        }

        //-- WARNINGS
        let mut bwarns = false;

        //-- duplicate_vertices
        self.serrors
            .push_str("=== duplicate_vertices (warnings) ===\n");
        rev = self.val.duplicate_vertices();
        print_errors(&mut self.serrors, &rev);
        if rev.is_empty() == false {
            bwarns = true;
        }

        //-- extra_root_properties
        self.serrors
            .push_str("=== extra_root_properties (warnings) ===\n");
        rev = self.val.extra_root_properties();
        print_errors(&mut self.serrors, &rev);
        if rev.is_empty() == false {
            bwarns = true;
        }

        //-- unused_vertices
        self.serrors
            .push_str("=== unused_vertices (warnings) ===\n");
        rev = self.val.unused_vertices();
        print_errors(&mut self.serrors, &rev);
        if rev.is_empty() == false {
            bwarns = true;
        }

        //-- bye-bye
        if bwarns == false {
            print_summary(&mut self.serrors, 1);
        } else {
            print_summary(&mut self.serrors, 0);
        }
        return Ok(self.isvalid);
    }
}

fn print_errors(s: &mut String, lserrs: &Vec<String>) {
    if lserrs.is_empty() {
        s.push_str("ok\n");
    } else {
        for (i, e) in lserrs.iter().enumerate() {
            let a = format!("  {}. {}\n", i + 1, e);
            s.push_str(&a);
        }
    }
}

fn print_summary(s: &mut String, finalresult: i32) {
    s.push_str("\n\n");
    s.push_str("============ SUMMARY ============\n");
    if finalresult == -1 {
        s.push_str("❌ File is invalid\n");
    } else if finalresult == 0 {
        s.push_str("⚠️  File is valid but has warnings\n");
    } else {
        s.push_str("✅ File is valid\n");
    }
    s.push_str("=================================");
}
