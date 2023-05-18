use errors::Status;

pub mod update;
pub mod send_barcode;
pub mod process_barcode;
pub mod looper;
pub mod errors;

static mut ERROR_STATUS : Status = Status::Ok;