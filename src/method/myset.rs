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
///     "total": None, // None 或 "null" 表示新增字段为NULL
///     "uid": 8,
///     "price": Some(88.2),
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
            fn get_v_type(t: &'static str) -> &'static str {
                match t {
                    "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                        t
                    },
                    "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                    "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                    "&f32" | "&f64" | "&f128" | "&bool" => {
                        t
                    },
                    "&&u8" | "&&u16" | "&&u32" | "&&u64" | "&&u128" | "&&usize" |
                    "&&i8" | "&&i16" | "&&i32" | "&&i64" | "&&i128" | "&&isize" |
                    "&&f32" | "&&f64" | "&&f128" | "&&bool" => {
                        t
                    },
                    "&core::option::Option<&str>" |
                    "&core::option::Option<alloc::string::String>" |
                    "&core::option::Option<&alloc::string::String>" => {
                        "&&str"
                    },
                    "&&core::option::Option<&str>" |
                    "&&core::option::Option<alloc::string::String>" |
                    "&&core::option::Option<&alloc::string::String>" => {
                        "&&str"
                    },
                    "&core::option::Option<u8>" |
                    "&core::option::Option<u16>" |
                    "&core::option::Option<u32>" |
                    "&core::option::Option<u64>" |
                    "&core::option::Option<u128>" |
                    "&core::option::Option<usize>" |
                    "&core::option::Option<i8>" |
                    "&core::option::Option<i16>" |
                    "&core::option::Option<i32>" |
                    "&core::option::Option<i64>" |
                    "&core::option::Option<i128>" |
                    "&core::option::Option<isize>" |
                    "&core::option::Option<f32>" |
                    "&core::option::Option<f64>" |
                    "&core::option::Option<f128>" |
                    "&core::option::Option<bool>" => {
                        "&u8"
                    },
                    "&&core::option::Option<u8>" |
                    "&&core::option::Option<u16>" |
                    "&&core::option::Option<u32>" |
                    "&&core::option::Option<u64>" |
                    "&&core::option::Option<u128>" |
                    "&&core::option::Option<usize>" |
                    "&&core::option::Option<i8>" |
                    "&&core::option::Option<i16>" |
                    "&&core::option::Option<i32>" |
                    "&&core::option::Option<i64>" |
                    "&&core::option::Option<i128>" |
                    "&&core::option::Option<isize>" |
                    "&&core::option::Option<f32>" |
                    "&&core::option::Option<f64>" |
                    "&&core::option::Option<f128>" |
                    "&&core::option::Option<bool>" => {
                        "&u8"
                    },
                    _ => {
                       "&&str"
                    },
                }
            }
            let mut keys = String::from("");
            let mut values = String::from("");
            $(
                keys = keys + $k + ",";
            )+
            $(
                let temp_op = $v;
                let op_v_type = type_of(&temp_op);
                let mut temp_v: String;
                let mut v_type = "&&str";
                if op_v_type.contains("&core::option::Option") {
                    let op_str = format!("{:?}", temp_op);
                    if op_str == "None".to_string() {
                        temp_v = "null".to_string();
                    } else {
                        let mut t = op_str.replace("Some(", "");
                        t.pop();
                        temp_v = t;
                        v_type = get_v_type(op_v_type)
                    }
                } else {
                    temp_v = format!("{:?}", temp_op);
                    v_type = get_v_type(op_v_type)
                }
                if temp_v.as_str() == "null" {
                    values = values + "NULL,";
                } else {
                    values = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            temp_v.remove(0);
                            temp_v.pop();
                            let mut v_r = temp_v.as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            values + "\"" + &v_r + "\","
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                        "&f32" | "&f64" | "&f128" | "&bool" => {
                            values + temp_v.as_str() + ","
                        },
                        "&&u8" | "&&u16" | "&&u32" | "&&u64" | "&&u128" | "&&usize" |
                        "&&i8" | "&&i16" | "&&i32" | "&&i64" | "&&i128" | "&&isize" |
                        "&&f32" | "&&f64" | "&&f128" | "&&bool" => {
                            values + temp_v.as_str() + ","
                        },
                        _ => {
                           "".to_string()
                        },
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
