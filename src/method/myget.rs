/// 获取一条数据，返回 sql 语句
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mysql_quick::{myget, my_run_vec, MysqlQuick, MysqlQuickCount};
/// # const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";
/// # let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();
/// // 1.根据id查寻一条数据
/// #[derive(Serialize, Deserialize, Debug)]
/// struct Item {
///     id: u64,
///     cc: String
/// }
/// let sql = myget!("for_test", 5, "id,content as cc"); // 查寻 id = 5 的数据
/// let res_get: Vec<Item> = my_run_vec(&mut conn, sql).unwrap();
/// # if res_get.len() != 1 {
/// #    return assert!(false);
/// # }
///
/// // 2.根据指定字段的值查寻数据(满足条件的全部数据)
/// # let info = r#"m'y,,a#@!@$$33^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// let sql = myget!("for_test", {"content": info}); // 查寻 content = info 的数据
/// let res_get: Vec<serde_json::Value> = my_run_vec(&mut conn, sql).unwrap();
/// # if res_get.len() < 1 {
/// #    return assert!(false);
/// # }
/// ```
///
#[macro_export]
macro_rules! myget {
    ($t:expr, {$k:tt: $v:expr} $(,$select:expr)?$(,)?) => {
        {
            fn _type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            fn get_table(tt: &str) -> &str {
                let t_list: Vec<&str> = tt.split_whitespace().collect();
                let table_change = t_list[t_list.len() - 1];
                table_change
            }
            // 将没有带上表名的字段，都重新命名为 主表字段  main_t_change是重命名后的
            fn _rename_field(field: &str, main_t_change: &str) -> String {
                let mut tmp_name = field.to_string();
                if !field.contains(".") {
                    let tmp = main_t_change.to_string() + "." + field;
                    tmp_name = tmp;
                }
                tmp_name
            }
            fn _get_select<T: Into<String> + std::fmt::Display>(s: T, main_table_change: &str) -> String {
                let mut tmp_select = String::from("");
                for v in s.to_string().split(",").collect::<Vec<&str>>().iter() {
                    let tmpv = v.trim();
                    tmp_select = tmp_select + _rename_field(tmpv, main_table_change).as_str() + ",";
                }
                tmp_select.pop();
                tmp_select
            }

            let keys = $k.to_string();
            let temp_v = $v;
            let v_type = _type_of(&temp_v);
            let values = match v_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "&mysql_quick::method::method::Sql<&str>" |
                "&mysql_quick::method::method::Sql<alloc::string::String>" => {
                    temp_v.to_string().replace("Sql", "")
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    temp_v.to_string() + ""
                },
                _ => {
                   "".to_string()
                }
            };
            let _table_change = get_table($t);
            let mut _select = "*";
            $(
                let tmp_s = _get_select($select, _table_change);
                _select = tmp_s.as_str();
            )?

            let sql = "SELECT ".to_string() + _select +
                " FROM " + $t +
                " WHERE " + keys.as_str() + "=" + values.as_str();

            sql
        }
    };
    ($t:expr, $v: expr $(,$select:expr)?$(,)?) => {
        {
            fn _type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            fn get_table(tt: &str) -> &str {
                let t_list: Vec<&str> = tt.split_whitespace().collect();
                let table_change = t_list[t_list.len() - 1];
                table_change
            }
            // 将没有带上表名的字段，都重新命名为 主表字段  main_t_change是重命名后的
            fn _rename_field(field: &str, main_t_change: &str) -> String {
                let mut tmp_name = field.to_string();
                if !field.contains(".") {
                    let tmp = main_t_change.to_string() + "." + field;
                    tmp_name = tmp;
                }
                tmp_name
            }
            fn _get_select<T: Into<String> + std::fmt::Display>(s: T, main_table_change: &str) -> String {
                let mut tmp_select = String::from("");
                for v in s.to_string().split(",").collect::<Vec<&str>>().iter() {
                    let tmpv = v.trim();
                    tmp_select = tmp_select + _rename_field(tmpv, main_table_change).as_str() + ",";
                }
                tmp_select.pop();
                tmp_select
            }

            let temp_v = $v;
            let v_type = _type_of(&temp_v);
            let values = match v_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "&mysql_quick::method::method::Sql<&str>" |
                "&mysql_quick::method::method::Sql<alloc::string::String>" => {
                    temp_v.to_string().replace("Sql", "")
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    temp_v.to_string() + ""
                },
                _ => {
                   "".to_string()
                }
            };
            let _table_change = get_table($t);
            let mut _select = "*";
            $(
                let tmp_s = _get_select($select, _table_change);
                _select = tmp_s.as_str();
            )?

            let sql = "SELECT ".to_string() + _select +
                " FROM " + $t +
                " WHERE id=" + values.as_str();

            sql
        }
    };
}
