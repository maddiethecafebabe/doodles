use core::ptr::NonNull;

mod node;

pub use node::{Iter, IterMut, Node};

pub struct LinkedList<T> {
    start: Option<NonNull<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { start: None }
    }

    pub fn from<I: IntoIterator<Item = T>>(data: I) -> Self {
        let stream = data.into_iter();
        let mut this = Self::new();

        for item in stream {
            this.append(item);
        }

        this
    }

    pub fn first(&self) -> Option<&Node<T>> {
        self.start.map(|ptr| unsafe { ptr.as_ref() })
    }

    pub fn first_mut(&mut self) -> Option<&mut Node<T>> {
        self.start.map(|mut ptr| unsafe { ptr.as_mut() })
    }

    pub fn append(&mut self, data: T) {
        match self.last_mut() {
            Some(node) => node.insert_after(data),
            None => {
                self.start = Some(Node::new_leaked(data))
            }
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut Node<T>> {
        match self.first_mut() {
            None => None,
            Some(mut node) => {
                loop {
                    if node.next_mut().is_none() {
                        return Some(node);
                    } else {
                        node = unsafe { node.next_mut_unchecked() };
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(self)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        if let Some(ptr) = self.start.take() {
            let _ = unsafe { Box::from_raw(ptr.as_ptr()) };
        }
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkedList")
            .field("start", &self.first())
            .finish()
    }
}
