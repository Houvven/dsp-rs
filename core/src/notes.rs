enum EqualTemperament {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl EqualTemperament {
    fn get_name(&self) -> &str {
        match self {
            EqualTemperament::C => "C",
            EqualTemperament::CSharp => "C#",
            EqualTemperament::D => "D",
            EqualTemperament::DSharp => "D#",
            EqualTemperament::E => "E",
            EqualTemperament::F => "F",
            EqualTemperament::FSharp => "F#",
            EqualTemperament::G => "G",
            EqualTemperament::GSharp => "G#",
            EqualTemperament::A => "A",
            EqualTemperament::ASharp => "A#",
            EqualTemperament::B => "B",
        }
    }

    fn get_semitone(&self) -> i32 {
        match self {
            EqualTemperament::C => -9,
            EqualTemperament::CSharp => -8,
            EqualTemperament::D => -7,
            EqualTemperament::DSharp => -6,
            EqualTemperament::E => -5,
            EqualTemperament::F => -4,
            EqualTemperament::FSharp => -3,
            EqualTemperament::G => -2,
            EqualTemperament::GSharp => -1,
            EqualTemperament::A => 0,
            EqualTemperament::ASharp => 1,
            EqualTemperament::B => 2,
        }
    }
}
