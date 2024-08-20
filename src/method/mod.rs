use serde::{Deserialize, Serialize};

#[macro_use]
mod mycount;

#[macro_use]
mod mydel;

#[macro_use]
mod mydelmany;

#[macro_use]
mod myfind;

#[macro_use]
mod myget;

#[macro_use]
mod myset;

#[macro_use]
mod mysetmany;

#[macro_use]
mod myupdate;

#[macro_use]
mod myupdatemany;

mod method;
pub use method::*;

/// 常用的 mysql 锁类型。直接加在 sql 语句后面
pub const MY_SHARED_LOCK: &str = " LOCK IN SHARE MODE";
/// 悲观锁，用于抢单什么的
pub const MY_EXCLUSIVE_LOCK: &str = " FOR UPDATE";

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct MysqlQuickCount {
    pub mysql_quick_count: u64,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_myset() {
        let name = Some("a");
        let name2: Option<String> = None;
        let aa = r#"m'y,,a#@!@$$^&^%&&#$,,adflll+_)"(_)*)(32389)"#;
        let sql = myset!("table", {
           "id": 23,
           "nick": aa,
           "name": name,
           "name2": name2,
           "age": Some(33),
           "empty": Some(""),
           "empty2": "",
        });
        assert_eq!(
            r#"INSERT INTO table ( id,nick,name,name2,age,empty,empty2 )  VALUES ( 23,"m'y,,a#@!@$$^&^%&&#$,,adflll+_)\\\"(_)*)(32389)","a",NULL,33,"","" )"#,
            sql
        )
    }
    #[test]
    fn test_myupdate() {
        let name = Some("a");
        let name2: Option<String> = None;
        let aa = r#"m'y,,a#@!@$$^&^%&&#$,,adflll+_)"(_)*)(32389)"#;
        let sql = myupdate!("table", 32, {
           "id": 23,
           "nick": aa,
           "name": name,
           "name2": name2,
           "age": Some(33),
           "empty": Some(""),
           "empty2": "",
        });
        println!("{}", sql);
        assert_eq!(
            r#"UPDATE table SET id=23,nick="m'y,,a#@!@$$^&^%&&#$,,adflll+_)\\\"(_)*)(32389)",name="a",name2=NULL,age=33,empty="",empty2="" WHERE id=32"#,
            sql
        )
    }
}
