diesel::table! {
  generic_table (generic_column) {
    generic_column -> Text,
  }
}

#[derive(Debug, PartialEq, diesel::QueryableByName)]
#[table_name = "generic_table"]
pub struct GenericTable {
  pub generic_column: String,
}
