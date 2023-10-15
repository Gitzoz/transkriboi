use core::fmt;

pub enum ModelFormat {
    GGML,
    COREML,
}

impl fmt::Display for ModelFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelFormat::COREML => write!(f, "whisper.cpp-coreml"),
            ModelFormat::GGML => write!(f, "whisper.cpp"),
        }
    }
}

pub enum ModelSize {
    TinyEn,
    Tiny,
    BaseEn,
    Base,
    SmallEn,
    Small,
    Medium,
    MediumEn,
    Large,
}

impl fmt::Display for ModelSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelSize::Base => write!(f, "base"),
            ModelSize::BaseEn => write!(f, "base.en"),
            ModelSize::Tiny => write!(f, "tiny"),
            ModelSize::TinyEn => write!(f, "tiny.en"),
            ModelSize::Small => write!(f, "small"),
            ModelSize::SmallEn => write!(f, "small.en"),
            ModelSize::Medium => write!(f, "medium"),
            ModelSize::MediumEn => write!(f, "medium.en"),
            ModelSize::Large => write!(f, "large"),
        }
    }
}
