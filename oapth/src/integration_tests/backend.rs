use crate::{
  integration_tests::{_migrate_doc_test, AuxTestParams},
  Backend, Commands,
};
use chrono::{DateTime, Duration, FixedOffset, Utc};

pub(crate) async fn _backend_has_migration_with_utc_time<B>(buffer: &mut String, c: &mut Commands<B>, _: AuxTestParams)
where
  B: Backend,
{
  let mg = _migrate_doc_test(buffer, c).await;
  let created_on = *c.backend.migrations(buffer, &mg).await.unwrap()[0].created_on();
  let range = created_on..=created_on + Duration::seconds(5);
  let utc: DateTime<FixedOffset> = Utc::now().into();
  assert!(range.contains(&utc));
}
