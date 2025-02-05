use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::detector::PitchDetector;
use crate::detector_fast_yin::FastYin;

pub mod detector;
pub mod detector_fast_yin;
mod notes;


static CACHE_CONTAINER: Lazy<Mutex<HashMap<String, Arc<Mutex<dyn PitchDetector>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn _get_detector(sample_rate: f32, buffer_size: usize) -> Arc<Mutex<dyn PitchDetector>> {
    let key = format!("{}_{}", sample_rate, buffer_size);
    let mut cache = CACHE_CONTAINER.lock().unwrap();

    if let Some(detector) = cache.get(&key) {
        return detector.clone();
    }

    let detector = Arc::new(Mutex::new(FastYin::new(sample_rate, buffer_size)));
    cache.insert(key, detector.clone());
    detector
}