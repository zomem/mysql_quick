

/// 新增数据 ，返回 sql 语句。
/// 下面示例中，user 为表名，，name、num 为字段名，，后面为新增的值。
/// Add new data, then return sql. exp: users table, field name and num. after them is value.
/// ```
/// let sql = set!("users", {
///     "name": string_t.clone(),
///     "num": 882,
/// });
/// run_set(sql).unwrap();
/// ```
#[macro_export] 
macro_rules! set {
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
                let temp_v = $v.clone();
                let v_type = type_of($v);
                values = match v_type {
                    "&str" => {
                        let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                        v_r = v_r.replace("\"", "\\\"");
                        values + "\"" + &v_r + "\","
                    },
                    "alloc::string::String" => {
                        let mut v_r = temp_v.to_string().as_str().replace("\\", "\\\\");
                        v_r = v_r.replace("\"", "\\\"");
                        values + "\"" + &v_r + "\","
                    },
                    _ => {
                        values + temp_v.to_string().as_str() + ","
                    }
                };
            )+
            
            keys.pop();
            values.pop();
    
            let sql: String = "INSERT INTO ".to_string() + $t + " ( " + keys.as_str() + " ) "
                + " VALUES ( " + values.as_str() + " )";
    
            sql
        }
    };
}
