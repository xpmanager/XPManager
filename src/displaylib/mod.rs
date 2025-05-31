pub mod passwords;
pub mod key;
pub mod encode;
pub mod decode;
pub mod log;

use colored::Colorize;
use crossterm::terminal;
use tabled::{
    settings::{
        location::ByColumnName,
        object::{Columns, Rows}, 
        Alignment, 
        Modify, 
        Style, 
        Width,
        Remove,
    }, 
    Table
};