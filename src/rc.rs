use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn exec() {
    test_friend();
}

struct User {
    name: String,
    friends: RefCell<Vec<Weak<User>>>
}

impl User {

    fn new(name: String) -> Rc<Self> {
        Rc::new(User {
            name,
            friends: RefCell::new(Vec::new())
        })
    }

    fn add_friend(user: &Rc<User>, friend: &Rc<User>) {
        user.friends.borrow_mut().push(Rc::downgrade(friend));
        friend.friends.borrow_mut().push(Rc::downgrade(user));
    }

    fn show_friends(&self) {
        print!("{} has friends as follow:", self.name);
        for friend in self.friends.borrow().iter() {
            println!("{}", friend.upgrade().unwrap().name)
        }
    }
}

fn test_friend() {
    let hantong6 = User::new("hantong6".to_string());
    let hantong8 = User::new("hantong8".to_string());

    User::add_friend(&hantong6, &hantong8);

    hantong6.show_friends();
    hantong8.show_friends();

}
