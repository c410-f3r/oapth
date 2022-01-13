diesel::table! {
  generic_table (generic_column) {
    generic_column -> Text,
  }
}

#[derive(Debug, PartialEq, diesel::QueryableByName)]
#[diesel(table_name = generic_table)]
pub(crate) struct GenericTable {
  pub(crate) generic_column: String,
}
