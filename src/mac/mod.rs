#![macro_use]

pub trait Mac {
  fn table_name(&self) -> &str;
  fn columns_default(&self) -> &[&str];
}

macro_rules! def_mac {
  ($s:ident, $table:tt, $cols:expr) => {
    pub struct $s {}

    impl $s {
      const TABLE: &'static str = $table;
      const COLUMNS_DEFAULT: &'static[&'static str] = $cols;

      fn new() -> $s {
        $s {}
      }
    }

    // it's Self, not self
    impl crate::mac::Mac for $s {
      fn table_name(&self) -> &str {
        Self::TABLE
      }

      fn columns_default(&self) -> &[&str] {
        Self::COLUMNS_DEFAULT
      }
    }
  };
}


mod quick_dev {
  #![warn(unused_imports)]
  use super::*;

  def_mac!(Test, "todo", &["id", "title", "description"]);
}