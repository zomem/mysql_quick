use mysql::*;
pub use mysql::{prelude::*, PooledConn, Transaction, TxOpts};
use serde::de::DeserializeOwned;
use std::fmt::Display;

/// Sql(String)，会将 String 识别为 sql 语句，而不是参数值
///
/// 仅支持 myget mycount myfind
#[derive(Debug)]
pub struct Sql<T: Into<String>>(pub T);
impl<T: Into<String> + Display> Display for Sql<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "Sql({})", self.0)
    }
}

pub struct MysqlQuick {
    pub pool: Pool,
}

impl MysqlQuick {
    /// 创建一个 mysql 的连接池。min/max 为最小最大连接数，url 为地址
    pub fn new(url: &str) -> anyhow::Result<MysqlQuick> {
        let pool = Pool::new(url)?;
        Ok(MysqlQuick { pool })
    }
}

/// 运行sql语句，返回最近一条语句的数据 id，如果上没有，则返回0
/// 用于：`myset、mydel、mysetmany、myupdate、myupdatemany`
pub fn my_run_drop(conn: &mut PooledConn, sql: String) -> anyhow::Result<u64> {
    conn.query_drop(sql)?;
    Ok(conn.last_insert_id())
}

/// 运行sql语句，返回结果列表
///
/// 用于：`myget、myfind、mycount`
pub fn my_run_vec<U>(conn: &mut PooledConn, sql: String) -> anyhow::Result<Vec<U>>
where
    U: DeserializeOwned,
{
    // let tmp_f: String = get_select_field(&sql);
    let rows: Vec<Row> = conn.exec(sql, ())?;
    let j_res: Vec<U> = rows_to_json(rows)?;
    Ok(j_res)
}

/// 事务执行
///
/// 运行sql语句，返回上一条语句的id，如果上没有，则返回0
///
/// 用于：`myset、myupdate、mydel、mysetmany、myupdatemany`
pub fn my_run_tran_drop(tran: &mut Transaction, sql: String) -> anyhow::Result<u64> {
    tran.query_drop(sql)?;
    let id = tran.last_insert_id();
    let id = if let Some(i) = id { i } else { 0 };
    Ok(id)
}

/// 事务执行
///
/// 运行sql语句
///
/// 用于：`myget、myfind、mycount`
pub fn my_run_tran_vec<U>(tran: &mut Transaction, sql: String) -> anyhow::Result<Vec<U>>
where
    U: DeserializeOwned,
{
    let rows: Vec<Row> = tran.exec(sql, ())?;
    let j_res: Vec<U> = rows_to_json(rows)?;
    Ok(j_res)
}

// fn is_json_string(s: &str) -> bool {
//     match serde_json::from_str::<serde_json::Value>(s) {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }

fn rows_to_json<U>(rows: Vec<Row>) -> anyhow::Result<Vec<U>>
where
    U: DeserializeOwned,
{
    if rows.len() == 0 {
        return Ok(vec![]);
    }
    let mut j_st = String::from("[");
    for row in rows.into_iter() {
        let mut one = "{".to_string();
        for column in row.columns_ref() {
            // Cells in a row can be indexed by numeric index or by column name
            let column_name = column.name_str().to_string();
            let column_value = &row[column.name_str().as_ref()];
            let tmp = row_value_as_string(column_value)?;
            one = one + "\"" + column_name.as_str() + "\": " + tmp.as_str() + ",";
        }
        one.pop();
        one.push('}');
        one.push(',');
        j_st = j_st + one.as_str();
    }
    j_st.pop();
    j_st.push(']');
    // println!("j_st>>>>>>>  {}\n", j_st);
    let json_result: Vec<U> = serde_json::from_str(j_st.as_str())?;
    Ok(json_result)
}
fn row_value_as_string(value: &Value) -> anyhow::Result<String> {
    match value {
        Value::NULL => Ok(String::from("null")),
        Value::Bytes(v) => {
            let mut info = String::from_utf8_lossy(v.as_slice())
                .into_owned()
                .to_string();
            info = serde_json::to_string(&info)?;
            Ok(format!(r#"{info}"#))
        }
        Value::Int(v) => Ok(format!("{v}")),
        Value::UInt(v) => Ok(format!("{v}")),
        Value::Float(v) => Ok(format!("{v}")),
        Value::Double(v) => Ok(format!("{v}")),
        Value::Date(year, month, day, hour, minutes, seconds, _micro) => {
            let m = if month < &10 {
                format!("0{month}")
            } else {
                format!("{month}")
            };
            let d = if day < &10 {
                format!("0{day}")
            } else {
                format!("{day}")
            };
            let h = if hour < &10 {
                format!("0{hour}")
            } else {
                format!("{hour}")
            };
            let min = if minutes < &10 {
                format!("0{minutes}")
            } else {
                format!("{minutes}")
            };
            let s = if seconds < &10 {
                format!("0{seconds}")
            } else {
                format!("{seconds}")
            };

            Ok(format!("\"{year}-{m}-{d} {h}:{min}:{s}\""))
        }
        Value::Time(negative, days, hours, minutes, seconds, micro) => Ok(format!(
            "\"{negative} {days} {hours}:{minutes}:{seconds}.{micro}\""
        )),
    }
}

fn _type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
