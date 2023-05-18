pub enum Status {
    Warn,
    Error,
    Ok,
}

pub enum Type {
    Ausnahme,
    ZuKurz,
    DhlLeitcode,
    BereitsGesendet,
    Ok,
}

pub struct Error {
    pub message: String,
    pub status: Status,
    pub error_type: Type,
}

// Error Ausnahme
pub fn ausnahme() -> Error {
    Error {
        message: "@C03Ausnahme".to_string(),
        status: Status::Warn,
        error_type: Type::Ausnahme,
    }
}

// // Error Zu kurz
// pub const ZU_KURZ: Error = Error {
//     message: "@C88Zu kurz".to_string(),
//     status: Status::Error,
//     error_type: Type::ZuKurz,
// };

// Error Zu kurz
pub fn zu_kurz() -> Error {
    Error {
        message: "@C88Zu kurz".to_string(),
        status: Status::Error,
        error_type: Type::ZuKurz,
    }
}

// // Error DHL Leitcode
// pub const DHL_LEITCODE: Error = Error {
//     message: "@C88DHL Leitcode".to_string(),
//     status: Status::Error,
//     error_type: Type::DhlLeitcode,
// };

// Error DHL Leitcode
pub fn dhl_leitcode() -> Error {
    Error {
        message: "@C88DHL Leitcode".to_string(),
        status: Status::Error,
        error_type: Type::DhlLeitcode,
    }
}

// // Error Bereits gesendet
// pub const BEREITS_GESENDET: Error = Error {
//     message: "@C88Bereits gesendet".to_string(),
//     status: Status::Error,
//     error_type: Type::BereitsGesendet,
// };

// Error Bereits gesendet
pub fn bereits_gesendet() -> Error {
    Error {
        message: "@C88Bereits gesendet".to_string(),
        status: Status::Error,
        error_type: Type::BereitsGesendet,
    }
}

// // Error OK
// pub const OK: Error = Error {
//     message: "OK".to_string(),
//     status: Status::Ok,
//     error_type: Type::Ok,
// };

// Error OK
pub fn ok() -> Error {
    Error {
        message: "OK".to_string(),
        status: Status::Ok,
        error_type: Type::Ok,
    }
}


