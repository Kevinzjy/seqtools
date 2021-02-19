// Utilities
// use std::fmt::Display;

// use chrono::Local;

// // A simple logging function
// pub fn debug<T: Display> (info: T) {
//     _logger("DEBUG", info);
// }

// pub fn info<T: Display> (info: T) {
//     _logger("INFO", info);
// }

// pub fn warn<T: Display> (info: T) {
//     _logger("WARN", info);
// }

// pub fn error<T: Display> (info: T) {
//     _logger("ERROR", info);
//     panic! ("Error occured!");
// }

// fn _logger<T: Display> (level: &str, info: T) {
//     let date = Local::now();
//     eprintln!("[{}] |{:5}| - {}", date.format("%a %Y-%m-%d %H:%M:%S"), level, info);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_logger() {
//         debug(String::from("Debugging"));
//         info("Information");
//         warn("Warning");
//     }

//     #[test]
//     #[should_panic]
//     fn test_error() {
//         error("Test error");
//     }
// }
