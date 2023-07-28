use serde::{Deserialize, Serialize};

#[macro_use]
mod mycount;

#[macro_use]
mod mydel;

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
