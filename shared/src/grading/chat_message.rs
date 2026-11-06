#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChatRoleKind {
    System,
    User,
    Assistant,
}

impl ChatRoleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ChatRoleKind::System => "system",
            ChatRoleKind::User => "user",
            ChatRoleKind::Assistant => "assistant",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: ChatRoleKind,
    pub text: String,
}
