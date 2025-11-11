use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use pyo3::prelude::*;
use pyo3::types::{PyDict};
use std::{path::Path, time::Duration};

#[pyfunction]
fn rswatcher(py: Python<'_>, path: String) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: Box<dyn Watcher> = Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap());

    // watch some stuff
    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    // just print all events, this blocks forever
    for e in rx {
        println!("{:?}", e);
    }
}

#[pymodule]
fn lp1379(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rswatcher, m)?)?;
    Ok(())
}