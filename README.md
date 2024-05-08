

### mysql 数据库连接方法封装
```rust
use mysql_quick::{MysqlQuick, run, find ...};
pub fn mysql_conn() -> PooledConn {
    let conn = MysqlQuick::new("mysql://root:12345678@localhost:3306/dev_db").unwrap().pool.get_conn().unwrap();
    conn
}
let mut conn = mysql_conn();
```


### mysql 查寻方法

|  运行sql   | 说明  |
|  ----  | ----  |
| my_run_vec  | 执行sql，返回vec类型数据，无数据则返回`vec![]` |
| my_run_drop  | 执行sql，无返回数据，最多返回id |
| my_run_tran_vec  | 事务执行sql，有返回vec类型数据，无数据则返回`vec![]` |
| my_run_tran_drop  | 事务执行sql，无返回数据，最多返回id |

```rust
let id: u64 = my_run_drop(&mut conn, sql).unwrap();

// 执行 sql 语句
let data: Vec<serde_json::Value> = my_run_vec(&mut conn, sql).unwrap();
```



### sql快捷生成

|  sql快捷生成方法   | 说明  |
|  ----  | ----  |
| mycount  | 返回计数的sql |
| mydel  | 删除一条数据的sql |
| mydelmany  | 批量删除数据的sql |
| myfind  | 查寻数据的sql |
| myget  | 查寻一条数据的sql |
| myset  | 新增一条数据的sql |
| mysetmany  | 批量新增数据的sql |
| myupdate  | 更新一条数据的sql |
| myupdatemany  | 批量更新数据的sql |
| 自定义  | 可以直接写自己的sql语句 |


以下内容，则为常用sql的快捷方法
```rust

// 新增一条数据
let id = my_run_drop(&mut conn, myset!("for_test", {
    "content": "ADFaadf",
    "uid": 9,
    "info": if let Some(a) = one_info {a} else {"null"},
})).unwrap();

// 删除一条数据
my_run_drop(&mut conn, mydel!("for_test", 50)).unwrap();

// 更新一条数据
my_run_drop(&mut conn, myupdate!("for_test", 56, {
    "content": "更新后的内容，一一一一"
})).unwrap();

// 批量 新增数据
let msql_2 = mysetmany!("for_test", vec![
    Item {uid: 1, content: "批量更新00adf"},
    Item {uid: 2, content: "2342341"},
    Item {uid: 3, content: "mmmmm"},
])
my_run_drop(&mut conn, msql).unwrap();

// 批量 更新数据
let sql = myupdatemany!("for_test", "uid", vec![
    Item {uid: 1, content: "批量更新00adf"},
    Item {uid: 2, content: "2342341"},
])
my_run_drop(&mut conn, sql).unwrap();



// 获取一条数据
let sql1 = myget!("for_test", 33, "id, content as cc");
#[derive(Serialize, Deserialize, Debug)]
struct Feedback {
    id: u64,
    cc: String
}
let res_get: Vec<Feedback> = my_run_vec(&mut conn, sql1).unwrap();

// 查寻数据
let sql_f = myfind!("for_test", {
    p0: ["uid", ">", 330],
    r: "p0",
    select: "*",
});
let res_find: Vec<Feedback> = my_run_vec(&mut conn, sql_f).unwrap();

// 获取计数
let res_count: Vec<MysqlQuickCount> = my_run_vec(&mut conn, mycount!("for_test", {})).unwrap();

// 自定义查寻
let list: Vec<serde_json::Value> =
    my_run_vec(&mut conn, "select distinct type_v3 from dishes".to_owned()).unwrap();

```


### mysql 事务示例
my_run_tran_vec、my_run_tran_drop
```rust
use mysql_quick::{TxOpts, MY_EXCLUSIVE_LOCK, MY_SHARED_LOCK};

let mut conn = mysql_conn();
// ---- 事务开始 ----
 let mut tran = conn.start_transaction(TxOpts::default()).unwrap();
let getsql = myget!("for_test", 5, "id,title,content,price,total,uid") + MY_EXCLUSIVE_LOCK;
let get_data: Vec<ForTestItem> = my_run_tran_vec(&mut tran, getsql).unwrap();
let tmp = get_data.0;
if tmp.len() == 0 {
    tran.rollback().unwrap();
} else {
    if tmp[0].total <= 0 {
        tran.rollback().unwrap();
    } else {
        let sql2 = myupdate!("for_test", 5, {"total": ["incr", -1]});
        my_run_tran_drop(&mut tran, sql2).unwrap();
        tran.commit().unwrap();
    }
}
// ---- 事务结束 ----


```
