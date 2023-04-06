/// 批量新增数据 ，返回 sql 语句。
/// 下面示例中，user 为表名，，name、num 为字段名，，后面为新增的值。
///
/// ```
/// #[derive(Serialize, Deserialize)]
/// struct Item {
///     content: String,
///     total: u32,
/// }
/// let vec_data = vec![
///     Item {content: String::from("aaa"), total: 12},
///     Item {content: String::from("bb"), total: 1},
/// ];
/// let sql = mysetmany!("content", vec_data);
/// ```
#[macro_export]
macro_rules! mysetmany {
    // ($t:expr, [$({$($k:tt: $v:expr),+$(,)?}),+$(,)?]) => {
    //     {
    //         fn type_of<T>(_: T) -> &'static str {
    //             std::any::type_name::<T>()
    //         }
    //         let mut keys = String::from("");
    //         let mut values = String::from("");
    //         $(
    //             if keys == "".to_string() {
    //                 $(
    //                     keys = keys + $k + ",";
    //                 )+
    //             }
    //             values = values + " ( ";
    //             $(
    //                 let temp_v = $v;
    //                 let v_type = type_of(&temp_v);
    //                 values = match v_type {
    //                     "&&str" => {
    //                         let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
    //                         v_r = v_r.replace("\"", "\\\"");
    //                         values + "\"" + &v_r + "\","
    //                     },
    //                     "&alloc::string::String" => {
    //                         let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
    //                         v_r = v_r.replace("\"", "\\\"");
    //                         values + "\"" + &v_r + "\","
    //                     },
    //                     "&&alloc::string::String" => {
    //                         let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
    //                         v_r = v_r.replace("\"", "\\\"");
    //                         values + "\"" + &v_r + "\","
    //                     },
    //                     _ => {
    //                         values + temp_v.to_string().as_str() + ","
    //                     }
    //                 };
    //             )+
    //             values.pop();
    //             values = values + " ),";
    //         )+

    //         keys.pop();
    //         values.pop();

    //         let sql: String = "INSERT INTO ".to_string() + $t + " ( " + keys.as_str() + " ) "
    //             + " VALUES " + values.as_str();

    //         sql
    //     }
    // };
    ($t:expr, $v: expr) => {{
        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut field_name = " (".to_string();
        let mut value = "".to_string();
        for i in 0..$v.len() {
            let mut item_str = serde_json::to_string(&$v[i]).unwrap();
            item_str.pop();
            item_str.remove(0);
            item_str.push(',');
            item_str.push('"');
            item_str.insert(0, ',');
            // ",\"content\":\"aaa\",\"total\":12,\"uid\":3,\"des\":\"nn\",\""
            value = value + " (";
            let re2 = regex::Regex::new("\":(.*?),\"").unwrap();
            for cap2 in re2.captures_iter(item_str.as_str()) {
                value = value + &cap2[1] + ",";
            }
            value.pop();
            value = value + "),";

            if i == 0 {
                let re = regex::Regex::new(",\"([0-9a-zA-Z_]+?)\":").unwrap();
                for cap in re.captures_iter(item_str.as_str()) {
                    field_name = field_name + &cap[1] + ",";
                }
                field_name.pop();
                field_name = field_name + ")";
            }
        }
        value.pop();
        let sql: String =
            "INSERT INTO ".to_string() + $t + field_name.as_str() + " VALUES" + value.as_str();

        sql
    }};
}
