mod method;
pub use method::*;

#[cfg(test)]
mod test {
    use crate::{
        my_run_vec, mycount, myfind, myget, mysetmany, myupdatemany, MysqlQuick, MysqlQuickCount,
    };
    use serde::{Deserialize, Serialize};

    const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";

    #[test]
    fn test_one() {
        let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();

        let sql = mycount!("for_test", {
            p0: ["total", ">", 0],
            r: "p0",
        });
        let res_count: Vec<MysqlQuickCount> = my_run_vec(&mut conn, sql).unwrap();

        println!("测试》》》{:?}", res_count);

        let des_str = r#"#@!@$$^&^%&&#\\,abc,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',ddd"#;
        let _sql = mycount!("for_test", {
            p0: ["content", "in", des_str],
            r: "p0",
        });
        let _sql = mycount!("for_test", {
            p0: ["total", ">", 0],
            r: "p0",
            distinct: "title",
        });

        let _sql = myfind!("for_test", {
            p0: ["total", ">", 1],
            r: "p0",
            select: "distinct title as t",
        });
        let _sql = myget!("for_test", {"uid": 1});
        let _sql = myget!("for_test", 5, "id,content as cc"); // 查寻 id = 5 的数据

        let info =
            r#"m'y,,a#@!@$$33^&^%&&#\\ \ \ \ \ \ \ \\\\\$,,adflll+_)"(_)*)(32389)d(ŐдŐ๑)🍉 .',"#;
        #[derive(Serialize, Deserialize)]
        struct Item {
            content: String,
            total: u32,
            price: Option<f32>,
        }
        let vec_data = vec![
            Item {
                content: info.to_owned(),
                total: 10,
                price: Some(30.5),
            },
            Item {
                content: String::from("批量新增"),
                total: 11,
                price: None,
            },
        ];
        let _sql = mysetmany!("for_test", vec_data);

        #[derive(Serialize, Deserialize)]
        struct Item2 {
            id: u64,
            content: String,
            total: u32,
        }
        let vec_data = vec![
            Item2 {
                id: 1,
                content: "ABC".to_string(),
                total: 1,
            },
            Item2 {
                id: 2,
                content: String::from("批量更新2111"),
                total: 1,
            },
        ];
        // 当前以 id 字段为查寻条件，更新 id 分别为7、8数据的content、total为对应的值。
        let _sql = myupdatemany!("for_test", "id,+total", vec_data);

        let sql = myupdatemany!("for_test", "id,total", vec_data);
        println!("sql》》》{}", sql);

        assert!(true)
    }
}
