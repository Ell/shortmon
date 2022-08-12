#[derive(Debug, Clone)]
pub struct MonitorError(pub &'static str);

impl std::fmt::Display for MonitorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Monitor Error: {}", self.0)
    }
}

impl std::error::Error for MonitorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
