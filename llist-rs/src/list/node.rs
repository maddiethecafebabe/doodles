use core::{ops::{Deref, DerefMut}, ptr::NonNull, marker::PhantomData};

use super::LinkedList;

pub struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub fn new_leaked(data: T) -> NonNull<Self> {
        unsafe { NonNull::new_unchecked(Box::leak(Box::new(Self::new(data)))) }
    }

    pub fn next(&self) -> Option<&Node<T>> {
        self.next.map(|ptr| unsafe { ptr.as_ref() })
    }

    pub fn next_mut(&mut self) -> Option<&mut Node<T>> {
        self.next.map(|mut ptr| unsafe { ptr.as_mut() })
    }

    pub fn insert_after(&mut self, data: T) {
        let node_ptr = Box::leak(Box::new(Node::new(data)));
    
        if let Some(next) = self.next {
            unsafe {
                node_ptr.set_next(Some(next));
            }
        }

        self.next = NonNull::new(node_ptr);
    }

    pub unsafe fn set_next(&mut self, next: Option<NonNull<Node<T>>>) {
        self.next = next;
    }

    #[inline]
    pub unsafe fn next_mut_unchecked(&mut self) -> &mut Node<T> {
        self.next.unwrap_unchecked().as_mut()
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("next", &self.next())
            .finish()
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        if let Some(node) = self.next.take() {
            let _ = unsafe { Box::from_raw(node.as_ptr()) };
        }
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub struct Iter<'a, T> {
    node: Option<&'a Node<T>>
}

impl<'a, T> Iter<'a, T> {
    pub fn new(map: &'a LinkedList<T>) -> Self {
        Self { node: map.first() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node = self.node?.next();
        self.node.map(Deref::deref)
    }
}

pub struct IterMut<'a, T> {
    node: Option<NonNull<Node<T>>>,
    _phantom: PhantomData<&'a mut Node<T>>
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(map: &'a mut LinkedList<T>) -> Self {
        Self {
            node: map.first_mut().map(|ptr| unsafe { NonNull::new_unchecked(ptr) }),
            _phantom: PhantomData
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = unsafe { self.node?.as_mut() };
         
        self.node = cur.next_mut().map(|ptr| unsafe { NonNull::new_unchecked(ptr) });

        Some(cur)
    }
}
