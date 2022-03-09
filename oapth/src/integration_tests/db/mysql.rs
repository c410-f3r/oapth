#[oapth_macros::_dev_tools]
pub(crate) async fn clean_drops_all_objs<B>(
  buffer: &mut String,
  c: &mut crate::Commands<B>,
  _: crate::integration_tests::AuxTestParams,
) where
  B: crate::Backend
{
  crate::integration_tests::create_foo_table(c, "").await;

  assert_eq!(c.backend.tables("").await.unwrap().len(), 1);

  c.clean(buffer).await.unwrap();

  assert_eq!(c.backend.tables("").await.unwrap().len(), 0);
}
