mod arguments;
mod error;
mod logging;
mod rpc;
mod scanner;
pub use error::{err_msg, Result};
use log::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = arguments::get_opt();
    logging::init(opt.verbose);
    let client = match rpc::init(opt.host, opt.port, opt.token).await {
        Ok(client) => Some(client),
        Err(e) => {
            error!("Connect to server failed: {}", e);
            if !opt.dry_run {
                return Ok(());
            }
            warn!("Continue because dry-run mode");
            None
        }
    };

    info!("Scan Mode: {}", opt.scan_mode);
    let list = match opt.scan_mode.as_str() {
        arguments::MODE_SINGLE => scanner::single_file_mode(opt.folder, opt.depth),
        arguments::MODE_FIXED => scanner::fixed_depth_mode(opt.folder, opt.depth),
        arguments::MODE_FILES => scanner::files_folder_mode(opt.folder, opt.depth),
        _ => {
            error!("Unsupported scan mode");
            return Ok(());
        }
    };
    info!("Scan result: {:?}", list);

    client.map(|c| c.report(list));

    if opt.daemon {
        error!("Daemon mod not supportted");
        loop {}
    }
    Ok(())
}
