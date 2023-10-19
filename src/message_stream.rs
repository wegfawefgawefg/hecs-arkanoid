pub type ExpiringMessages = Vec<ExpiringMessage>;

#[derive(Clone)]
pub struct ExpiringMessage {
    pub text: String,
    pub lifetime: u32,
}
