/// 批量更新数据 ，返回 sql 语句。
/// ```
/// # use serde::{Deserialize, Serialize};
/// # use mysql_quick::{myupdatemany, my_run_drop, MysqlQuick, MysqlQuickCount};
/// # const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";
/// # let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();
/// # let info = r#"m'y,,a#@!@$$33^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
/// #[derive(Serialize, Deserialize)]
/// struct Item {
///     id: u64,
///     content: String,
///     total: Option<u32>,
/// }
/// let vec_data = vec![
///     Item {id: 1, content: "ABC".to_string(), total: Some(1)},
///     Item {id: 2, content: String::from("批量更新2111"), total: None},
/// ];
/// // 1.单个条件
/// // 当前以 id 字段为查寻条件，更新 id 分别为1、2数据的content、total为对应的值。
/// let sql = myupdatemany!("for_test", "id", vec_data);
/// my_run_drop(&mut conn, sql).unwrap();
///
/// // 2.多个条件
/// // 当前以 id && total 字段为查寻条件，更新满足 1 && 1 与 2 && 1 的数据content为对应的值。
/// let sql = myupdatemany!("for_test", "id,total", vec_data);
///
/// // 3.对特定字段进行原子性批量更新数据
/// // 如下，表示以 id 为查寻条件，total 字段要进行 incr 更新操作(注：total 不会作为查寻条件)。
/// let sql = myupdatemany!("for_test", "id,+total", vec_data);
/// ```
///
///
#[macro_export]
macro_rules! myupdatemany {
    ($t:expr, $i:expr, $v: expr) => {{
        let i_info = $i.clone();
        let i_vec: Vec<String> = i_info
            .split(",")
            .into_iter()
            .map(|info| info.to_string())
            .collect();

        let mut incr_field: Vec<String> = vec![];
        let mut query_field: Vec<String> = vec![];
        for m in 0..i_vec.len() {
            if i_vec[m].contains("+") {
                incr_field.push(i_vec[m].clone())
            } else {
                query_field.push(i_vec[m].clone())
            }
        }
        let i_data = query_field.join(",");
        // let i_data = "aa";
        // 中间生成的表名
        let table_upmj = $t.clone().to_owned() + "_upmj";
        let table = $t.clone().to_owned();

        let mut field_equl: Vec<String> = vec![];
        let mut select_vec: Vec<String> = vec![];

        for i in 0..$v.len() {
            let item_str = $crate::to_string(&$v[i]).unwrap();
            let o: $crate::Value = $crate::from_str(&item_str).unwrap();

            // SELECT  1 AS id, 11 AS code, 'nam' AS name, 44 AS book
            let mut field_list: Vec<&str> = vec![];
            let mut select_item: Vec<String> = vec![];

            for key in o.as_object().unwrap().keys() {
                if i == 0 {
                    field_list.push(&key);
                }

                let temp_v = &o[key];
                if (temp_v.is_number()) {
                    select_item.push(temp_v.to_string() + " AS " + &key);
                } else if temp_v.is_null() {
                    select_item.push("NULL".to_owned() + " AS " + &key);
                } else if temp_v.is_string() {
                    let t_v = temp_v.as_str().unwrap();
                    if t_v == "null" {
                        select_item.push("NULL".to_owned() + " AS " + &key);
                    } else {
                        let mut v_r = t_v.to_string().as_str().replace("\\", "\\\\");
                        v_r = v_r.replace("\"", "\\\"");
                        select_item.push("\"".to_string() + &v_r + "\"" + " AS " + &key);
                    }
                }
            }

            select_vec.push("SELECT ".to_string() + select_item.join(",").as_str());

            if i == 0 {
                field_equl = field_list
                    .iter()
                    .map(|x| {
                        let mut is_incr = false;
                        for c in 0..incr_field.len() {
                            if incr_field[c].contains(x) {
                                is_incr = true;
                                break;
                            }
                        }
                        if is_incr {
                            table.clone()
                                + "."
                                + x
                                + " = "
                                + table.clone().as_str()
                                + "."
                                + x
                                + " + "
                                + table_upmj.as_str()
                                + "."
                                + x
                        } else {
                            table.clone() + "." + x + " = " + table_upmj.as_str() + "." + x
                        }
                    })
                    .collect();
            }
        }

        let sql: String = "UPDATE ".to_string()
            + $t
            + " JOIN( "
            + select_vec.join(" UNION ").as_str()
            + " ) AS "
            + table_upmj.as_str()
            + " USING("
            + i_data.as_str()
            + ") SET "
            + field_equl.join(", ").as_str();

        sql
    }};
}
