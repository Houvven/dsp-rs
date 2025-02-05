pub trait PitchDetector: Send + Sync {
    fn detect(&mut self, audio_buffer: &[f32]) -> PitchDetectionResult;
}

#[derive(Default, Clone, Debug)]
pub struct PitchDetectionResult {
    pub frequency: f32,
    pub probability: f32,
    pub pitched: bool,
}
