#[cfg(feature = "sm-dev")]
pub(crate) async fn _clean_drops_all_objs<D>(
  (buffer_cmd, _, buffer_idents): (
    &mut String,
    &mut Vec<crate::sm::DbMigration>,
    &mut Vec<crate::Identifier>,
  ),
  c: &mut crate::sm::Commands<D>,
  _: crate::sm::integration_tests::AuxTestParams,
) where
  D: crate::sm::SchemaManagement,
{
  crate::sm::integration_tests::create_foo_table(buffer_cmd, c, "").await;

  c.database.table_names(buffer_cmd, buffer_idents, "").await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  c.clear((buffer_cmd, buffer_idents)).await.unwrap();

  c.database.table_names(buffer_cmd, buffer_idents, "").await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();
}
