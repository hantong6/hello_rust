use std::time::Instant;
use crate::box1::NodeType::{File, Folder};

pub fn exec() {
    box_array();
    test_file_system();
}

fn box_array() {
    let start = Instant::now();
    let arr_in_stack = [0; 100_000];
    println!("arr in stack cost: {:?}", start.elapsed()); //12.8us

    let start = Instant::now();
    let arr_in_heap = Box::new([0; 100_000]);
    println!("arr in heap cost: {:?}", start.elapsed()); //193.2us
}

enum NodeType {
    File, Folder
}

struct FolderNode {
    name: String,
    node_type: NodeType,
    children: Box<Vec<FolderNode>>
}

impl FolderNode {
    fn new(name: String, node_type: NodeType) -> FolderNode {
        FolderNode {
            name,
            node_type,
            children: Box::new(vec![])
        }
    }

    fn add_child(&mut self, folder_node: FolderNode) {
        self.children.push(folder_node);
    }

    fn get_child(&mut self, name: &str) -> Option<&mut FolderNode> {
        if self.children.is_empty() {
            return None;
        }
        for child in self.children.iter_mut() {
            if child.name == name {
                return Some(child);
            }
        }
        None
    }
}

trait FileSystem {
    fn create_file(&mut self, name: &str);
    fn create_folder(&mut self, name: &str);
    fn list_contents(&self);
}

impl FileSystem for FolderNode {

    fn create_file(&mut self, name: &str) {
        let node = FolderNode::new(name.to_string(), File);
        self.add_child(node);
    }

    fn create_folder(&mut self, name: &str) {
        let node = FolderNode::new(name.to_string(), Folder);
        self.add_child(node);
    }

    fn list_contents(&self) {
        println!("{} content is follow:", self.name);
        print_children(self, 0);

        fn print_children(node: &FolderNode, depth: usize) {
            if node.children.is_empty() { 
                return;
            }
            for child in node.children.iter() {
                match child.node_type {
                    File => println!("{}{}", "---".repeat(depth), child.name),
                    Folder => {
                        println!("{}{}", "---".repeat(depth), child.name);
                        print_children(child, depth + 1);
                    }
                }
            }
        }
    }
}

fn test_file_system() {
    let mut folder_node = FolderNode::new("init".to_string(), Folder);
    folder_node.create_file("111");
    folder_node.create_file("222");
    folder_node.create_folder("333");
    let mut folder333 = folder_node.get_child("333").unwrap();
    folder333.create_file("3331");
    folder333.create_folder("3332");
    folder333.create_file("3333");
    let mut folder3333 = folder333.get_child("3332").unwrap();
    folder3333.create_file("33321");
    folder3333.create_file("33322");
    folder_node.list_contents();
}
