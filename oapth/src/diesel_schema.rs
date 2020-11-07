table! {
    _oapth_migration (id) {
        id -> Integer,
        _oapth_migration_group_version -> Integer,
        
        checksum -> Text,
        created_on -> Timestamp,
        name -> Text,
        version -> Integer,
    }
}

table! {
    _oapth_migration_group (version) {
        version -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    _oapth_migration,
    _oapth_migration_group,
);
