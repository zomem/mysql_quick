/// 1.通过id，更新数据 ，返回 sql 语句。
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mysql_quick::{myupdate, my_run_drop, MysqlQuick, MysqlQuickCount};
/// # const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";
/// # let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();
/// # let info = r#"m'y,,a#@!@$$33^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// let sql = myupdate!("for_test", 6, {
///     "title": "更新操作",
///     "content": info,
///     "uid": 9,
///     "price": None,// None 或 "null" 表示更新字段值为NULL
///     "total": Some(20),
/// });
/// my_run_drop(&mut conn, sql).unwrap();
///
/// // 原子更新，(如果使用[字段，值]的方式，都所有都需要使用这种形式)
/// let sql = myupdate!("for_test", 7, {
///     "title": ["set", "价格减10"],  // set 修改操作
///     "price": ["incr", -10],   // incr 原子性加减
///     "content": ["unset", ""],   // unset 清空值
/// });
/// my_run_drop(&mut conn, sql).unwrap();
///
/// ```
///
/// 2.通过指定字段的值，更新数据 ，返回 sql 语句。
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mysql_quick::{myupdate, my_run_drop, MysqlQuick, MysqlQuickCount};
/// # const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";
/// # let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();
/// let sql = myupdate!("for_test", {"uid": 3}, {"title": "更新了uid为3的数据"}); // 更新 uid = 3 的第一条数据
/// my_run_drop(&mut conn, sql).unwrap();
///
/// // 原子性更新
/// let sql = myupdate!("for_test", {"uid": 3}, {"total": ["incr", 1]});
/// my_run_drop(&mut conn, sql).unwrap();
/// ```
#[macro_export]
macro_rules! myupdate {
    ($t:expr, {$ik:tt: $iv:expr}, {$($k:tt: [$m:tt, $v:expr]),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            fn get_v_type(t: &str) -> &'static str {
                if t.contains("u8") ||
                    t.contains("u16") ||
                    t.contains("u32") ||
                    t.contains("u64") ||
                    t.contains("u128") ||
                    t.contains("usize") ||
                    t.contains("i8") ||
                    t.contains("i16") ||
                    t.contains("i32") ||
                    t.contains("i64") ||
                    t.contains("i64") ||
                    t.contains("i128") ||
                    t.contains("isize") ||
                    t.contains("f32") ||
                    t.contains("f64") ||
                    t.contains("f128") ||
                    t.contains("bool")
                {
                    return "&u8";
                }
                "&&str"
            }
            let tmp_ik = $ik.to_string();
            let i_data = $iv;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                "&f32" | "&f64" | "&f128" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                }
            };


            let mut temp_s = String::from("");
            $(
                let temp_op = $v;
                let op_v_type = type_of(&temp_op);
                let mut temp_v: String;
                let mut v_type = "&&str";
                let value;
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
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            temp_v.remove(0);
                            temp_v.pop();
                            let mut v_r = temp_v.as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            "\"".to_string() + &v_r + "\","
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                        "&f32" | "&f64" | "&f128" | "&bool" => {
                            temp_v + ","
                        },
                        _ => {
                           "".to_string()
                        }
                    };
                }

                let tmp_s = match $m {
                    "set" => $k.to_string() + "=" + value.as_str(),
                    "incr" => {
                        let mut op = "+";
                        let first = &value.as_str()[0..1];
                        if first == "-" {
                            op = ""
                        }
                        $k.to_string() + "=" + $k + op + value.as_str()
                    },
                    "unset" => $k.to_string() + "=NULL,",
                    _ => $k.to_string() + "=" + value.as_str(),
                };
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE " + tmp_ik.as_str() + "=" + tmp_i.as_str();

            sql
        }
    };

    ($t:expr, {$ik:tt: $iv:expr}, {$($k:tt: $v:expr),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            fn get_v_type(t: &str) -> &'static str {
                if t.contains("u8") ||
                    t.contains("u16") ||
                    t.contains("u32") ||
                    t.contains("u64") ||
                    t.contains("u128") ||
                    t.contains("usize") ||
                    t.contains("i8") ||
                    t.contains("i16") ||
                    t.contains("i32") ||
                    t.contains("i64") ||
                    t.contains("i64") ||
                    t.contains("i128") ||
                    t.contains("isize") ||
                    t.contains("f32") ||
                    t.contains("f64") ||
                    t.contains("f128") ||
                    t.contains("bool")
                {
                    return "&u8";
                }
                "&&str"
            }
            let tmp_ik = $ik.to_string();
            let i_data = $iv;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                "&f32" | "&f64" | "&f128" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_op = $v;
                let op_v_type = type_of(&temp_op);
                let mut temp_v: String;
                let mut v_type = "&&str";
                let value;
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
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            temp_v.remove(0);
                            temp_v.pop();
                            let mut v_r = temp_v.as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            "\"".to_string() + &v_r + "\","
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                        "&f32" | "&f64" | "&f128" | "&bool" => {
                            temp_v + ","
                        },
                        _ => {
                           "".to_string()
                        },
                    };
                }
                let tmp_s = $k.to_string() + "=" + value.as_str();
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE " + tmp_ik.as_str() + "=" + tmp_i.as_str();

            sql
        }
    };

    ($t:expr, $i:expr, {$($k:tt: [$m:tt, $v:expr]),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            fn get_v_type(t: &str) -> &'static str {
                if t.contains("u8") ||
                    t.contains("u16") ||
                    t.contains("u32") ||
                    t.contains("u64") ||
                    t.contains("u128") ||
                    t.contains("usize") ||
                    t.contains("i8") ||
                    t.contains("i16") ||
                    t.contains("i32") ||
                    t.contains("i64") ||
                    t.contains("i64") ||
                    t.contains("i128") ||
                    t.contains("isize") ||
                    t.contains("f32") ||
                    t.contains("f64") ||
                    t.contains("f128") ||
                    t.contains("bool")
                {
                    return "&u8";
                }
                "&&str"
            }
            let i_data = $i;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                "&f32" | "&f64" | "&f128" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_op = $v;
                let op_v_type = type_of(&temp_op);
                let mut temp_v: String;
                let mut v_type = "&&str";
                let value;
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
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            temp_v.remove(0);
                            temp_v.pop();
                            let mut v_r = temp_v.as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            "\"".to_string() + &v_r + "\","
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                        "&f32" | "&f64" | "&f128" | "&bool" => {
                            temp_v + ","
                        },
                        _ => {
                           "".to_string()
                        },
                    };
                }
                let tmp_s = match $m {
                    "set" => $k.to_string() + "=" + value.as_str(),
                    "incr" => {
                        let mut op = "+";
                        let first = &value.as_str()[0..1];
                        if first == "-" {
                            op = ""
                        }
                        $k.to_string() + "=" + $k + op + value.as_str()
                    },
                    "unset" => $k.to_string() + "=NULL,",
                    _ => $k.to_string() + "=" + value.as_str(),
                };
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE id=" + tmp_i.as_str();

            sql
        }
    };

    ($t:expr, $i:expr, {$($k:tt: $v:expr),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            fn get_v_type(t: &str) -> &'static str {
                if t.contains("u8") ||
                    t.contains("u16") ||
                    t.contains("u32") ||
                    t.contains("u64") ||
                    t.contains("u128") ||
                    t.contains("usize") ||
                    t.contains("i8") ||
                    t.contains("i16") ||
                    t.contains("i32") ||
                    t.contains("i64") ||
                    t.contains("i64") ||
                    t.contains("i128") ||
                    t.contains("isize") ||
                    t.contains("f32") ||
                    t.contains("f64") ||
                    t.contains("f128") ||
                    t.contains("bool")
                {
                    return "&u8";
                }
                "&&str"
            }
            let i_data = $i;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                "&f32" | "&f64" | "&f128" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_op = $v;
                let op_v_type = type_of(&temp_op);
                let mut temp_v: String;
                let mut v_type = "&&str";
                let value;
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
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            temp_v.remove(0);
                            temp_v.pop();
                            let mut v_r = temp_v.as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            "\"".to_string() + &v_r + "\","
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&u128" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&i128" | "&isize" |
                        "&f32" | "&f64" | "&f128" | "&bool" => {
                            temp_v + ","
                        },
                        _ => {
                           "".to_string()
                        },
                    };
                }
                let tmp_s = $k.to_string() + "=" + value.as_str();
                temp_s = temp_s + tmp_s.as_str();
            )+

            temp_s.pop();

            let sql: String = "UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE id=" + tmp_i.as_str();

            sql
        }
    };
}
