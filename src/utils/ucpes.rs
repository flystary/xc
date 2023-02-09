extern crate colored;
extern crate tabled;
// use colored::*;
use tabled::{Alignment, Format, Full, Head, Indent, Modify, Row};
use tabled::{Style, Table};

pub type Ucpes = Vec<super::ucpe::Ucpe>;

pub trait Dis {
    // fn check(&self) -> Self;
    fn display(&self);
}

impl Dis for Ucpes {
    fn display(&self) {
        let table = Table::new(self)
            .with(Style::NO_BORDER)
            .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
            .with(Modify::new(Head).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(1..)).with(Alignment::left()))
            .with(Modify::new(Row(0..1)).with(Format(|s| s.to_uppercase())))
            .with(Modify::new(Row(1..)).with(Format(|s| s.to_string())));

        println!("{}", table);
    }
}
