struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(x) = self.head.as_deref() {
            Some(&x.elem)
        } else {
            None
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();

        while let Some(mut node) = curr {
            curr = node.next.take();
        }
    }
}
