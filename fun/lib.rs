use errors::Status;

pub mod update;
pub mod send_barcode;
pub mod process_barcode;
pub mod looper;
pub mod errors;
pub mod username_camelcase;
pub mod sync;

static mut ERROR_STATUS : Status = Status::Ok;