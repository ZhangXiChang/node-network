use anyhow::{anyhow, Result};
use x509_parser::{
    certificate::X509Certificate,
    der_parser::asn1_rs::FromDer,
    extensions::{GeneralName, ParsedExtension},
};

pub trait CertDer {
    fn get_dns_name(&self) -> Result<String>;
}

impl CertDer for Vec<u8> {
    fn get_dns_name(&self) -> Result<String> {
        let (_, x509certificate) = X509Certificate::from_der(self)?;
        x509certificate
            .tbs_certificate
            .extensions()
            .iter()
            .find_map(|x509extension| {
                if let ParsedExtension::SubjectAlternativeName(subject_alternative_name) =
                    x509extension.parsed_extension()
                {
                    return subject_alternative_name.general_names.iter().find_map(
                        |general_name| {
                            if let GeneralName::DNSName(dns_name) = general_name {
                                return Some(dns_name.to_string());
                            }
                            None
                        },
                    );
                }
                None
            })
            .ok_or(anyhow!("没有找到DNSName"))
    }
}
