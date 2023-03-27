

### mysql 数据库连接方法封装  
  
|  方法   | 说明  |
|  ----  | ----  |
| mycount  | 返回计数 |
| mydel  | 删除一条数据 |
| myfind  | 查寻数据 |
| myget  | 查寻一条数据 |
| myset  | 新增一条数据 |
| mysetmany  | 批量新增数据 |
| myupdate  | 更新一条数据 |

依赖：
```toml
mysql = "23.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
regex = "1.7"
```

### mysql 查寻示例  
run、run_drop  
```rust
use mysql_quick::{MysqlQuick, run, find ...};
pub fn mysql_conn() -> PooledConn {
    let conn = MysqlQuick::new(1, 10, "mysql://root:12345678@localhost:3306/dev_db").unwrap().pool.get_conn().unwrap();
    conn
}
let mut conn = mysql_conn();

// 新增一条数据
let id = my_run_drop(&mut conn, myset!("for_test", {
    "content": "ADFaadf",
    "uid": 9,
})).unwrap();

// 删除一条数据
my_run_drop(&mut conn, mydel!("for_test", 50)).unwrap();

// 更新一条数据
my_run_drop(&mut conn, myupdate!("for_test", 56, {
    "content": "更新后的内容，一一一一"
})).unwrap();

// 批量 新增数据
let msql = mysetmany!("for_test", [
    {"uid": 1, "content": "批量更新00adf"},
    {"uid": 2, "content": "2342341"},
    {"uid": 3, "content": "mmmmm"},
    {"uid": 4, "content": "zzzzzz"},
    {"uid": 5, "content": "奔苦asda工工"},
    {"uid": 6, "content": "555"}
]);
my_run_drop(&mut conn, msql).unwrap();

// 获取一条数据
let sql1 = myget!("for_test", 33, "id as id, feedback.content as cc");
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
    select: "id, content as cc",
});
let res_find: Vec<Feedback> = my_run_vec(&mut conn, sql_f).unwrap();

// 获取计数
let res_count: Vec<u64> = my_run_vec(&mut conn, mycount!("for_test", {})).unwrap();


```


### mysql 事务示例  
run_tran、run_tran_drop  
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