

### mysql 数据库连接方法封装  
  
|  方法   | 说明  |
|  ----  | ----  |
| count  | 返回计数 |
| del  | 删除一条数据 |
| find  | 查寻数据 |
| get  | 查寻一条数据 |
| set  | 新增一条数据 |
| setmany  | 批量新增数据 |
| update  | 更新一条数据 |

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
let id = run_drop(&mut conn, set!("feedback", {
    "content": "ADFaadf",
    "uid": 9,
})).unwrap();

// 删除一条数据
run_drop(&mut conn, del!("feedback", 50)).unwrap();

// 更新一条数据
run_drop(&mut conn, update!("feedback", 56, {
    "content": "更新后的内容，一一一一"
})).unwrap();

// 批量 新增数据
let msql = setmany!("feedback", [
    {"uid": 1, "content": "批量更新00adf"},
    {"uid": 2, "content": "2342341"},
    {"uid": 3, "content": "mmmmm"},
    {"uid": 4, "content": "zzzzzz"},
    {"uid": 5, "content": "奔苦asda工工"},
    {"uid": 6, "content": "555"}
]);
run_drop(&mut conn, msql).unwrap();

// 获取一条数据
let sql1 = get!("feedback", 33, "id as id, feedback.content as cc");
#[derive(Serialize, Deserialize, Debug)]
struct Feedback {
    id: u64,
    cc: String
}
let res_get: (Vec<Feedback>, Option<(u64, String)>) = run(&mut conn, sql1).unwrap();

// 查寻数据
let sql_f = find!("feedback", {
    p0: ["uid", ">", 330],
    r: "p0",
    select: "id, content as cc",
});
let res_find: (Vec<Feedback>, Option<(u64, String)>) = run(&mut conn, sql_f).unwrap();

// 获取计数
let res_count: (Vec<u64>, Option<u64>) = run(&mut conn, count!("feedback", {})).unwrap();


```


### mysql 事务示例  
run_tran、run_tran_drop  
```rust
use mysql_quick::{TxOpts, MY_EXCLUSIVE_LOCK, MY_SHARED_LOCK};

let mut conn = mysql_conn();
// ---- 事务开始 ----
let mut tran = conn.start_transaction(TxOpts::default()).unwrap();
let getsql = get!("feedback", 59, "id,num,content,created_at") + MY_EXCLUSIVE_LOCK;  // 加锁操作，
let get_data: (Vec<SomeThing>, Option<(u64,u64,String,String)>) = run_tran(&mut tran, getsql).unwrap();

let tmp = get_data.0;
if tmp.len() == 0 {
    // 回滚事务
    tran.rollback().unwrap();
} else {
    if tmp[0].num <= 0 {
        // 回滚事务
        tran.rollback().unwrap();
    } else {
        let sql2 = update!("feedback", 59, {"num": ["incr", -1]});
        run_tran_drop(&mut tran, sql2).unwrap();
        // 提交事务
        tran.commit().unwrap();
    }
}
// ---- 事务结束 ----


```