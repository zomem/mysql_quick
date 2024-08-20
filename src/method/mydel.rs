/// 1.通过id，删除数据，返回 sql 语句。
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mysql_quick::{mydel, my_run_drop, MysqlQuick, MysqlQuickCount};
/// # const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";
/// # let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();
/// let sql = mydel!("for_test", 12);
/// my_run_drop(&mut conn, sql).unwrap();
///
/// ```
/// 2.通过指定字段的值，删除全部数据，返回 sql 语句。
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mysql_quick::{mydel, my_run_drop, MysqlQuick, MysqlQuickCount};
/// # const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";
/// # let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();
/// // 删除 uid = 5 的全部数据
/// let sql = mydel!("for_test", {"uid": 5});
/// my_run_drop(&mut conn, sql).unwrap();
/// ```
#[macro_export]
macro_rules! mydel {
    ($t:expr, {$k:tt: $v:expr}) => {{
        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let keys = $k.to_string();
        let temp_v = $v;
        let v_type = type_of(&temp_v);
        let values = match v_type {
            "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                v_r = v_r.replace("\"", "\\\"");
                "\"".to_string() + &v_r + "\""
            }
            "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" | "&i8" | "&i16" | "&i32"
            | "&i64" | "&i128" | "&isize" | "&f32" | "&f64" | "&f128" | "&bool" => {
                temp_v.to_string() + ""
            }
            _ => "".to_string(),
        };

        let sql: String =
            "DELETE FROM ".to_string() + $t + " WHERE " + keys.as_str() + "=" + values.as_str();

        sql
    }};

    ($t:expr, $v: expr) => {{
        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let temp_v = $v;
        let v_type = type_of(&temp_v);
        let values = match v_type {
            "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                v_r = v_r.replace("\"", "\\\"");
                "\"".to_string() + &v_r + "\""
            }
            "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" | "&i8" | "&i16" | "&i32"
            | "&i64" | "&i128" | "&isize" | "&f32" | "&f64" | "&f128" | "&bool" => {
                temp_v.to_string() + ""
            }
            _ => "".to_string(),
        };

        let sql: String = "DELETE FROM ".to_string() + $t + " WHERE id=" + values.as_str();

        sql
    }};
}
