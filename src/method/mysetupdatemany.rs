///
/// 批量新增或更新数据 ，返回 sql 语句。
/// 第二个参数为指定 不重复的字段。
/// 如果第二个参数里的字段同时存在，则更新其他字段的值，否则新增。
/// ```
/// struct Item {
///     content: String,
///     total: u32,
///     price: Option<f32>,
/// }
/// let vec_data = vec![
///     Item {
///         content: "null".to_string(), // NULL 值
///         total: 11,
///         price: None, // DEFAULT 值
///     },
///     Item {
///         content: info.to_owned(),
///         total: 10,
///         price: Some(30.5), // 30.5
///     },
/// ];
/// let sql = mysetupdatemany!("for_test", "price", vec_data);
/// my_run_drop(&mut conn, sql).unwrap();
/// ```
#[macro_export]
macro_rules! mysetupdatemany {
    ($t:expr, $i:expr, $v: expr) => {{
        let mut field_name = " (".to_string();
        let mut value = "".to_string();
        let mut values_upd = String::from("");
        let unique_keys = $i.split(",").collect::<Vec<&str>>();
        for i in 0..$v.len() {
            let item_str = $crate::to_string(&$v[i]).unwrap();
            let o: $crate::Value = $crate::from_str(&item_str).unwrap();
            value = value + " (";
            for key in o.as_object().unwrap().keys() {
                if i == 0 {
                    field_name = field_name + &key + ",";
                    if !unique_keys.contains(&key.as_str()) {
                        values_upd = values_upd + &key + " = VALUES(" + &key + "),";
                    };
                }
                let temp_v = &o[key];
                if (temp_v.is_number()) {
                    value = value + temp_v.to_string().as_str() + ",";
                } else if temp_v.is_null() {
                    value = value + "DEFAULT,";
                } else if temp_v.is_string() {
                    let t_v = temp_v.as_str().unwrap();
                    if t_v == "null" {
                        value = value + "NULL,";
                    } else {
                        let mut v_r = t_v.to_string().as_str().replace("\\", "\\\\");
                        v_r = v_r.replace("\"", "\\\"");
                        value = value + "\"" + &v_r + "\","
                    }
                }
            }
            if i == 0 {
                field_name.pop();
                field_name = field_name + ")";
            }
            value.pop();
            value = value + "),";
        }

        value.pop();
        values_upd.pop();

        let sql: String = "INSERT INTO ".to_string()
            + $t
            + field_name.as_str()
            + " VALUES"
            + value.as_str()
            + " ON DUPLICATE KEY UPDATE "
            + values_upd.as_str();

        sql
    }};
}
