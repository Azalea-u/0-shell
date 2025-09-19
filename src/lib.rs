// Library root: re-exports and glue code

pub mod behavior;
pub mod commands;
pub fn run() {
    behavior::shell::run();
}

