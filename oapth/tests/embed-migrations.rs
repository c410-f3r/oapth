#![cfg(feature = "embed-migrations")]

use oapth::Commands;

async fn _it_works() {
  let groups = oapth::embed_migrations!("oapth-test-utils/migrations.toml");
  let mut commands = Commands::with_backend(());
  commands.migrate_from_groups(&mut String::new(), groups).await.unwrap();
}
