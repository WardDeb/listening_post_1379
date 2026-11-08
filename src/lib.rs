use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::Path;
use std::sync::mpsc::{RecvTimeoutError, channel};
use std::time::Duration;

fn to_py_err(e: notify::Error) -> PyErr {
    PyOSError::new_err(e.to_string())
}

fn kind_str(kind: &EventKind) -> &'static str {
    match kind {
        EventKind::Create(_) => "create",
        EventKind::Modify(_) => "modify",
        EventKind::Remove(_) => "remove",
        EventKind::Access(_) => "access",
        EventKind::Any | EventKind::Other => "other",
    }
}

#[pyfunction]
fn rswatcher(py: Python<'_>, path: String, callback: Py<PyAny>) -> PyResult<()> {
    let (tx, mut rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).map_err(to_py_err)?;
    watcher
        .watch(Path::new(&path), RecursiveMode::Recursive)
        .map_err(to_py_err)?;

    loop {
        py.check_signals()?;

        let rx = &mut rx;
        match py.detach(move || rx.recv_timeout(Duration::from_millis(100))) {
            Ok(Ok(event)) => {
                let dict = PyDict::new(py);
                dict.set_item("kind", kind_str(&event.kind))?;
                dict.set_item(
                    "paths",
                    event
                        .paths
                        .iter()
                        .map(|p| p.display().to_string())
                        .collect::<Vec<String>>(),
                )?;
                callback.call1(py, (dict,))?;
            }
            Ok(Err(e)) => return Err(to_py_err(e)),
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
}

#[pymodule]
fn lp1379(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rswatcher, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{AccessKind, CreateKind, ModifyKind, RemoveKind};
    use std::sync::mpsc;
    use std::{fs, thread};

    #[test]
    fn kinds_are_stable() {
        assert_eq!(kind_str(&EventKind::Create(CreateKind::File)), "create");
        assert_eq!(kind_str(&EventKind::Modify(ModifyKind::Any)), "modify");
        assert_eq!(kind_str(&EventKind::Remove(RemoveKind::Folder)), "remove");
        assert_eq!(kind_str(&EventKind::Access(AccessKind::Any)), "access");
        assert_eq!(kind_str(&EventKind::Any), "other");
        assert_eq!(kind_str(&EventKind::Other), "other");
    }

    #[test]
    fn missing_path_raises_oserror() {
        Python::initialize();
        Python::attach(|py| {
            let err = rswatcher(py, "/does/not/exist".into(), py.None()).unwrap_err();
            assert!(err.is_instance_of::<PyOSError>(py));
        });
    }

    #[test]
    fn reports_created_file() {
        let dir = std::env::temp_dir().join(format!("lp1379-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("hello.txt");

        let (done_tx, done_rx) = mpsc::channel();
        let watch_dir = dir.to_string_lossy().into_owned();
        thread::spawn(move || {
            Python::initialize();
            let kind = Python::attach(|py| -> PyResult<String> {
                let module = PyModule::from_code(
                    py,
                    c"class Done(Exception): pass
seen = []
def cb(event):
    if any(p.endswith('hello.txt') for p in event['paths']):
        seen.append(event['kind'])
        raise Done",
                    c"cb.py",
                    c"cb",
                )?;
                let err = rswatcher(py, watch_dir, module.getattr("cb")?.unbind()).unwrap_err();
                assert!(err.is_instance(py, &module.getattr("Done")?));
                module.getattr("seen")?.get_item(0)?.extract()
            });
            done_tx.send(kind).unwrap();
        });

        thread::sleep(Duration::from_millis(500));
        fs::write(&file, "hi").unwrap();

        let kind = done_rx
            .recv_timeout(Duration::from_secs(10))
            .expect("no event received within 10s")
            .unwrap();
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(kind, "create");
    }
}
