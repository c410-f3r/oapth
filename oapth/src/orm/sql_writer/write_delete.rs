use crate::orm::{
  buffer_write_fmt, node_was_already_visited, AuxNodes, SqlValue, SqlWriter, SqlWriterLogic, Table,
  TableParams,
};
use alloc::string::String;

impl<'entity, T> SqlWriterLogic<'entity, T>
where
  T: Table<'entity>,
  T::Associations: SqlWriter<Error = T::Error>,
{
  #[inline]
  pub(crate) fn write_delete(
    aux: &mut AuxNodes,
    buffer_cmd: &mut String,
    table: &TableParams<'entity, T>,
  ) -> Result<(), T::Error> {
    if node_was_already_visited(aux, table)? {
      return Ok(());
    }
    table.associations().write_delete(aux, buffer_cmd)?;
    Self::write_delete_manager(buffer_cmd, table)?;
    Ok(())
  }

  fn write_delete_manager(
    buffer_cmd: &mut String,
    table: &TableParams<'entity, T>,
  ) -> Result<(), T::Error> {
    let id_value = if let Some(el) = table.id_field().value() { el } else { return Ok(()) };
    buffer_write_fmt(
      buffer_cmd,
      format_args!("DELETE FROM {} WHERE {}=", T::TABLE_NAME, T::PRIMARY_KEY_NAME),
    )?;
    id_value.write(buffer_cmd)?;
    buffer_cmd.push(';');
    Ok(())
  }
}
