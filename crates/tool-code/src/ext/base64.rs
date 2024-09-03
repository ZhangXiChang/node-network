use base64::{engine::general_purpose, Engine};

pub trait Base64 {
    fn to_base64(&self) -> String;
}

impl Base64 for Vec<u8> {
    fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(self)
    }
}
