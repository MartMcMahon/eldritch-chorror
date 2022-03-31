use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Chore {
    id: i32,
    text: String,
    rarity: Rarity,
}

#[derive(Debug, PartialEq, Eq)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Spicy,
}

fn main() {}

#[test]
fn it_works() {
    let opts = Opts::from_url("mysql://root:@localhost:3306").unwrap();
    let pool = Pool::new(opts).unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(r"CREATE DATABASE IF NOT EXISTS eldritch_chorror")
        .unwrap();
    conn.query_drop(r"USE eldritch_chorror").unwrap();

    match conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS chores ( INT NOT NULL id,
    //      PRIMARY KEY id)",
        // )
        // .unwrap();
        // text TEXT NOT NULL,
        // rarity ENUM('c','u','r','s'),
        // ),
    ) {
        Ok(res) => {
            println!("{:#?}", res);
        }

        _ => {
            // println!("{:#?}", res);
        }
    };

    let x = conn.query_iter(r"SELECT * FROM chores").unwrap();

    println!("{:#?}", x);

    // conn.query_drop(
    //     r"CREATE TABLE IF NOT EXISTS chores (
    //     id INT NOT NULL,
    //     text TEXT NOT NULL,
    //     rarity ENUM('c','u','r','s'),
    //     PRIMARY KEY id
    // )",
    // )
    // .unwrap();

    assert_eq!(2 + 2, 4);
}
