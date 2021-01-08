#![cfg(feature = "embed-migrations")]

use oapth::Commands;

oapth::embed_migrations!("oapth-test-utils/migrations.cfg");

async fn _it_works() {
  let mut commands = Commands::with_back_end(());
  let groups = GROUPS.iter().map(|e| (e.0, e.1.iter().cloned()));
  commands.migrate_from_groups(groups).await.unwrap();
}
