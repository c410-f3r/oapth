use crate::{integration_tests::AuxTestParams, BackEnd, Commands};

pub async fn migrate_works<B>(c: &mut Commands<B>, aux: AuxTestParams)
where
  B: BackEnd
{
  crate::integration_tests::schema::migrate_works(c, aux, 6).await
}
