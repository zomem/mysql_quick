

### 数据库连接
  
[node 版本](https://www.npmjs.com/package/access-db)  


依赖：
```toml
mysql = "22.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
regex = "1.6"
```
  
目前仅支持 mysql


### mysql 查寻示例  
my_run、my_run_id  
```rust
use access_db::{AccessMy, my_run, myfind ...};
pub fn mysql_conn() -> PooledConn {
    let conn = AccessMy::new(1, 10, "mysql://root:12345678@localhost:3306/dev_db").pool.get_conn().unwrap();
    conn
}
let mut conn = mysql_conn();

// 新增一条数据
let id = my_run_id(&mut conn, myset!("feedback", {
    "content": "ADFaadf",
    "uid": 9,
}));

// 删除一条数据
my_run_id(&mut conn, mydel!("feedback", 50));

// 更新一条数据
my_run_id(&mut conn, myupdate!("feedback", 56, {
    "content": "更新后的内容，一一一一"
}));

// 获取一条数据
let sql1 = myget!("feedback", 33, "id as id, feedback.content as cc");
#[derive(Serialize, Deserialize, Debug)]
struct Feedback {
    id: u64,
    cc: String
}
let res_get: (Vec<Feedback>, Option<(u64, String)>) = my_run(&mut conn, sql1);

// 查寻数据
let sql_f = myfind!("feedback", {
    p0: ["uid", ">", 330],
    r: "p0",
    page: 2,
    limit: 5,
    select: "id, content as cc",
});
let res_find: (Vec<Feedback>, Option<(u64, String)>) = my_run(&mut conn, sql_f);

// 获取计数
let res_count: (Vec<u64>, Option<u64>) = my_run(&mut conn, mycount!("feedback", {}));

// 批量 新增数据
let msql = mysetmany!("feedback", [
    {"uid": 1, "content": "批量更新00adf"},
    {"uid": 2, "content": "2342341"},
    {"uid": 3, "content": "mmmmm"},
    {"uid": 4, "content": "zzzzzz"},
    {"uid": 5, "content": "奔苦asda工工"},
    {"uid": 6, "content": "555"}
]);
my_run_id(&mut conn, msql);

```


### mysql 事务示例  
my_do、my_do_id  
```rust
use access_db::{TxOpts, MY_EXCLUSIVE_LOCK};

let mut conn2 = my_connect();
let mut tran2 = conn2.start_transaction(TxOpts::default()).unwrap();
let getsql = myget!("feedback", 59, "id,num,content,created_at") + MY_EXCLUSIVE_LOCK;  // 加锁操作，
let get_data: (Vec<SomeThing>, Option<(u64,u64,String,String)>) = my_do(&mut tran2, getsql);

let tmp = get_data.0;
if tmp.len() == 0 {
    // 回滚事务
    tran2.rollback().unwrap();
} else {
    if tmp[0].num <= 0 {
        // 回滚事务
        tran2.rollback().unwrap();
    } else {
        let sql2 = myupdate!("feedback", 59, {"num": ["incr", -1]});
        my_do_id(&mut tran2, sql2);
        // 提交事务
        tran2.commit().unwrap();
    }
}


```