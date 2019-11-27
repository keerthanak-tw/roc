pub mod can;
pub mod collections;
pub mod graph;
pub mod ident;
pub mod operator;
pub mod parse;
pub mod region;

pub mod string;

pub mod build;
pub mod constrain;
pub mod ena;
pub mod fmt;
pub mod gen;
pub mod infer;
pub mod module;
pub mod pretty_print_types;
pub mod solve;
pub mod subs;
pub mod types;
pub mod unify;

extern crate bumpalo;
extern crate im_rc;
extern crate inkwell;
extern crate wyhash;

#[macro_use]
extern crate log;
