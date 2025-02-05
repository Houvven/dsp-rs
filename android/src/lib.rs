use dsp_core::_get_detector;
use dsp_core::detector::PitchDetector;
use jni::objects::JClass;
use jni::sys::jobject;
use jni::JNIEnv;
use jni_fn::jni_fn;

#[jni_fn("com.bw.dsp.PitchDetector")]
pub unsafe fn detect(
    mut env: JNIEnv,
    _class: JClass,
    sample_rate: f64,
    buffer_size: i32,
    audio_buffer: &[f32],
) -> jobject {
    let detector = _get_detector(sample_rate as f32, buffer_size as usize);
    let mut detector = detector.lock().unwrap();
    let result = detector.detect(audio_buffer);

    let result_jclass = env.find_class("com/bw/dsp/PitchDetectedResult");

    env.new_object(
        result_jclass.unwrap(),
        "(FFZ)V",
        &[
            result.frequency.into(),
            result.probability.into(),
            result.pitched.into(),
        ],
    )
    .unwrap()
    .as_raw()
}
