
// yes ik, i want to stay as std-only as possible I wont be using cfg_if // might make my own

#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod environment;
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod expr;
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod interpreter;
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod literals;
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod digit;
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod parser;
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod scanner;
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod stmt;
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod functions;
#[cfg(all(feature = "interpreter", not(feature = "compiler")))]
pub mod runner;
