use crate::{
  integration_tests::{AuxTestParams, _migrate_doc_test},
  BackEnd, Commands,
};

pub async fn _all_tables_returns_the_number_of_tables_of_oapth_schema<B>(
  c: &mut Commands<B>,
  _: AuxTestParams,
) where
  B: BackEnd,
{
  assert_eq!(c.back_end.all_tables("_oapth").await.unwrap().len(), 0);
  _migrate_doc_test(c).await;
  assert_eq!(c.back_end.all_tables("_oapth").await.unwrap().len(), 2);
}

pub async fn _migrate_works<B>(c: &mut Commands<B>, aux: AuxTestParams)
where
  B: BackEnd,
{
  crate::integration_tests::schema::_migrate_works(c, aux, 2).await
}
