use crate::orm::{
  buffer_write_fmt, node_was_already_visited, truncate_if_ends_with_char, AuxNodes, SqlValue,
  SqlWriter, SqlWriterLogic, Table, TableFields, TableParams,
};

impl<'entity, T> SqlWriterLogic<'entity, T>
where
  T: Table<'entity>,
  T::Associations: SqlWriter<Error = T::Error>,
{
  #[inline]
  pub(crate) fn write_update(
    aux: &mut AuxNodes,
    buffer_cmd: &mut String,
    table: &TableParams<'entity, T>,
  ) -> Result<(), T::Error> {
    if node_was_already_visited(aux, table)? {
      return Ok(());
    }
    Self::write_update_manager(buffer_cmd, table)?;
    table.associations().write_update(aux, buffer_cmd)?;
    Ok(())
  }

  fn write_update_manager(
    buffer_cmd: &mut String,
    table: &TableParams<'entity, T>,
  ) -> Result<(), T::Error> {
    let id_value = if let Some(el) = table.id_field().value() { el } else { return Ok(()) };

    buffer_write_fmt(buffer_cmd, format_args!("UPDATE {} SET ", T::TABLE_NAME))?;

    buffer_write_fmt(buffer_cmd, format_args!("{}=", table.id_field().name()))?;
    id_value.write(buffer_cmd)?;
    buffer_cmd.push(',');
    table.fields().write_update_values(buffer_cmd)?;
    truncate_if_ends_with_char(buffer_cmd, ',');

    buffer_cmd.push_str(" WHERE ");
    buffer_write_fmt(buffer_cmd, format_args!("{}=", T::PRIMARY_KEY_NAME))?;
    id_value.write(buffer_cmd)?;
    buffer_cmd.push(';');

    Ok(())
  }
}
