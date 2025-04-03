const EMPTY_REF: u32 = u32::MAX;

pub(crate) struct Node<T> {
    prev: u32,
    next: u32,
    item: T,
}

pub(crate) struct LinkList<T> {
    nodes: Vec<Node<T>>,
}

impl<T: Copy> LinkList<T> {
    #[inline]
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn new(items: Vec<T>) -> Self {
        let mut nodes = Vec::with_capacity((2 * items.len()).min(16));
        if items.is_empty() {
            return Self { nodes };
        }

        let mut prev = EMPTY_REF;
        let mut next = 1;
        for item in items {
            nodes.push(Node { prev, next, item });
            prev = next - 1;
            next += 1;
        }

        nodes.last_mut().unwrap().next = EMPTY_REF;

        Self { nodes }
    }

    fn split_at(&mut self, index: u32, a: T, b: T) -> (u32, u32) {
        // insert a new node as next and update this node value

        let new_index = self.nodes.len() as u32;

        let mut node = &mut self.nodes[index as usize];
        let next = node.next;
        node.next = new_index;
        node.item = a;

        self.nodes.push(Node {
            prev: index,
            next,
            item: b,
        });

        let index_next = next as usize;

        if index_next < self.nodes.len() {
            self.nodes[index_next].prev = new_index;
        }

        (index, new_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::list::LinkList;

    #[test]
    fn test_00() {
        let mut list = LinkList::new(vec![0, 1, 3]);
        list.split_at(1, 1, 2);

        assert_eq!(list.nodes[0].item, 0);
        assert_eq!(list.nodes[1].item, 1);
        assert_eq!(list.nodes[3].item, 2);
        assert_eq!(list.nodes[2].item, 3);
    }

    #[test]
    fn test_01() {
        let mut list = LinkList::new(vec![0, 2, 3]);
        list.split_at(0, 0, 1);

        assert_eq!(list.nodes[0].item, 0);
        assert_eq!(list.nodes[3].item, 1);
        assert_eq!(list.nodes[1].item, 2);
        assert_eq!(list.nodes[2].item, 3);
    }

    #[test]
    fn test_02() {
        let mut list = LinkList::new(vec![0, 1, 2]);
        list.split_at(2, 2, 3);

        assert_eq!(list.nodes[0].item, 0);
        assert_eq!(list.nodes[1].item, 1);
        assert_eq!(list.nodes[2].item, 2);
        assert_eq!(list.nodes[3].item, 3);
    }
}
