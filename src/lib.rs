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
//! ![example](https://raw.githubusercontent.com/Rinrin0413/rogger/master/assets/images/ex_v0_1_1.png)
//! 
//! # Features
//! 
//! - `jst`: Use JST for timestamps.
//! 
//! - `utc_jst`: Provide additional logging macros as `*_jst!()` in module `jst`.
//! 
//! [![MIT](https://img.shields.io/github/license/Rinrin0413/rogger?color=%23A11D32&style=for-the-badge)](./LICENSE)

/// Info log
/// 
/// # Example
/// 
/// ```
/// let ver = "0.1.0";
/// info!("Version: {}", ver); //< 2022-12-18T07:02:03(UTC) INFO crate_name::module_path: Version: 0.1.0
/// ```
#[cfg(not(feature = "jst"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            " INFO".green(),
            format!("{}:", module_path!()).bright_black(),
            format_args!($($arg)*)
        );
    }
}

/// Info log
/// 
/// # Example
/// 
/// ```
/// let ver = "0.1.0";
/// info!("Version: {}", ver); //< 2022-12-18T07:02:03(JST) INFO crate_name::module_path: Version: 0.1.0
/// ```
#[cfg(feature = "jst")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(JST)".bright_black(),
            " INFO".green(),
            format!("{}:", module_path!()).bright_black(),
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
/// warn!("Your device \"{}\" is deprecated", warn); //< 2022-12-18T07:02:03(UTC) WARN crate_name::module_path: Your device "Foo PC" is deprecated
/// ```
#[cfg(not(feature = "jst"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            " WARN".bright_yellow(),
            format!("{}:", module_path!()).bright_black(),
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
/// warn!("Your device \"{}\" is deprecated", warn); //< 2022-12-18T07:02:03(JST) WARN crate_name::module_path: Your device "Foo PC" is deprecated
/// ```
#[cfg(feature = "jst")]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(JST)".bright_black(),
            " WARN".bright_yellow(),
            format!("{}:", module_path!()).bright_black(),
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
/// error!("Fatal: {}", err); //< 2022-12-18T07:02:03(UTC) ERROR crate_name::module_path: Fatal: Operating System is not found
/// ```
#[cfg(not(feature = "jst"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "ERROR".bright_red(),
            format!("{}:", module_path!()).bright_black(),
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
/// error!("Fatal: {}", err); //< 2022-12-18T07:02:03(JST) ERROR crate_name::module_path: Fatal: Operating System is not found
/// ```
#[cfg(feature = "jst")]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(JST)".bright_black(),
            "ERROR".bright_red(),
            format!("{}:", module_path!()).bright_black(),
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
/// debug!("Buffer: 0x{:x}", buf); //< 2022-12-18T07:02:03(UTC) DEBUG crate_name::module_path: Buffer: 0x12345678
/// ```
#[cfg(not(feature = "jst"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "DEBUG".bright_cyan(),
            format!("{}:", module_path!()).bright_black(),
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
/// debug!("Buffer: 0x{:x}", buf); //< 2022-12-18T07:02:03(JST) DEBUG crate_name::module_path: Buffer: 0x12345678
/// ```
#[cfg(feature = "jst")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(JST)".bright_black(),
            "DEBUG".bright_cyan(),
            format!("{}:", module_path!()).bright_black(),
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
/// trace!("Age: {}", age); //< 2022-12-18T07:02:03(UTC) TRACE crate_name::module_path: Age: 17
/// ```
#[cfg(not(feature = "jst"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            "TRACE".bright_blue(),
            format!("{}:", module_path!()).bright_black(),
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
/// trace!("Age: {}", age); //< 2022-12-18T07:02:03(JST) TRACE crate_name::module_path: Age: 17
/// ```
#[cfg(feature = "jst")]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        println!("{}{} {} {} {}", 
            (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(JST)".bright_black(),
            "TRACE".bright_blue(),
            format!("{}:", module_path!()).bright_black(),
            format_args!($($arg)*)
        );
    }
}

/// Flag
/// 
/// # Example
/// 
/// ```
/// flag!(); //< 2022-12-18T07:02:03(UTC) FLAG crate_name::module_path: [src/main.rs:21]
/// flag!("i wake up!"); //< 2022-12-18T07:02:03(UTC) FLAG crate_name::module_path: [src/main.rs:21] i wake up!
/// ```
#[cfg(not(feature = "jst"))]
#[macro_export]
macro_rules! flag {
    () => {
        println!("{}{} {} {} {}",
            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            " FLAG".bright_magenta(),
            format!("{}:", module_path!()).bright_black(),
            format_args!("[{}:{}]", file!(), line!())
        );
    };
    ($($arg:tt)*) => {
        println!("{}{} {} {} {} {}",
            chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(UTC)".bright_black(),
            " FLAG".bright_magenta(),
            format!("{}:", module_path!()).bright_black(),
            format_args!("[{}:{}]", file!(), line!()),
            format_args!($($arg)*)
        );
    };
}

