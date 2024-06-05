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
///     "price": "null",    // 表示更新该字段值为NULL
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
            let tmp_ik = $ik.to_string();
            let i_data = $iv;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                }
            };


            let mut temp_s = String::from("");
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                let value;
                if temp_v.to_string().as_str() == "null" {
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            "\"".to_string() + &v_r + "\","
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                        "&f32" | "&f64" | "&bool" => {
                            temp_v.to_string() + ","
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

            let sql: String = "(UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE " + tmp_ik.as_str() + "=" + tmp_i.as_str() + ")";

            sql
        }
    };

    ($t:expr, {$ik:tt: $iv:expr}, {$($k:tt: $v:expr),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
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
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                let value;
                if temp_v.to_string().as_str() == "null" {
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            "\"".to_string() + &v_r + "\","
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                        "&f32" | "&f64" | "&bool" => {
                            temp_v.to_string() + ","
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

            let sql: String = "(UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE " + tmp_ik.as_str() + "=" + tmp_i.as_str() + ")";

            sql
        }
    };

    ($t:expr, $i:expr, {$($k:tt: [$m:tt, $v:expr]),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let i_data = $i;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                let value;
                if temp_v.to_string().as_str() == "null" {
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            "\"".to_string() + &v_r + "\","
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                        "&f32" | "&f64" | "&bool" => {
                            temp_v.to_string() + ","
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

            let sql: String = "(UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE id=" + tmp_i.as_str() + ")";

            sql
        }
    };

    ($t:expr, $i:expr, {$($k:tt: $v:expr),+$(,)?}) => {
        {
            fn type_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let i_data = $i;
            let i_type = type_of(&i_data);
            let tmp_i = match i_type {
                "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                    let mut v_r = i_data.to_string().as_str().replace("\\", "\\\\");
                    v_r = v_r.replace("\"", "\\\"");
                    "\"".to_string() + &v_r + "\""
                },
                "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                "&f32" | "&f64" | "&bool" => {
                    i_data.to_string() + ""
                },
                _ => {
                   "".to_string()
                },
            };


            let mut temp_s = String::from("");
            $(
                let temp_v = $v;
                let v_type = type_of(&temp_v);
                let value;
                if temp_v.to_string().as_str() == "null" {
                    value = "NULL,".to_string();
                } else {
                    value = match v_type {
                        "&&str" | "&alloc::string::String" | "&&alloc::string::String" => {
                            let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                            v_r = v_r.replace("\"", "\\\"");
                            "\"".to_string() + &v_r + "\","
                        },
                        "&u8" | "&u16" | "&u32" | "&u64" | "&usize" |
                        "&i8" | "&i16" | "&i32" | "&i64" | "&isize" |
                        "&f32" | "&f64" | "&bool" => {
                            temp_v.to_string() + ","
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

            let sql: String = "(UPDATE ".to_string() + $t + " SET " + temp_s.as_str()
                + " WHERE id=" + tmp_i.as_str() + ")";

            sql
        }
    };
}
