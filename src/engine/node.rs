#[derive(Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub body: Vec<Body>
}

impl Node {
    pub fn new(name: String) -> Node {
        Self {
            id: 0,
            name,
            body: Vec::new()
        }
    }
    pub fn get_child_by_id(&self, id: i32) -> Option<Node> {
        for i in self.clone().body {
            match i {
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