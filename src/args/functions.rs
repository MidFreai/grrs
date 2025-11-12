use tabled::{Table, 
    settings::{Remove, Style, object::{Columns, Rows}}
};

use super::cli::Line;

pub fn new_table(file_lines:&Vec<Line>) -> Table{
    let mut table = Table::new(file_lines);
    table.with(Remove::row(Rows::first()));
    table.with(Remove::column(Columns::one(1)));
    table.with(Style::modern_rounded());

    table
}