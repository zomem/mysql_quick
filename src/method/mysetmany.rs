///
/// 批量新增数据 ，返回 sql 语句。
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mysql_quick::{mysetmany, my_run_drop, MysqlQuick, MysqlQuickCount};
/// # const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";
/// # let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();
/// # let info = r#"m'y,,a#@!@$$33^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// #[derive(Serialize, Deserialize)]
/// struct Item {
///     content: String,
///     total: u32,
///     price: Option<f32>,
/// }
/// let vec_data = vec![
///     Item {content: info.to_owned(), total: 10, price: Some(30.5)},
///     Item {content: String::from("批量新增"), total: 11, price: None}, // "null" 也表示 NULL
/// ];
/// let sql = mysetmany!("for_test", vec_data);
/// my_run_drop(&mut conn, sql).unwrap();
/// ```
#[macro_export]
macro_rules! mysetmany {
    ($t:expr, $v: expr) => {{
        let mut field_name = " (".to_string();
        let mut value = "".to_string();
        for i in 0..$v.len() {
            let item_str = $crate::to_string(&$v[i]).unwrap();
            let o: $crate::Value = $crate::from_str(&item_str).unwrap();
            value = value + " (";
            for key in o.as_object().unwrap().keys() {
                if i == 0 {
                    field_name = field_name + &key + ",";
                }
                let temp_v = &o[key];
                if (temp_v.is_number()) {
                    value = value + temp_v.to_string().as_str() + ",";
                } else if temp_v.is_null() {
                    value = value + "NULL,";
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
        let sql: String =
            "INSERT INTO ".to_string() + $t + field_name.as_str() + " VALUES" + value.as_str();

        sql
    }};
}
