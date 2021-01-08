use crate::{
  integration_tests::{_migrate_doc_test, AuxTestParams},
  BackEnd, Commands,
};

pub(crate) async fn all_tables_returns_the_number_of_tables_of_oapth_schema<B>(
  c: &mut Commands<B,>,
  _: AuxTestParams,
) where
  B: BackEnd
{
  assert_eq!(c.back_end.tables("_oapth").await.unwrap().len(), 0);
  let _ = _migrate_doc_test(c).await;
  assert_eq!(c.back_end.tables("_oapth").await.unwrap().len(), 2);
}

pub(crate) async fn migrate_works<B>(c: &mut Commands<B>, aux: AuxTestParams)
where
  B: BackEnd
{
  crate::integration_tests::schema::migrate_works(c, aux, 2).await
}
