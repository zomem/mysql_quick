

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


mod method;
pub use method::*;


/// 常用的 mysql 锁类型。直接加在 sql 语句后面
pub const MY_SHARED_LOCK: &str = " LOCK IN SHARE MODE";
/// 悲观锁，用于抢单什么的
pub const MY_EXCLUSIVE_LOCK: &str = " FOR UPDATE";
