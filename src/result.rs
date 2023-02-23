pub enum ErrorKind{
    ErrCWD,
    ErrMissingArg,
    ErrMissingHTMLFile,
    ErrInvalidIPv4,
    ErrInvalidIPv6,
    ErrInvalidIP,
    ErrInvalidPort,
    ErrInvalidRefeshDuration,
    ErrSocketBinding,
    ErrMissingRequestFile,
    ErrUnKnown,
}

pub fn err_msg(err: ErrorKind) -> String {
    match err {
        ErrorKind::ErrCWD => format!(""),
        ErrorKind::ErrMissingArg => format!(""),
        ErrorKind::ErrMissingHTMLFile => format!(""),
        ErrorKind::ErrInvalidIPv4 => format!(""),
        ErrorKind::ErrInvalidIPv6 => format!(""),
        ErrorKind::ErrInvalidIP => format!(""),
        ErrorKind::ErrInvalidPort => format!(""),
        ErrorKind::ErrInvalidRefeshDuration => format!(""),
        ErrorKind::ErrSocketBinding => format!(""),
        ErrorKind::ErrMissingRequestFile => format!(""),
        ErrorKind::ErrUnKnown => format!(""),
    }
}
