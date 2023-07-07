use anyhow::{Context, Result};
use qr_backend::app;
use std::net::SocketAddr;
use structopt::StructOpt;
use tokio::runtime;

#[derive(Debug, StructOpt)]
#[structopt(name = "qr-api-server")]
struct Opt {
    #[structopt(short = "j", long, env = "QR_API_SERVER_THREADS")]
    threads: Option<usize>,
    #[structopt(short, long, env = "QR_API_SERVER_BIND")]
    bind: SocketAddr,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut builder = runtime::Builder::new_multi_thread();
    builder.enable_all();

    if let Some(j) = opt.threads {
        builder.worker_threads(j);
    }

    let runtime = builder
        .build()
        .context("Failed to build the Tokio Runtime")?;

    // 指定したスレッド数でサーバーを実行する
    runtime.block_on(app::app(opt.bind))?;

    Ok(())
}
