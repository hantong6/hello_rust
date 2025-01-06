use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

pub fn exec() {
    test_closure();
}

struct PageCache<T, X, Y> where T: Fn(X, Y) -> String, X: Copy + Eq + Hash + Display, Y: Copy + Eq + Hash + Display {
    query: T,
    value: HashMap<X, HashMap<Y, String>>
}

impl<T, X, Y> PageCache<T, X, Y> where T: Fn(X, Y) -> String, X: Copy + Eq + Hash + Display, Y: Copy + Eq + Hash + Display {
    fn new(query: T) -> PageCache<T, X, Y> {
        PageCache {
            query,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, user_id: X, article_id: Y) -> String {
        match self.value.get_mut(&user_id) {
            Some(v) => {
                match v.get(&article_id) {
                    Some(r) => {
                        println!("page cache exist, get by user_id: {}, article_id: {}, value: {}", user_id, article_id, r);
                        String::from(r)
                    },
                    None => {
                        let r = (self.query)(user_id, article_id);
                        v.insert(article_id, r.clone());
                        println!("page cache not exist, render by user_id: {}, article_id: {}, value: {}", user_id, article_id, r);
                        r
                    }
                }
            },
            None => {
                let r = (self.query)(user_id, article_id);
                let mut v = HashMap::new();
                v.insert(article_id, r.clone());
                self.value.insert(user_id, v);
                println!("page cache not exist, render by user_id: {}, article_id: {}, value: {}", user_id, article_id, r);
                r
            }
        }
    }
}

fn test_closure() {
    let mut cache = PageCache::new(|x, y| format!("user_id: {}, article: {}", x, y));

    let v1 = cache.value(1, "a");
    let v2 = cache.value(2, "b");
    let v1 = cache.value(1, "a");

}