
// yes ik, i want to stay as std-only as possible I wont be using cfg_if // might make my own

pub mod runner;

pub mod environment;
pub mod expr;
pub mod literals;
pub mod digit;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod functions;

#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod interpreter;

#[cfg(all(feature = "compiler", not(feature = "interpreter")))]
pub mod compiler;
