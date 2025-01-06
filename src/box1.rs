use std::cmp::PartialEq;
use std::time::Instant;
use serde_json::Value::String;
use crate::box1::NodeType::{File, Folder};

pub fn exec() {
    box_array();

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
}

trait FileSystem {
    fn create_file(parent: &mut FolderNode, name: &str);
    fn create_folder(parent: &mut FolderNode, name: &str);
    fn list_contents(parent: &FolderNode);
}

impl FileSystem for FolderNode {

    fn create_file(parent: &mut FolderNode, name: &str) {
        let node = FolderNode::new(name.to_string(), File);
        parent.add_child(node);
    }

    fn create_folder(parent: &mut FolderNode, name: &str) {
        let node = FolderNode::new(name.to_string(), Folder);
        parent.add_child(node);
    }

    fn list_contents(parent: &FolderNode) {
        println!("{} content is follow:", parent.name);
        print_children(parent, 0);

        fn print_children(node: &FolderNode, depth: usize) {
            if node.children.is_empty() { 
                return;
            }
            for child in node.children.iter() {
                match child.node_type {
                    File => println!("{}{}", " ".repeat(depth), child.name),
                    Folder => print_children(child, depth + 1)
                }
            }
        }
    }
}

fn test_file_system() {
    let folder_node = FolderNode::new("init", Folder);
    
}
