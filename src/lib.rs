mod method;
pub use method::*;

pub use regex::Regex;
pub use serde_json::{Value, from_str, to_string};

#[cfg(test)]
mod test {
    use crate::{
        MysqlQuick, MysqlQuickCount, Sql, my_run_vec, mycount, mydel, myfind, myget, myset,
        mysetmany, myupdate, myupdatemany,
    };
    use serde::{Deserialize, Serialize};

    const MYSQL_URL: &str = "mysql://root:12345678@localhost:3306/dev_db";

    #[test]
    fn test_one() {
        let mut conn = MysqlQuick::new(MYSQL_URL).unwrap().pool.get_conn().unwrap();
        let sql = mydel!("for_test", 12);
        println!("sql》》》{:?}", sql);

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

    #[test]
    fn test_complex() {
        let sql = myfind!("for_test", {
            j0: ["user_id", "inner", "dev.users.id as u"],
            p0: ["content", "=", "内容"],
            p1: ["uid", "=", "1"],
            r: "p0 && p1",
        });
        println!("@@@ _1__ {}", sql);
        let sql = myfind!("for_test", {
            p0: ["content", "=", r#"' OR '1'='1'; --"#],
            p1: ["uid", "=", "1"],
            r: "p0 && p1",
        });
        println!("@@@ _2__ {}", sql);

        let sql1 = myfind!("hospital", {
            p0: ["hospital_name", "like", "院%"],
            r: "p0",
            select: "hospital_id",
        });
        let sql2 = mycount!("patient", {
            p0: ["investigation_id", "=", Sql("investigation.investigation_id")],
            r: "p0",
        });
        let sql3 = mycount!("delete_patient", {
            p0: ["investigation_id", "=", Sql("investigation.investigation_id")],
            r: "p0",
        });
        let sql = myfind!("investigation", {
            j1: ["hospital_id", "inner", "hospital.hospital_id"],
            p0: ["hospital_id", "in", Sql(sql1)],
            p1: ["inv_type", "=", "门诊"],
            r: "p0 && p1",
            select: "investigation_id, hospital_id, hospital.hospital_name, status_op_dateTime, (".to_string()
                + sql2.as_str() + ") as patient_count, ("
                + sql3.as_str() + ") as delete_patient_count",
        });
        println!("sql>>>>>  {} \n", sql);

        assert_eq!(
            sql,
            r#"SELECT investigation.investigation_id,investigation.hospital_id,hospital.hospital_name,investigation.status_op_dateTime,(SELECT count(*) as mysql_quick_count FROM patient WHERE patient.investigation_id = (investigation.investigation_id)) as patient_count,(SELECT count(*) as mysql_quick_count FROM delete_patient WHERE delete_patient.investigation_id = (investigation.investigation_id)) as delete_patient_count FROM investigation INNER JOIN hospital ON investigation.hospital_id = hospital.hospital_id WHERE (investigation.hospital_id IN (SELECT hospital.hospital_id FROM hospital WHERE hospital.hospital_name LIKE "院%" ) AND investigation.inv_type = "门诊") "#
        );
    }

    #[test]
    fn test_set() {
        let sql = myset!("for_test", {
            "name":  "wzj" ,
            "age":  32,
            "name": &Some("wzj"),
            "age": &Some(32),
        });
        println!("@@@ _1__ {}", sql);

        // #[derive(Debug)]
        // struct Userbc {
        //     phone: String,
        //     c: u32,
        // }
        // let sql = myset!("for_test", {
        //     "name": Some(Userbc{phone: "select".to_owned(), c: 3}),
        //     "age": Some((32,"select")),
        // });
        // INSERT INTO for_test ( name,age )  VALUES ( "serbc { phone: \"select\", c: 3 ",(32, "select") )
        // INSERT INTO for_test ( name,age )  VALUES ( "serbc { phone: \"select\", c: 3 ","32, \"select\"" )
        println!("@@@ 11111__ {}", sql);

        let s = 2;
        let sql = myupdate!("sku_unit", {"unit_sn": 100000}, {
            "status": &s
        });
        println!("{}", sql);

        let sql = myset!("talbe", {
           "name": r#"m'y,,a#@!@$$^&^%&&#$,,adflll+_)"(\_)*)(32389)d(ŐдŐ๑)🍉 .',"#,
           "b": Some(r#"m'y,,a#@!@$$^&^%&&#$,,adflll+_)"(\_)*)(32389)d(ŐдŐ๑)🍉 .',"#),
           "cb": "null",
        });
        println!("sql,,,  {}", sql);
    }

    #[test]
    fn test_option() {
        let age: Option<u32> = None;
        let sql = myset!("for_test", {
            "name":  "wzj" ,
            "age":  age,
            "name2": &Some("wzj"),
            "age2": age,
            "content": "null"
        });
        println!("sql__ {}", sql);

        let sql = myupdate!("for_test", {"cid": 3}, {
            "name": ["set", "wzj"],
            "age": ["set", age],
            "name2":["set", &Some("wzj")],
            "age2": ["incr", Some(32)],
            "content": ["set", "null"]
        });
        println!("sql22__ {}", sql);

        let sql = myupdate!("for_test", 3, {
            "name": ["set", "wzj"],
            "age": ["set", age],
            "name2":["set", &Some("wzj")],
            "age2": ["incr", Some(32)],
            "content": ["set", "null"]
        });
        println!("sql22__ {}", sql);

        let sql = myupdate!("for_test", {"cid": 3}, {
            "name": "wzj",
            "age": age,
            "name2": &Some("wzj"),
            "age2": Some(32),
            "content": "null"
        });
        println!("sql22__ {}", sql);

        let sql = myupdate!("for_test", 3, {
            "name": "wzj",
            "age": age,
            "name2": &Some("wzj"),
            "age2": Some(32),
            "content": "null"
        });
        println!("sql22__ {}", sql);

        #[derive(Serialize, Deserialize)]
        struct Item {
            content: String,
            total: u32,
            price: Option<f32>,
        }
        let vec_data = vec![
            Item {
                content: String::from("批量22新增"),
                total: 10,
                price: Some(30.5),
            },
            Item {
                content: "null".to_string(),
                total: 11,
                price: None,
            },
        ];
        let sql = mysetmany!("for_test", vec_data);
        println!("sql__ {}", sql);

        #[derive(Serialize, Deserialize)]
        struct Item2 {
            id: u64,
            content: Option<String>,
            total: u32,
        }
        let vec_data = vec![
            Item2 {
                id: 1,
                content: Some("ABC".to_string()),
                total: 1,
            },
            Item2 {
                id: 2,
                content: Some("null".to_string()),
                total: 1,
            },
        ];
        // 当前以 id 字段为查寻条件，更新 id 分别为7、8数据的content、total为对应的值。
        let sql = myupdatemany!("for_test", "id,+total", vec_data);
        println!("\nsql_aa_ {}", sql);

        let sql = myupdatemany!("for_test", "id,total", vec_data);
        println!("\nsql_bb_ {}", sql);
    }
}
