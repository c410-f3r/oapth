/// Forms all fields of a table.
pub trait TableFields {
  /// See [crate::Error].
  type Error: From<crate::Error>;
  /// Iterator of fields.
  type FieldNames: Iterator<Item = &'static str>;

  /// Yields all table field names.
  fn field_names(&self) -> Self::FieldNames;

  /// Writes the table instance values for INSERT statements.
  fn write_insert_values(&self, buffer_cmd: &mut String) -> Result<(), Self::Error>;

  /// Writes the table instance values for UPDATE statements.
  fn write_update_values(&self, buffer_cmd: &mut String) -> Result<(), Self::Error>;
}
