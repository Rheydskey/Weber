use crate::engine::StateTag;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub node_type: StateTag,
    pub body: Box<Body>
}

impl Node {
    pub fn new(name: String) -> Node {
        Self {
            id: 0,
            name,
            node_type: StateTag::Unset,
            body: Box::new(Body::Empty)
        }
    }
    pub fn new_with_type(name: String, node_type: StateTag) -> Node {
        Self {
            id: 0,
            name,
            node_type,
            body: Box::new(Body::Empty)
        }
    }

    pub fn get_body(&self) -> Body {
        self.body.as_ref().clone()
    }

    pub fn get_child_by_id(&self, id: i32) -> Option<Node> {
            match self.clone().get_body() {
                Body::Empty => {}
                Body::Text(_) => {}
                Body::Element(e) => {
                    if e.id == id {
                        return Some(e);
                    } else if let Some(e) = e.get_child_by_id(id) {
                        return Some(e);
                    } else {
                        return None;
                    }
                }
                Body::Elements(e) => {
                    for i in e {
                        if i.id == id {
                            return Some(i);
                        } else if let Some(e) = i.get_child_by_id(id) {
                            return Some(e);
                        } else {
                            return None;
                        }
                    }
                }
            }

        None
    }
}

#[derive(Debug, Clone)]
pub enum Body {
    Empty,
    Text(String),
    Element(Node),
    Elements(Vec<Node>)
}