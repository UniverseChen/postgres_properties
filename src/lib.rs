/// postgres table's properties
///
/// # Examples
/// ```
/// use postgres_properties::postgres_properties_derive::Properties;
/// use postgres_properties::PgTableProperties;
/// #[derive(Properties)]
/// #[general(
///     schema = "schema",
///     table = "table",
///     owner = "owner",
///     tablespace = "tablespace",
///     partitioned_table = "false",
///     comments = "comments"
/// )]
/// struct S;
///
/// assert_eq!("schema".to_string(), S::schema());
/// assert_eq!(r#""schema""#.to_string(), S::schema_sql());
/// assert_eq!("table".to_string(), S::table());
/// assert_eq!(r#""table""#.to_string(), S::table_sql());
/// assert_eq!("owner".to_string(), S::owner());
/// assert_eq!(r#""owner""#.to_string(), S::owner_sql());
/// assert_eq!("tablespace".to_string(), S::tablespace());
/// assert_eq!(r#""tablespace""#.to_string(), S::tablespace_sql());
/// assert_eq!(false, S::partitioned_table());
/// assert_eq!("comments".to_string(), S::comments());
/// assert_eq!(r#""schema"."table""#.to_string(), S::schema_table_sql());
/// assert_eq!(r#""schema"."table"."field""#.to_string(), S::schema_table_field_sql("field"));
/// ```
pub trait PgTableProperties {
    /// schema name
    fn schema() -> String;
    fn schema_sql() -> String;
    /// table name
    fn table() -> String;
    fn table_sql() -> String;
    /// table's owner name
    fn owner() -> String;
    fn owner_sql() -> String;
    /// tablespace name
    fn tablespace() -> String;
    fn tablespace_sql() -> String;
    /// is the partition
    fn partitioned_table() -> bool;
    /// table's comments
    fn comments() -> String;
    /// query table name, such as "schema"."table"
    fn schema_table_sql() -> String;
    /// field complete name, such as "schema"."table"."field"
    fn schema_table_field_sql(field: &str) -> String;
}

pub extern crate postgres_properties_derive;
