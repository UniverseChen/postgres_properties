# postgres_properties

postgresql table's properties

## 简介 - Introduction

该程序定义了 postgres 数据库表的属性; `<br />`
This lib define property of postgres table; `<br />`

<br />

目前版本包含了一些常规属性: `<br />`
Consist of general property: `<br />`

```sql
schema				模式名		schema name
name				表名		table name
owner				所有者		owner name
tablespace			表空间		table space name
partitioned_table		是否为分区表	 whether partition table
comments			注释		comments
```

## 示例 - Example

```rust
use postgres_properties::postgres_properties_derive::Properties;
use postgres_properties::PgTableProperties;

#[derive(Properties)]
#[general(
    schema = "schema",
    name = "table",
    owner = "owner",
    tablespace = "tablespace",
    partitioned_table = "false",
    comments = "comments"
)]
struct S;

assert_eq!("schema".to_string(), S::schema());
assert_eq!(r#""schema""#.to_string(), S::schema_sql());
assert_eq!("table".to_string(), S::name());
assert_eq!(r#""table""#.to_string(), S::name_sql());
assert_eq!("owner".to_string(), S::owner());
assert_eq!(r#""owner""#.to_string(), S::owner_sql());
assert_eq!("tablespace".to_string(), S::tablespace());
assert_eq!(r#""tablespace""#.to_string(), S::tablespace_sql());
assert_eq!(false, S::partitioned_table());
assert_eq!("comments".to_string(), S::comments());
assert_eq!(r#""schema"."table""#.to_string(), S::name_with_schema_sql());
assert_eq!(r#""schema"."table"."field""#.to_string(), S::field_complete_sql("field"));

```
