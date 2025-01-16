pub trait PitchDetector {
    fn get_pitch(&mut self, audio_buffer: &[f32]) -> PitchDetectionResult;
}

pub struct PitchDetectionResult {
    pub frequency: f32,
    pub probability: f32,
    pub pitched: bool,
}

impl Default for PitchDetectionResult {
    fn default() -> Self {
        PitchDetectionResult {
            frequency: -1.0,
            probability: -1.0,
            pitched: false,
        }
    }
}
