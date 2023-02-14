

#[macro_use]
mod count;

#[macro_use]
mod del;

#[macro_use]
mod find;

#[macro_use]
mod get;

#[macro_use]
mod set;

#[macro_use]
mod setmany;

#[macro_use]
mod update;


mod method;
pub use method::*;


/// 常用的 mysql 锁类型。直接加在 sql 语句后面
pub const MY_SHARED_LOCK: &str = " LOCK IN SHARE MODE";
/// 悲观锁，用于抢单什么的
pub const MY_EXCLUSIVE_LOCK: &str = " FOR UPDATE";
