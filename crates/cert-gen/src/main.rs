use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use uuid::Uuid;

#[derive(Parser)]
struct CLIArgs {
    ///证书名称，默认"cert"
    #[arg(long)]
    cert_name: Option<String>,
    ///输出目录，默认"./"
    #[arg(long)]
    out_dir: Option<String>,
}

fn main() -> Result<()> {
    //解析命令行参数
    let cli_args = CLIArgs::parse();
    //生成证书
    let cert_key = rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
    //设置输出目录
    let mut out_dir = PathBuf::from("./");
    if let Some(cli_args_out_dir) = cli_args.out_dir {
        out_dir = PathBuf::from(cli_args_out_dir);
    }
    //输出到文件
    create_dir_all(out_dir.clone())?;
    let mut cert_name = "cert".to_string();
    if let Some(cli_args_cert_name) = cli_args.cert_name {
        cert_name = cli_args_cert_name;
    }
    File::create(out_dir.join(cert_name.clone() + ".cer"))?
        .write_all(&cert_key.cert.der().to_vec())?;
    File::create(out_dir.join(cert_name.clone() + ".key"))?
        .write_all(&cert_key.key_pair.serialize_der())?;
    Ok(())
}
