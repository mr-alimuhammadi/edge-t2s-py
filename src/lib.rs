use edge_t2s::tts::{
    TTS,
    edge_tts::{EdgeTTS, edge_tts_config::EdgeTTSConfig},
};

use pyo3::{prelude::*, wrap_pyfunction};
use pyo3_async_runtimes::tokio::future_into_py;

#[pyfunction]
fn t2s(
    py: Python,
    text: String,
    narrator_name: String,
    rate: i16,
    pitch: i16,
) -> PyResult<Bound<PyAny>> {
    future_into_py(py, async move {
        let tts = EdgeTTS::new(EdgeTTSConfig::new(
            narrator_name,
            Some(rate),
            Some(pitch),
            None,
        ));
        let mut client = tts.connect().await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("connect failed: {e}"))
        })?;

        let audio = tts.send_content(&mut client, text).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("send_content failed: {e}"))
        })?;

        Ok(audio)
    })
}

#[pymodule]
fn edge_t2s_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(t2s, m)?)?;
    Ok(())
}
