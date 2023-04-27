pub mod functions;
pub mod data;
pub mod toml_loader;

pub mod logger {
    #[allow(unused_macros)]
    macro_rules! log {
        ($msg:expr) => {
            //color the log green
            println!("\x1b[32m[log]: {}\x1b[0m", $msg);
        };
        ($msg:expr, $($arg:tt)*) => {
            //color the log green
            println!("\x1b[32m[log]: {}\x1b[0m", format!($msg, $($arg)*));
        };
    }
    #[allow(unused_macros)]
    macro_rules! error {
        ($msg:expr) => {
            //color the log red
            eprintln!("\x1b[31m[error]: {}\x1b[0m", $msg);
        };
        ($msg:expr, $($arg:tt)*) => {
            //color the log red
            eprintln!("\x1b[31m[error]: {}\x1b[0m", format!($msg, $($arg)*));
        };
    }
    #[allow(unused_macros)]
    macro_rules! debug {
        ($msg:expr) => {
            //color the log blue
            if cfg!(debug_assertions) {
                eprintln!("\x1b[34m[debug]: {}\x1b[0m", $msg);
            }
        };
        ($msg:expr, $($arg:tt)*) => {
            //color the log blue
            if cfg!(debug_assertions) {
                eprintln!("\x1b[34m[debug]: {}\x1b[0m", format!($msg, $($arg)*));
            }
        };
    }
    #[allow(unused_macros)]
    macro_rules! alert {
        ($msg:expr) => {
            //color the log yellow
            eprintln!("\x1b[33m[warn]: {}\x1b[0m", $msg);
        };
        ($msg:expr, $($arg:tt)*) => {
            //color the log yellow
            eprintln!("\x1b[33m[warn]: {}\x1b[0m", format!($msg, $($arg)*));
        };
    }
    #[allow(unused_imports)]
    pub(crate) use log;
    #[allow(unused_imports)]
    pub(crate) use error;
    #[allow(unused_imports)]
    pub(crate) use debug;
    #[allow(unused_imports)]
    pub(crate) use alert;
}

pub trait Event {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()>;
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()>;
    fn status(&self) -> bool;
}