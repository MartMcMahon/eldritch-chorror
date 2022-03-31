use mysql::prelude::*;
use mysql::*;

fn main() {}

#[test]
fn it_works() {
    #[derive(Debug, PartialEq, Eq)]
    struct Payment {
        customer_id: i32,
        amount: i32,
        account_name: Option<String>,
    }

    let opts = Opts::from_url(
        "mysql://eldritch_chorror:af8287e3e2064f03bcd0584f0d01729a@wonk.gg:3306/eldritch_chorror",
    )
    .unwrap();
    let pool = Pool::new(opts).unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(
        r"CREATE TABLE chores (
        id INT NOT NULL,
        text TEXT NOT NULL,
        rarity ENUM('c','u','r','s'),
        PRIMARY KEY id
    );",
    )
    .unwrap();
    assert_eq!(2 + 2, 4);
}
