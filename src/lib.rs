/// postgres table's properties
///
/// # Examples
/// ```
/// use postgres_properties::postgres_properties_derive::Properties;
/// use postgres_properties::PgTableProperties;
/// #[derive(Properties)]
/// #[general(
///     schema = "schema",
///     name = "table",
///     owner = "owner",
///     tablespace = "tablespace",
///     partitioned_table = "false",
///     comments = "comments"
/// )]
/// struct S;
///
/// assert_eq!("schema".to_string(), S::schema());
/// assert_eq!("table".to_string(), S::name());
/// assert_eq!("owner".to_string(), S::owner());
/// assert_eq!("tablespace".to_string(), S::tablespace());
/// assert_eq!(false, S::partitioned_table());
/// assert_eq!("comments".to_string(), S::comments());
/// assert_eq!("\"schema\".\"table\"".to_string(), S::name_with_schema_sql());
/// ```
pub trait PgTableProperties {
    /// schema name
    fn schema() -> String;
    /// table name
    fn name() -> String;
    /// table's owner name
    fn owner() -> String;
    /// tablespace name
    fn tablespace() -> String;
    /// is the partition
    fn partitioned_table() -> bool;
    /// table's comments
    fn comments() -> String;
    /// query table name
    fn name_with_schema_sql() -> String;
}

pub extern crate postgres_properties_derive;
// pub use postgres_properties_derive::*;
