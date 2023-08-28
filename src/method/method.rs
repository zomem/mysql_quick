use mysql::*;
pub use mysql::{prelude::*, PooledConn, Transaction, TxOpts};
use serde::de::DeserializeOwned;

pub struct MysqlQuick {
    pub pool: Pool,
}

impl MysqlQuick {
    /// 创建一个 mysql 的连接池。min/max 为最小最大连接数，url 为地址
    /// ```
    /// let conn = MysqlQuick::new("mysql://root:12345678@localhost:3306/dev_db").unwrap();
    /// ```
    pub fn new(url: &str) -> Result<MysqlQuick> {
        let pool = Pool::new(url);
        match pool {
            Ok(p) => Ok(MysqlQuick { pool: p }),
            Err(e) => Err(e),
        }
    }
}

/// 运行sql语句，返回最近一条语句的数据id，如果上没有，则返回0，用于set返回对应的id。
/// 其他方法mydel、mysetmany、myupdate，则不用管这个id。
/// ### 用于：myset、mydel、mysetmany、myupdate
/// ```
/// let id = my_run_drop(&mut conn, myset!("feedback", {
///    "content": "ADFaadf",
///    "uid": 9,
/// })).unwrap();
///
/// ```
///
///
pub fn my_run_drop(conn: &mut PooledConn, sql: String) -> Result<u64> {
    let temp_res = conn.query_drop(sql);
    match temp_res {
        Ok(_) => Ok(conn.last_insert_id()),
        Err(e) => Err(e),
    }
}

/// 运行sql语句，返回想要的结果
/// ### 用于：myget、myfind、mycount
/// ### 示例
/// ```
/// let sql = myget!("feedback", 33, "*");
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Feedback {
///     id: u64,
///     cc: String
/// }
/// let res_get: Vec<Feedback> = my_run_vec(&mut conn, sql).unwrap();
/// ```
pub fn my_run_vec<U>(conn: &mut PooledConn, sql: String) -> Result<Vec<U>>
where
    U: DeserializeOwned,
{
    // let tmp_f: String = get_select_field(&sql);
    let rows: Result<Vec<Row>> = conn.exec(sql, ());
    match rows {
        Ok(r) => {
            let j_res: Vec<U> = rows_to_json(r);
            Ok(j_res)
        }
        Err(e) => Err(e),
    }
}

/// ### 事务执行
/// 运行sql语句，返回上一条语句的id，如果上没有，则返回0
/// ### 用于：myset、myupdate、mydel、mysetmany
/// ### 示例
/// ```
/// let id = my_run_tran_drop(&mut tran, myset!("feedback", {
///    "content": "ADFaadf",
///     "uid": 9,
/// })).unwrap();
///
/// my_run_tran_drop(&mut tran, mydel!("feedback", 50)).unwrap();
///
/// my_run_tran_drop(&mut tran, myupdate!("feedback", 56, {
///     "content": "更新后的内容，一一一一"
/// })).unwrap();
/// ```
///
///
pub fn my_run_tran_drop(tran: &mut Transaction, sql: String) -> Result<u64> {
    let temp_tran = tran.query_drop(sql);
    match temp_tran {
        Ok(_) => {
            let id = tran.last_insert_id();
            let id = if let Some(i) = id { i } else { 0 };
            Ok(id)
        }
        Err(e) => Err(e),
    }
}

/// ### 事务执行
/// 运行sql语句
/// ### 用于：myget、myfind、mycount
/// ### 示例
/// ```
/// let sql1 = myget!("feedback", 33, "id as id, feedback.content as cc");
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Feedback {
///     id: u64,
///     cc: String
/// }
/// let res_get: Vec<Feedback> = my_run_tran_vec(&mut tran, sql1).unwrap();
/// println!("get 结果 {:#?}", res_get);
/// ```
pub fn my_run_tran_vec<U>(tran: &mut Transaction, sql: String) -> Result<Vec<U>>
where
    U: DeserializeOwned,
{
    let rows: Result<Vec<Row>> = tran.exec(sql, ());
    match rows {
        Ok(r) => {
            let j_res: Vec<U> = rows_to_json(r);
            Ok(j_res)
        }
        Err(e) => Err(e),
    }
}

// fn is_json_string(s: &str) -> bool {
//     match serde_json::from_str::<serde_json::Value>(s) {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }

fn rows_to_json<U>(rows: Vec<Row>) -> Vec<U>
where
    U: DeserializeOwned,
{
    if rows.len() == 0 {
        return vec![];
    }
    let mut j_st = String::from("[");
    for row in rows.into_iter() {
        let mut one = "{".to_string();
        for column in row.columns_ref() {
            // Cells in a row can be indexed by numeric index or by column name
            let column_name = column.name_str().to_string();
            let column_value = &row[column.name_str().as_ref()];
            let tmp = row_value_as_string(column_value);
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
    let json_result: Vec<U> = serde_json::from_str(j_st.as_str()).unwrap();
    json_result
}
fn row_value_as_string(value: &Value) -> String {
    match value {
        Value::NULL => String::from("null"),
        Value::Bytes(v) => {
            let mut info = String::from_utf8_lossy(v.as_slice())
                .into_owned()
                .to_string();
            info = serde_json::to_string(&info).unwrap();
            format!(r#"{info}"#)
        }
        Value::Int(v) => format!("{v}"),
        Value::UInt(v) => format!("{v}"),
        Value::Float(v) => format!("{v}"),
        Value::Double(v) => format!("{v}"),
        Value::Date(year, month, day, hour, minutes, seconds, _micro) => {
            format!("\"{year}-{month}-{day} {hour}:{minutes}:{seconds}\"")
        }
        Value::Time(negative, days, hours, minutes, seconds, micro) => {
            format!("\"{negative} {days} {hours}:{minutes}:{seconds}.{micro}\"")
        }
    }
}

fn _type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
