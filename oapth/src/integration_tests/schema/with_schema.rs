use crate::{
  integration_tests::{_migrate_doc_test, AuxTestParams},
  Backend, Commands,
};

pub(crate) async fn all_tables_returns_the_number_of_tables_of_oapth_schema<B>(
  buffer: &mut String,
  c: &mut Commands<B,>,
  _: AuxTestParams,
) where
  B: Backend
{
  assert_eq!(c.backend.tables("_oapth").await.unwrap().len(), 0);
  let _ = _migrate_doc_test(buffer, c).await;
  assert_eq!(c.backend.tables("_oapth").await.unwrap().len(), 2);
}

pub(crate) async fn migrate_works<B>(buffer: &mut String, c: &mut Commands<B>, aux: AuxTestParams)
where
  B: Backend
{
  crate::integration_tests::schema::migrate_works(buffer, c, aux, 2).await
}
