use std::sync::Arc;

#[derive(Clone)]
pub struct CertKey {
    pub cert_der: Arc<Vec<u8>>,
    pub key_der: Arc<Vec<u8>>,
}
