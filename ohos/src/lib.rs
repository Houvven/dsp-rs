use dsp_core::detector::{PitchDetectionResult, PitchDetector};
use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::ToNapiValue;
use napi_ohos::sys::{napi_env, napi_value};
use napi_ohos::{Env, NapiRaw};
use dsp_core::_get_detector;

#[derive(Debug, Default)]
pub struct NapiPitchDetectionResult {
    frequency: f32,
    probability: f32,
    pitched: bool,
}

impl ToNapiValue for NapiPitchDetectionResult {
    unsafe fn to_napi_value(env: napi_env, val: Self) -> napi_ohos::Result<napi_value> {
        let env: Env = Env::from_raw(env);
        let mut result = env.create_object()?;
        result.set_named_property("frequency", val.frequency)?;
        result.set_named_property("probability", val.probability)?;
        result.set_named_property("pitched", val.pitched)?;
        Ok(result.raw())
    }
}

#[napi(ts_return_type="object")]
pub fn get_pitch_detection_result(
    sample_rate: f64,
    buffer_size: i32,
    audio_buffer: &[f32],
) -> NapiPitchDetectionResult {
    let detector = _get_detector(sample_rate as f32, buffer_size as usize);
    let result: PitchDetectionResult;

    {
        let mut detector = detector.lock().unwrap();
        result = detector.detect(audio_buffer);
    }

    NapiPitchDetectionResult {
        frequency: result.frequency,
        probability: result.probability,
        pitched: result.pitched,
    }
}
