use crate::engine::node::{Node, Body};

pub mod parser;
pub mod node;

#[derive(Debug, Clone)]
pub enum StateTag {
    Open(String),
    Close(String),
    CloseDouble(String),
    Unset
}


pub fn parse(str: String) {
    let mut nodes : Vec<Node> = Vec::new();
    let mut last = StateTag::Unset;
    let mut hoz_vec = 0;
    let mut current_node = Node::new(String::new());
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
                        current_node = Node::new_with_type(name.clone(), StateTag::Open(String::new()));
                        nodes.push(current_node.clone());
                    }

                    StateTag::CloseDouble(e) => {
                        let name = e.clone();
                        current_node = Node::new_with_type(name.clone(), StateTag::CloseDouble(String::new()));
                        nodes.push(current_node.clone());
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
                    StateTag::Close(_) => {
                        match e {
                            '<' => {
                                last = StateTag::Open(String::new());
                            }

                            '>' => continue,
                            '/' => continue,
                            '\n' | ' ' => continue,
                            e => {
                                if let Some(last_node) =nodes.last_mut() {
                                match last_node.clone().get_body() {
                                    Body::Empty => {
                                        if let Some(last_node) = nodes.last_mut() {
                                            last_node.body = Box::new(Body::Text(format!("{}", e)));
                                        }
                                    }
                                    Body::Text(string) => {
                                        if let Some(last_node) = nodes.last_mut() {
                                            last_node.body = Box::new(Body::Text(format!("{}{}", string, e)));
                                        }

                                    }
                                    _ => continue
                                }
                                }
                            }
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
        //println!("{} +> {:?} / {:?}",hoz_vec, last, current_node.clone());
    }

    println!("{:#?}", nodes);
}