#![cfg(feature = "embed-migrations")]

use oapth::Commands;

async fn _it_works() {
  let groups = oapth::embed_migrations!("oapth-test-utils/migrations.toml");
  let mut commands = Commands::with_back_end(());
  let groups_iter = groups.iter().map(|e| (e.0, e.1.iter().cloned()));
  commands.migrate_from_groups(groups_iter).await.unwrap();
}
