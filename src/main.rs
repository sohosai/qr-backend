use anyhow::{Context, Result};
use std::net::SocketAddr;
use structopt::StructOpt;
use tokio::runtime;
use warp::Filter;

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
    // /pingにアクセスするとpingが返るだけの素朴なダミー値を入れている
    runtime.block_on(warp::serve(warp::path("ping").map(|| String::from("ping"))).run(opt.bind));

    Ok(())
}
