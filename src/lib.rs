use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use pyo3::prelude::*;
use pyo3::types::{PyDict};
use std::{path::Path, time::Duration};

#[pyfunction]
fn rswatcher(py: Python<'_>, path: String, callback: Py<PyAny>) -> PyResult<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: Box<dyn Watcher> = Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap());

    // watch some stuff
    watcher
        .watch(Path::new(&path), RecursiveMode::Recursive)
        .unwrap();

loop {
    py.check_signals()?;
    
    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(Ok(event)) => {
            let dict = PyDict::new(py);
            dict.set_item("kind", format!("{:?}", event.kind))?;
            dict.set_item("paths", 
                event.paths.iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<String>>()
            )?;
            callback.call1(py, (dict,))?;
        }
        Ok(Err(e)) => {
            println!("watch error: {:?}", e);
        }
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            continue;
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            break;
        }
    }
}
    Ok(())
}

#[pymodule]
fn lp1379(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rswatcher, m)?)?;
    Ok(())
}