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

pub fn ausnahme() -> Error {
    Error {
        message: "@C03Ausnahme".to_string(),
        status: Status::Warn,
        error_type: Type::Ausnahme,
    }
}

pub fn zu_kurz() -> Error {
    Error {
        message: "@C88Zu kurz".to_string(),
        status: Status::Error,
        error_type: Type::ZuKurz,
    }
}

pub fn dhl_leitcode() -> Error {
    Error {
        message: "@C88DHL Leitcode".to_string(),
        status: Status::Error,
        error_type: Type::DhlLeitcode,
    }
}

pub fn bereits_gesendet() -> Error {
    Error {
        message: "@C88Bereits gesendet".to_string(),
        status: Status::Error,
        error_type: Type::BereitsGesendet,
    }
}

pub fn ok() -> Error {
    Error {
        message: "OK".to_string(),
        status: Status::Ok,
        error_type: Type::Ok,
    }
}


