pub fn exec() {
    map_init();
    or_insert();
    map_struct_key();
    map_move_ownership();
    count_words();
}

use std::collections::HashMap;
fn map_init() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];
    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }
    // 使用两种方法实现 team_map2
    // 提示:其中一种方法是使用 `collect` 方法
    // let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();
    let teams_map2: HashMap<&str, i32> = HashMap::from(teams);

    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}

fn or_insert() {
    // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
    let mut player_stats = HashMap::new();
    // 查询指定的 key, 若不存在时，则插入新的 kv 值
    player_stats.entry("health").or_insert(100);
    assert_eq!(player_stats["health"], 100);
    // 通过函数来返回新的值
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);
    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(*health, 100);
    *health -= 50;
    assert_eq!(*health, 50);
    println!("Success!");
    fn random_stat_buff() -> u8 {
        // 为了简单，我们没有使用随机，而是返回一个固定的值
        42
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Viking {
    name: String,
    country: String,
}
impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}
fn map_struct_key() {
    // 使用 HashMap 来存储 viking 的生命值
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // 使用 derive 的方式来打印 viking 的当前状态
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
fn map_move_ownership() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);
    let v2 = "hello";
    let mut m2 = HashMap::new();
    // 所有权在这里发生了转移
    m2.insert(v2, v1);
    assert_eq!(v2, "hello");
    println!("Success!")
}

fn count_words() {
    let content = "hello web3, hello rust";
    let content : Vec<_> = content.split(" ").collect();
    let mut count = HashMap::with_capacity(10);
    for word in content {
       let value = count.get(word);
        match value {
            Some(v) => count.insert(word, v + 1),
            None => count.insert(word, 1)
        };
    }
    println!("count: {:#?}", count)
}
