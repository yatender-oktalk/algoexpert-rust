#[allow(dead_code)]
pub struct Node<T: Ord> {
    pub data: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T: Ord> Node<T> {
    pub fn new(data: T) -> Option<Box<Self>> {
        Some(Box::new(Self {
            data: data,
            left: None,
            right: None,
        }))
    }
}
