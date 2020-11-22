#[cfg(feature = "dev-tools")]
pub async fn _clean_drops_all_objs<B>(
  c: &mut crate::Commands<B>,
  aux: crate::integration_tests::AuxTestParams,
) where
  B: crate::BackEnd,
{
  crate::integration_tests::_create_foo_table(c, aux.default_schema_prefix).await;
  assert_eq!(c.back_end.all_tables(aux.default_schema).await.unwrap().len(), 1);

  c.clean().await.unwrap();

  assert_eq!(c.back_end.all_tables(aux.default_schema).await.unwrap().len(), 0);
}
