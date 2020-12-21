use crate::{
  integration_tests::{_migrate_doc_test, AuxTestParams},
  BackEnd, Commands,
};
use chrono::{DateTime, Duration, FixedOffset, Utc};

pub async fn _back_end_has_migration_with_utc_time<B>(c: &mut Commands<B>, _: AuxTestParams)
where
  B: BackEnd
{
  let mg = _migrate_doc_test(c).await;
  let created_on = *c.back_end.migrations(&mg).await.unwrap()[0].created_on();
  let range = created_on..=created_on + Duration::seconds(5);
  let utc: DateTime<FixedOffset> = Utc::now().into();
  assert!(range.contains(&utc));
}
