/// 新增数据 ，返回 sql 语句。
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mysql_quick::{myset, my_run_drop, MysqlQuick, MysqlQuickCount};
/// # const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";
/// # let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();
/// # let info = r#"m'y,,a#@!@$$33^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// let sql = myset!("for_test", {
///     "title": "set 新增",
///     "content": info,
///     "total": "null", // null 表示该字段为NULL
///     "uid": 8,
///     "price": 88.2,
/// });
/// let id = my_run_drop(&mut conn, sql).unwrap();
/// # if id <= 0 {
/// #    return assert!(false);
/// # }
/// ```
#[macro_export]
macro_rules! myset {
    ($t:expr, {$($k:tt: $v:expr),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let mut keys = String::from("");
            let mut values = String::from("");
            $(
                keys = keys + $k + ",";
            )+
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                if temp_v.to_string().as_str() == "null" {
                    values = values + "NULL,";
                } else {
                    values = match v_type {
                        "&&str" => {
                            let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            values + "\"" + &v_r + "\","
                        },
                        "&alloc::string::String" => {
                            let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            values + "\"" + &v_r + "\","
                        },
                        "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            values + "\"" + &v_r + "\","
                        },
                        _ => {
                            values + temp_v.to_string().as_str() + ","
                        }
                    };
                }
            )+

            keys.pop();
            values.pop();

            let sql: String = "INSERT INTO ".to_string() + $t + " ( " + keys.as_str() + " ) "
                + " VALUES ( " + values.as_str() + " )";

            sql
        }
    };
}
