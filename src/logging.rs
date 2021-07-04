//use wasi_common::pipe::WritePipe;

pub fn env_logger_init() {
    env_logger::init();

    log::debug!("Printing debug output.");
}
