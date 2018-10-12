pub enum LogLevel {
    Debug, Info, Error, Warning, Fatal
}

impl LogLevel {
    pub fn to_string (&self) -> &str { 
        match self {
            Debug => "Debug",
            Info => "Info",
            Error => "Error",
            Warning => "Warning",
            Fatal => "Fatal"
        }
    }
}

#[test]
fn test_LogLevel() {
    let message = LogLevel::Error;
    let message_str = message.to_string();
    assert!(message_str == "Error");
}