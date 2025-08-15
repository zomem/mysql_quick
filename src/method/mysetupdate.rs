/// 新增或更新数据 ，返回 sql 语句。
/// 第二个参数为指定 不重复的字段。
/// 如果第二个参数里的字段同时存在，则更新其他字段的值，否则新增。
/// ```
/// let sql = mysetupdate!("for_test", "uid,title", {
///     "title": "set 新增",
///     "content": info,
///     "total": "null", // 新增字段为NULL
///     "total2": None, // 忽略该字段（默认值DEFAULT）
///     "uid": 8,
///     "price": Some(88.2), // 将新增为88.2
/// });
/// let id = my_run_drop(&mut conn, sql).unwrap();
/// ```
#[macro_export]
macro_rules! mysetupdate {
    ($t:expr, $i:expr, {$($k:tt: $v:expr),+$(,)?}) => {
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
            let mut values_upd = String::from("");
            let unique_keys = $i.split(",").collect::<Vec<&str>>();
            $(
                let temp_op = $v;
                let op_v_type = type_of(&temp_op);
                let mut temp_v: String;
                let mut v_type = "&&str";
                let mut is_option_none = false;
                if op_v_type.contains("&core::option::Option") {
                    let op_str = format!("{:?}", temp_op);
                    if op_str == "None".to_string() {
                        temp_v = "null".to_string();
                        is_option_none = true;
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
                let item_k = $k;
                if !is_option_none {
                    if temp_v.as_str() == "null" || temp_v.as_str() == "\"null\"" {
                        values = values + "NULL,";
                        if !unique_keys.contains(&item_k) {
                            values_upd = values_upd + item_k + " = VALUES(" + item_k + "),";
                        }
                    } else {
                        match v_type {
                            "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                                temp_v.remove(0);
                                temp_v.pop();
                                // let mut v_r = temp_v.as_str().replace("\\", "\\\\");
                                // v_r = v_r.replace("\"", "\\\"");
                                values = values + "\"" + &temp_v + "\",";
                                if !unique_keys.contains(&item_k) {
                                    values_upd = values_upd + item_k + " = VALUES(" + item_k + "),";
                                }
                            },
                            "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                            "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                            "&f32" | "&f64" | "&f128" | "&bool" => {
                                values = values + temp_v.as_str() + ",";
                                if !unique_keys.contains(&item_k) {
                                    values_upd = values_upd + item_k + " = VALUES(" + item_k + "),";
                                }
                            },
                            "&&u8" | "&&u16" | "&&u32" | "&&u64" | "&&u128" | "&&usize" |
                            "&&i8" | "&&i16" | "&&i32" | "&&i64" | "&&i128" | "&&isize" |
                            "&&f32" | "&&f64" | "&&f128" | "&&bool" => {
                                values = values + temp_v.as_str() + ",";
                                if !unique_keys.contains(&item_k) {
                                    values_upd = values_upd + item_k + " = VALUES(" + item_k + "),";
                                }
                            },
                            _ => {
                               "".to_string();
                            },
                        };
                    }
                    keys = keys + item_k + ",";
                }
            )+

            keys.pop();
            values.pop();
            values_upd.pop();

            let sql: String = "INSERT INTO ".to_string() + $t + " ( " + keys.as_str() + " ) "
                + " VALUES ( " + values.as_str() + " )" + " ON DUPLICATE KEY UPDATE " + values_upd.as_str();

            sql
        }
    };
}
