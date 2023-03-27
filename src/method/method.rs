
use std::fmt::Debug;
use mysql::*;
use serde::de::DeserializeOwned;
use serde::Serialize;


pub use mysql::TxOpts;
pub use mysql::PooledConn;
pub use mysql::Transaction;
pub use mysql::prelude::*;

pub struct MysqlQuick {
    pub pool: Pool,
}

impl MysqlQuick {
    /// 创建一个 mysql 的连接池。min/max 为最小最大连接数，url 为地址
    /// ```
    /// let conn = MysqlQuick::new(10, 100, "mysql://root:12345678@localhost:3306/dev_db").unwrap();
    /// ```
    pub fn new(min: usize, max: usize, url: &str) -> Result<MysqlQuick> {
        let pool = Pool::new_manual(min, max, url);
        match pool {
            Ok(p) => Ok(MysqlQuick {
                pool: p
            }),
            Err(e) => Err(e)
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
        Err(e) => Err(e)
    }
}

/// 运行sql语句，返回想要的结果
/// ### 用于：myget、myfind、mycount
/// ### 示例
/// ```
/// let sql1 = myget!("feedback", 33, "id as id, feedback.content as cc");
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Feedback {
///     id: u64,
///     cc: String
/// }
/// let res_get: (Vec<Feedback>, Option<(u64, String)>) = my_run(&mut conn, sql1).unwrap();
/// println!("get 结果 {:#?}", res_get);
/// ```
/// 
/// 
#[deprecated(since = "1.0.0", note = "此方法将在未来弃用，推荐使用 my_run_vec 。")]
pub fn my_run<T, U>(conn: &mut PooledConn, sql: String) -> Result<(Vec<U>, Option<T>)>
where
    T: FromRow + Serialize + Clone + Debug,
    U: DeserializeOwned
{
    let tmp_f: String = get_select_field(sql.clone());

    let check_res: Result<Vec<T>> = conn.query(sql);
    match check_res {
        Ok(c) => {
            if c.len() == 0 {
                Ok((vec![], None))
            } else {
                let check_one = c[0].clone();
                let res: Vec<U> = json_res(c, tmp_f.as_str());
                Ok((res, Some(check_one)))
            }
        },
        Err(e) => Err(e)
    }
}

/// 运行sql语句，返回想要的结果
/// ### 用于：myget、myfind、mycount
/// ### 示例
/// ```
/// let sql1 = myget!("feedback", 33, "id as id, feedback.content as cc");
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Feedback {
///     id: u64,
///     cc: String
/// }
/// let res_get: Vec<Feedback> = my_run_vec(&mut conn, sql1).unwrap();
/// println!("get 结果 {:#?}", res_get);
/// ```
/// 
/// 
pub fn my_run_vec<U>(conn: &mut PooledConn, sql: String) -> Result<Vec<U>>
where
    U: DeserializeOwned
{
    let tmp_f: String = get_select_field(sql.clone());
    let rows: Result<Vec<Row>> = conn.exec(sql, ());
    match rows {
        Ok(r) => {
            let j_res: Vec<U> = json_rows(r, tmp_f.as_str());
            Ok(j_res)
        },
        Err(e) => Err(e)
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
            let id = if let Some(i) = id {i} else {0};
            Ok(id)
        },
        Err(e) => Err(e)
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
/// let res_get: (Vec<Feedback>, Option<(u64, String)>) = my_run_tran(&mut tran, sql1).unwrap();
/// println!("get 结果 {:#?}", res_get);
/// ```
/// 
/// 
#[deprecated(since = "1.0.0", note = "此方法将在未来弃用，推荐使用 my_run_tran_vec 。")]
pub fn my_run_tran<T, U>(tran: &mut Transaction, sql: String) -> Result<(Vec<U>, Option<T>)>
where
    T: FromRow + Serialize + Clone + Debug,
    U: DeserializeOwned
{
    let tmp_f: String = get_select_field(sql.clone());
    let check_res: Result<Vec<T>> = tran.query(sql);
    match check_res {
        Ok(c) => {
            if c.len() == 0 {
                Ok((vec![], None))
            } else {
                let check_one = c[0].clone();
                let res: Vec<U> = json_res(c, tmp_f.as_str());
                Ok((res, Some(check_one)))
            }
        },
        Err(e) => Err(e)
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
/// 
/// 
pub fn my_run_tran_vec<U>(tran: &mut Transaction, sql: String) -> Result<Vec<U>>
where
    U: DeserializeOwned
{
    let tmp_f: String = get_select_field(sql.clone());
    let rows: Result<Vec<Row>> = tran.exec(sql, ());
    match rows {
        Ok(r) => {
            let j_res: Vec<U> = json_rows(r, tmp_f.as_str());
            Ok(j_res)
        },
        Err(e) => Err(e)
    }
}






fn json_rows<U>(rows: Vec<Row>, fields: &str) -> Vec<U> 
where
    U: DeserializeOwned
{
    let mut j_st = String::from("[");
    let field_string: String = fields.split_whitespace().collect();
    let field_list: Vec<&str> = field_string.split(",").collect();
    if rows.len() == 0 {
        let json_result: Vec<U> = serde_json::from_str(format!("[]").as_str()).unwrap();
        return json_result;
    }
    if field_list.len() == 1 {
        if field_list[0] == "mysql_quick_count" {
            let tmp = if let Some(s) = as_string(rows[0].clone(), 0) {s} else {String::from("null")};
            let json_result: Vec<U> = serde_json::from_str(format!("[{tmp}]").as_str()).unwrap();
            return json_result;
        }
    }
    for i in 0..rows.len() {
        let row = rows[i].clone();
        let mut one = "{".to_string();
        for (n, f_name) in field_list.iter().enumerate() {
            let tmp = if let Some(s) = as_string(row.clone(), n) {s} else {String::from("null")};
            one = one + "\"" + *f_name + "\": " + tmp.as_str() + ",";
        }
        one.pop();
        one.push('}');
        one.push(',');
        j_st = j_st + one.as_str();
    }
    j_st.pop();
    j_st.push(']');
    let json_result: Vec<U> = serde_json::from_str(j_st.as_str()).unwrap();
    json_result
}

fn as_string(row: Row, index: usize) -> Option<String> {
    row.as_ref(index).map(|value| match value {
        Value::NULL => String::from("null"),
        Value::Bytes(v) => "\"".to_string() + String::from_utf8_lossy(v.as_slice()).into_owned().as_str() + "\"",
        Value::Int(v) => format!("{v}"),
        Value::UInt(v) => format!("{v}"),
        Value::Float(v) => format!("{v}"),
        Value::Double(v) => format!("{v}"),
        Value::Date(year, month, day, hour, minutes, seconds, _micro) 
            => format!("\"{year}-{month}-{day} {hour}:{minutes}:{seconds}\""),
        Value::Time(negative, days, hours, minutes, seconds, micro) 
            => format!("\"{negative} {days} {hours}:{minutes}:{seconds}.{micro}\""),
    })
}


fn _type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

// 获取sql里面的 select 字段
fn get_select_field(sql: String) -> String {
    let re = regex::Regex::new(r"SELECT(.*)FROM").unwrap();
    let caps = re.captures(sql.as_str()).unwrap();

    let tmp_caps = caps[1].to_string();
    let table_field_vec: Vec<&str> = tmp_caps.split(",").collect();

    let mut field_vec: Vec<String> = vec![];
    for tf in table_field_vec.iter() {
        let temp_tf = *tf;
        if temp_tf.contains(" as ") {
            let tmpt: Vec<&str> = temp_tf.split("as").collect();
            let tmp_f = if let Some(l) = tmpt.last() {*l} else {""};
            let field: String = tmp_f.split_whitespace().collect();
            field_vec.push(field);
        } else if temp_tf.contains(" AS ") {
            let tmpt: Vec<&str> = temp_tf.split("AS").collect();
            let tmp_f = if let Some(l) = tmpt.last() {*l} else {""};
            let field: String = tmp_f.split_whitespace().collect();
            field_vec.push(field);
        } else if temp_tf.contains(".") {
            let tmpt: Vec<&str> = temp_tf.split(".").collect();
            let tmp_f = if let Some(l) = tmpt.last() {*l} else {""};
            let field: String = tmp_f.split_whitespace().collect();
            field_vec.push(field);
        } else {
            let field: String = temp_tf.split_whitespace().collect();
            field_vec.push(field);
        }
    }

    let result_field = field_vec.join(",");
    
    result_field
}





fn json_res<T, U>(p: Vec<T>, fields: &str) -> Vec<U> 
where
    T: FromRow + Serialize + Debug,
    U: DeserializeOwned
{
    let mut j_st = String::from("[");
    let field_string: String = fields.split_whitespace().collect();
    let field_list: Vec<&str> = field_string.split(",").collect();
    for item in p.iter() {
        let v_type = _type_of(item);
        if v_type.contains("(") {
            let tuple_i = serde_json::to_string_pretty(item).unwrap();
            let tm2: Vec<&str> = tuple_i.split("\n").collect();
            let tm = &tm2[1..tm2.len()-1];
            let mut one = "{".to_string();
            for (i, f_name) in field_list.iter().enumerate() {
                let mut tmp = tm[i].to_string();
                let last = &tmp[tmp.len()-1..tmp.len()];
                if last == "," {
                    tmp.pop();
                }
                one = one + "\"" + *f_name + "\": " + tmp.as_str() + ",";
            }
            one.pop();
            one.push('}');
            one.push(',');
            j_st = j_st + one.as_str();
        } else {
            let tuple_i = serde_json::to_string(item).unwrap();
            j_st = j_st + tuple_i.as_str() + ",";
        }
    }
    j_st.pop();
    j_st.push(']');
    let json_result: Vec<U> = serde_json::from_str(j_st.as_str()).unwrap();
    json_result
}
