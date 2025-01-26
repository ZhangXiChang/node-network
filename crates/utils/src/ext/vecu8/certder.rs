use anyhow::Result;
use x509_parser::{
    certificate::X509Certificate,
    der_parser::asn1_rs::FromDer,
    extensions::{GeneralName, ParsedExtension},
};

pub trait CertDer {
    fn get_dns_name(&self) -> Result<Option<String>>;
}

impl CertDer for Vec<u8> {
    fn get_dns_name(&self) -> Result<Option<String>> {
        let (_, x509certificate) = X509Certificate::from_der(self)?;
        for x509extension in x509certificate.tbs_certificate.extensions() {
            if let ParsedExtension::SubjectAlternativeName(subject_alternative_name) =
                x509extension.parsed_extension()
            {
                for general_name in &subject_alternative_name.general_names {
                    if let GeneralName::DNSName(dns_name) = general_name {
                        return Ok(Some(dns_name.to_string()));
                    }
                }
            }
        }
        Ok(None)
    }
}
