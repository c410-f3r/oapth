use crate::{integration_tests::AuxTestParams, Backend, Commands};

pub(crate) async fn migrate_works<B>(buffer: &mut String, c: &mut Commands<B>, aux: AuxTestParams)
where
  B: Backend
{
  crate::integration_tests::schema::migrate_works(buffer, c, aux, 6).await
}
