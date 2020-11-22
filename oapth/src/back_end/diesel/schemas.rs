diesel::table! {
  all_tables (table_name) {
    table_name -> Text,
  }
}

#[derive(Debug, PartialEq, diesel::QueryableByName)]
#[table_name = "all_tables"]
pub struct AllTables {
  pub table_name: String,
}
