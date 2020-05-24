use log::error;

fn report(line: usize, place: &str, message: &str) {
    error!("[line {}] Error {}: {}", line, place, message);
}

pub fn error(line: usize, message: &str) {
    report(line, "", message)
}
