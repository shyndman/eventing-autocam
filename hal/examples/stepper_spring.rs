use aa_foundation::prelude::*;
use aa_hal::pantilt::PanTiltController;
use anyhow::Result;

fn main() {
    aa_foundation::tracing::setup_dev_tracing_subscriber();
    if let Err(err) = run() {
        error!("{:?}", err);
    }
}

fn run() -> Result<()> {
    let pantilt = PanTiltController::init_system_controller()?;
    pantilt.update_target(200.0)?;

    pantilt.join()
}
