use crate::engine::node::Node;

pub mod parser;
pub mod node;

#[derive(Debug)]
enum StateTag {
    Open(String),
    Close(String),
    CloseDouble(String),
    Unset
}


pub fn parse(str: String) {
    let mut nodes : Vec<Node> = Vec::new();
    let mut last = StateTag::Unset;
    let mut hoz_vec = 0;
    let mut root = Node::new(".".to_string());
    let mut current = Node::new(String::new());
    for i in str.chars() {
        match i  {
            '<' => {
                last = StateTag::Open(String::new());
                hoz_vec += 1;
            }

            '>' => {
                match &last {
                    StateTag::Open(e) => {
                        let name = e.clone();
                        last = StateTag::Close(name.clone());
                        current = Node::new(name.clone());
                        nodes.push(current);
                    }

                    StateTag::CloseDouble(e) => {
                        let name = e.clone();
                        last = StateTag::CloseDouble(name.clone());
                        current = Node::new(name.clone());
                        nodes.push(current);
                        last = StateTag::Unset;
                    }
                    _ => {}
                }
            }

            '/' => {
                last =  StateTag::CloseDouble(String::new());
                hoz_vec -= 2;
            },

            e => {
                match &last {
                    StateTag::Open(current) => {
                        last = StateTag::Open(format!("{}{}", current, e));
                    }
                    StateTag::Close(current) => {
                        match e {
                            '<' => {
                                last = StateTag::Open(String::new());
                            }

                            '>' => continue,
                            '/' => continue,
                            '\n' => continue,
                            _ => continue
                        }
                    }
                    StateTag::CloseDouble(current) => {
                        match e {
                            '\n' => continue,
                            _ => {last = StateTag::CloseDouble(format!("{}{}", current, e))}
                        }

                    }
                    StateTag::Unset => {}
                }
            }
        }
        println!("{} +> {:?}",hoz_vec, last);
    }

    println!("{:?}", nodes);
}