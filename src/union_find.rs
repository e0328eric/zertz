use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct UnionFind<T> {
    inner: HashMap<T, usize>,
    parent_data: Vec<usize>,
    rank_data: Vec<usize>,
}

impl<T> UnionFind<T>
where
    T: Eq + Hash,
{
    pub fn union(&mut self, x: &T, y: &T) -> Option<()> {
        let x_location = self.find(x)?;
        let y_location = self.find(y)?;

        if x_location == y_location {
            return Some(());
        }

        if self.rank_data[x_location] < self.rank_data[y_location] {
            self.parent_data[x_location] = y_location;
        } else {
            self.parent_data[y_location] = x_location;

            if self.rank_data[x_location] == self.rank_data[y_location] {
                self.rank_data[x_location] += 1;
            }
        }

        Some(())
    }

    pub fn find(&mut self, x: &T) -> Option<usize> {
        let x_location = self.inner.get(x)?;
        Some(self.find_parent(*x_location))
    }

    fn find_parent(&mut self, location: usize) -> usize {
        if self.parent_data[location] == location {
            location
        } else {
            let root_parent = self.find_parent(self.parent_data[location]);
            self.parent_data[location] = root_parent;
            root_parent
        }
    }
}

impl<T> From<Vec<T>> for UnionFind<T>
where
    T: Eq + Hash,
{
    fn from(elems: Vec<T>) -> Self {
        let mut inner = HashMap::with_capacity(elems.len());
        let mut parent_data = Vec::with_capacity(elems.len());
        let mut rank_data = vec![0; elems.len()];

        for (idx, elem) in elems.into_iter().enumerate() {
            inner.insert(elem, idx);
            parent_data.push(idx);
        }

        Self {
            inner,
            parent_data,
            rank_data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_item_union_find() {
        let elems = vec![1];
        let mut union_find = UnionFind::from(elems);

        assert_eq!(union_find.find(&1), Some(0));
    }

    #[test]
    fn complex_union_find() {
        let elems = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut union_find = UnionFind::from(elems);

        union_find.union(&1, &2);
        union_find.union(&4, &5);
        union_find.union(&6, &1);
        union_find.union(&3, &7);
        union_find.union(&7, &8);
        union_find.union(&2, &5);

        assert_eq!(union_find.find(&1), union_find.find(&6));
        assert_eq!(union_find.find(&2), union_find.find(&6));
        assert_eq!(union_find.find(&3), union_find.find(&3));
        assert_eq!(union_find.find(&4), union_find.find(&6));
        assert_eq!(union_find.find(&5), union_find.find(&6));
        assert_eq!(union_find.find(&6), union_find.find(&6));
        assert_eq!(union_find.find(&7), union_find.find(&3));
        assert_eq!(union_find.find(&8), union_find.find(&3));
    }
}
