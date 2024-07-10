use std::sync::Arc;

use anyhow::{anyhow, Result};
use x509_parser::{
    certificate::X509Certificate,
    der_parser::asn1_rs::FromDer,
    extensions::{GeneralName, ParsedExtension},
};

pub fn x509_dns_name_from_cert_der(cert_der: Arc<Vec<u8>>) -> Result<String> {
    let (_, x509certificate) = X509Certificate::from_der(&cert_der)?;
    for x509extension in x509certificate.tbs_certificate.extensions() {
        if let ParsedExtension::SubjectAlternativeName(subject_alternative_name) =
            x509extension.parsed_extension()
        {
            for general_name in subject_alternative_name.general_names.iter() {
                if let GeneralName::DNSName(dns_name) = general_name {
                    return Ok(dns_name.to_string());
                }
            }
        }
    }
    Err(anyhow!("获取DNSName失败"))
}
