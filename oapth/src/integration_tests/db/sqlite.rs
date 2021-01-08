#[oapth_macros::_dev_tools]
pub(crate) async fn clean_drops_all_objs<B>(
  c: &mut crate::Commands<B,>,
  _: crate::integration_tests::AuxTestParams,
) where
  B: crate::BackEnd
{
  crate::integration_tests::create_foo_table(c, "").await;
  assert_eq!(c.back_end.tables("").await.unwrap().len(), 1);

  c.clean().await.unwrap();

  assert_eq!(c.back_end.tables("").await.unwrap().len(), 0);
}
