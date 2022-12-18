//! Logging macros for Rinrin.rs  
//! This crate is VERY optimized for Rinrin.rs
//! so it may not be suitable for other projects.  
//! but usage is very simple and easy.
//! 
//! # Installation
//! 
//! Add the following to your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! rogger = "*"
//! chrono = "0.4"
//! colored = "2"
//! ```
//! 
//! Choose the version of the crate freely
//! but it is safer to do as above.
//! 
//! # Usage
//! 
//! ```rust
//! use rogger::*;
//! use chrono::Utc;
//! use colored::Colorize;
//! 
//! fn main() {
//!     let ver = "0.1.0";
//!     info!("Version: {}", ver);
//! 
//!     let dev = "Foo PC";
//!     warn!("Your device \"{}\" is deprecated", dev);
//! 
//!     let err = "Operating System is not found";
//!     error!("Fatal: {}", err);
//! 
//!     let buf = 0x12345678;
//!     debug!("Buffer: 0x{:x}", buf);
//! 
//!     let age = 17;
//!     trace!("Age: {}", age);
//! 
//!     flag!();
//!     flag!("i wake up!");
//! }
//! ```
//! 
//! ![example](https://raw.githubusercontent.com/Rinrin0413/rogger/master/assets/images/ex.png)
//! 
//! [![MIT](https://img.shields.io/github/license/Rinrin0413/rogger?color=%23A11D32&style=for-the-badge)](./LICENSE)

/// Info log
/// 
/// # Example
/// 
/// ```
/// let ver = "0.1.0";
/// info!("Version: {}", ver); //< 2022-12-18T07:02:03(UTC) INFO Version: 0.1.0
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}{} {} {}", 
            Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "INFO".bright_green(),
            format_args!($($arg)*)
        );
    }
}

/// Warn log
/// 
/// # Example
/// 
/// ```
/// let dev = "Foo PC"
/// warn!("Your device \"{}\" is deprecated", warn); //< 2022-12-18T07:02:03(UTC) WARN Your device "Foo PC" is deprecated
/// ```
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{}{} {} {}", 
            Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "WARN".bright_yellow(),
            format_args!($($arg)*)
        );
    }
}

/// Error log
/// 
/// # Example
/// 
/// ```
/// let err = "Operating System is not found";
/// error!("Fatal: {}", err); //< 2022-12-18T07:02:03(UTC) ERROR Fatal: Operating System is not found
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{}{} {} {}", 
            Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "ERROR".bright_red(),
            format_args!($($arg)*)
        );
    }
}

/// Debug log
/// 
/// # Example
/// 
/// ```
/// let buf = 0x12345678;
/// debug!("Buffer: 0x{:x}", buf); //< 2022-12-18T07:02:03(UTC) DEBUG Buffer: 0x12345678
/// ```
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        println!("{}{} {} {}", 
            Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "DEBUG".bright_cyan(),
            format_args!($($arg)*)
        );
    }
}

/// Trace log
/// 
/// # Example
/// 
/// ```
/// let age = 17;
/// trace!("Age: {}", age); //< 2022-12-18T07:02:03(UTC) TRACE Age: 17
/// ```
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        println!("{}{} {} {}", 
            Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "TRACE".bright_blue(),
            format_args!($($arg)*)
        );
    }
}

/// Flag
/// 
/// # Example
/// 
/// ```
/// flag!(); //< 2022-12-18T07:02:03(UTC) FLAG [src/main.rs:21]
/// flag!("i wake up!"); //< 2022-12-18T07:02:03(UTC) FLAG [src/main.rs:21] i wake up!
/// ```
#[macro_export]
macro_rules! flag {
    () => {
        println!("{}{} {} {}",
            Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "FLAG".bright_magenta(),
            format_args!("[{}:{}]", file!(), line!())
        );
    };
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}",
            Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "FLAG".bright_magenta(),
            format_args!("[{}:{}]", file!(), line!()),
            format_args!($($arg)*)
        );
    };
}
