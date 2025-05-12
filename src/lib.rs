use edge_t2s::tts::{
    TTS,
    edge_tts::{EdgeTTS, edge_tts_config::EdgeTTSConfig},
};
use pyo3::prelude::*;

#[pyfunction]
async fn t2s(text: String, narrator_name: String, rate: i16, pitch: i16) -> PyResult<Vec<u8>> {
    let tts = EdgeTTS::new(EdgeTTSConfig::new(
        narrator_name,
        Some(rate),
        Some(pitch),
        None,
    ));

    let mut client = tts.connect().await.map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("TTS connect failed: {e}"))
    })?;

    let audio = tts.send_content(&mut client, text).await.map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Send content failed: {e}"))
    })?;

    Ok(audio)
}

#[pymodule]
fn edge_tts_py(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(t2s, m)?)?;
    Ok(())
}
