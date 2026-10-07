use crate::AppError;
use crate::course::{CourseOutline, UnitLesson};

/// A set of recordings of the whole course, each made by one speech engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VoiceEngineKind {
    Kokoro,
    MacosSay,
}

impl VoiceEngineKind {
    pub const ALL: [VoiceEngineKind; 2] = [VoiceEngineKind::Kokoro, VoiceEngineKind::MacosSay];

    pub fn code(self) -> &'static str {
        match self {
            VoiceEngineKind::Kokoro => "kokoro",
            VoiceEngineKind::MacosSay => "macos_say",
        }
    }
}

impl TryFrom<&str> for VoiceEngineKind {
    type Error = AppError;

    fn try_from(code: &str) -> Result<VoiceEngineKind, AppError> {
        VoiceEngineKind::ALL
            .into_iter()
            .find(|engine| engine.code() == code)
            .ok_or_else(|| AppError::from(format!("unknown voice engine code; [code={code}]")))
    }
}

/// Where one engine's recording of a clip lives, addressed by the hash of the recording's bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClipAudioEntry {
    pub clip_key: String,
    pub engine: VoiceEngineKind,
    pub audio_sha256: String,
    pub audio_size_bytes: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CourseShardContents {
    pub outline: CourseOutline,
    pub clip_audio: Vec<ClipAudioEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnitShardContents {
    pub lesson: UnitLesson,
    pub clip_audio: Vec<ClipAudioEntry>,
}
