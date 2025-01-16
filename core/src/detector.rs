pub trait PitchDetector {
    fn get_pitch(&mut self, audio_buffer: &[f32]) -> PitchDetectionResult;
}

#[derive(Default, Clone)]
pub struct PitchDetectionResult {
    pub frequency: f32,
    pub probability: f32,
    pub pitched: bool,
}