/// Flag
/// 
/// # Example
/// 
/// ```
/// flag!(); //< 2022-12-18T07:02:03(JST) FLAG crate_name::module_path: [src/main.rs:21]
/// flag!("i wake up!"); //< 2022-12-18T07:02:03(JST) FLAG crate_name::module_path: [src/main.rs:21] i wake up!
/// ```
#[cfg(feature = "jst")]
#[macro_export]
macro_rules! flag {
    () => {
        println!("{}{} {} {} {}",
            (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(JST)".bright_black(),
            " FLAG".bright_magenta(),
            format!("{}:", module_path!()).bright_black(),
            format_args!("[{}:{}]", file!(), line!())
        );
    };
    ($($arg:tt)*) => {
        println!("{}{} {} {} {} {}",
            (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
            "(JST)".bright_black(),
            " FLAG".bright_magenta(),
            format!("{}:", module_path!()).bright_black(),
            format_args!("[{}:{}]", file!(), line!()),
            format_args!($($arg)*)
        );
    };
}


/// Logging macros for JST
#[cfg(feature = "utc_jst")]
pub mod jst {
    /// Info log
    /// 
    /// # Example
    /// 
    /// ```
    /// let ver = "0.1.0";
    /// info_jst!("Version: {}", ver); //< 2022-12-18T07:02:03(JST) INFO crate_name::module_path: Version: 0.1.0
    /// ```
    #[macro_export]
    macro_rules! info_jst {
        ($($arg:tt)*) => {
            println!("{}{} {} {} {}", 
                (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
                "(JST)".bright_black(),
                " INFO".green(),
                format!("{}:", module_path!()).bright_black(),
                format_args!($($arg)*)
            );
        }
    }

    /// Warn log
    /// 
    /// # Example
    /// 
    /// ```
    /// let ver = "0.1.0";
    /// warn_jst!("Version: {}", ver); //< 2022-12-18T07:02:03(JST) WARN crate_name::module_path: Version: 0.1.0
    /// ```
    #[macro_export]
    macro_rules! warn_jst {
        ($($arg:tt)*) => {
            println!("{}{} {} {} {}", 
                (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
                "(JST)".bright_black(),
                " WARN".bright_yellow(),
                format!("{}:", module_path!()).bright_black(),
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
    /// error_jst!("Fatal: {}", err); //< 2022-12-18T07:02:03(JST) ERROR crate_name::module_path: Fatal: Operating System is not found
    /// ```
    #[macro_export]
    macro_rules! error_jst {
        ($($arg:tt)*) => {
            println!("{}{} {} {} {}", 
                (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
                "(JST)".bright_black(),
                "ERROR".bright_red(),
                format!("{}:", module_path!()).bright_black(),
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
    /// debug_jst!("Buffer: 0x{:x}", buf); //< 2022-12-18T07:02:03(JST) DEBUG crate_name::module_path: Buffer: 0x12345678
    /// ```
    #[macro_export]
    macro_rules! debug_jst {
        ($($arg:tt)*) => {
            println!("{}{} {} {} {}", 
                (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
                "(JST)".bright_black(),
                "DEBUG".bright_cyan(),
                format!("{}:", module_path!()).bright_black(),
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
    /// trace_jst!("Age: {}", age); //< 2022-12-18T07:02:03(JST) TRACE crate_name::module_path: Age: 17
    /// ```
    #[macro_export]
    macro_rules! trace_jst {
        ($($arg:tt)*) => {
            println!("{}{} {} {} {}", 
                (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
                "(JST)".bright_black(),
                "TRACE".bright_blue(),
                format!("{}:", module_path!()).bright_black(),
                format_args!($($arg)*)
            );
        }
    }

    /// Flag
    /// 
    /// # Example
    /// 
    /// ```
    /// flag_jst!(); //< 2022-12-18T07:02:03(JST) FLAG crate_name::module_path: [src/main.rs:21]
    /// flag_jst!("i wake up!"); //< 2022-12-18T07:02:03(JST) FLAG crate_name::module_path: [src/main.rs:21] i wake up!
    /// ```
    #[macro_export]
    macro_rules! flag_jst {
        () => {
            println!("{}{} {} {} {}",
                (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
                "(JST)".bright_black(),
                " FLAG".bright_magenta(),
                format!("{}:", module_path!()).bright_black(),
                format_args!("[{}:{}]", file!(), line!())
            );
        };
        ($($arg:tt)*) => {
            println!("{}{} {} {} {} {}",
                (chrono::Utc::now() + chrono::Duration::hours(9)).format("%Y-%m-%dT%H:%M:%S").to_string().bright_black(),
                "(JST)".bright_black(),
                " FLAG".bright_magenta(),
                format!("{}:", module_path!()).bright_black(),
                format_args!("[{}:{}]", file!(), line!()),
                format_args!($($arg)*)
            );
        };
    }
}
