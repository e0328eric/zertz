use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Default)]
pub struct UnionFind<T> {
    inner: HashMap<T, usize>,
    reverse: HashMap<usize, T>,
    parent_data: Vec<usize>,
    rank_data: Vec<usize>,
}

impl<T> UnionFind<T>
where
    T: Eq + Hash,
{
    pub fn union(&mut self, x: &T, y: &T) {
        let x_location = self.find(x);
        let y_location = self.find(y);

        if x_location == y_location {
            return;
        }

        if self.rank_data[x_location] < self.rank_data[y_location] {
            self.parent_data[x_location] = y_location;
        } else {
            self.parent_data[y_location] = x_location;

            if self.rank_data[x_location] == self.rank_data[y_location] {
                self.rank_data[x_location] += 1;
            }
        }
    }

    pub fn find(&mut self, x: &T) -> usize {
        let x_location = self
            .inner
            .get(x)
            .expect("[Zertz Internal Error]: UnionFind::Find");
        self.find_parent(*x_location)
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

impl<T> UnionFind<T>
where
    T: Clone + Eq + Hash,
{
    pub fn clear(&mut self) {
        let keys: Vec<T> = self.inner.keys().cloned().collect();

        for (idx, elem) in keys.into_iter().enumerate() {
            *self.inner.get_mut(&elem).unwrap() = idx;
            *self.reverse.get_mut(&idx).unwrap() = elem;
            self.parent_data[idx] = idx;
        }
        for data in self.rank_data.iter_mut() {
            *data = 0;
        }
    }

    pub fn components(&self) -> Vec<T> {
        let mut output = Vec::with_capacity(self.inner.len());
        let mut parent_data = self.parent_data.clone();
        parent_data.sort();
        parent_data.dedup();

        for pd in parent_data {
            output.push(self.reverse[&pd].clone());
        }

        output
    }
}

impl<T> From<Vec<T>> for UnionFind<T>
where
    T: Clone + Eq + Hash,
{
    fn from(elems: Vec<T>) -> Self {
        let mut inner = HashMap::with_capacity(elems.len());
        let mut reverse = HashMap::with_capacity(elems.len());
        let mut parent_data = Vec::with_capacity(elems.len());
        let rank_data = vec![0; elems.len()];

        for (idx, elem) in elems.into_iter().enumerate() {
            inner.insert(elem.clone(), idx);
            reverse.insert(idx, elem);
            parent_data.push(idx);
        }

        Self {
            inner,
            reverse,
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

        assert_eq!(union_find.find(&1), 0);
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

        union_find.clear();

        union_find.union(&2, &3);
        union_find.union(&1, &5);
        union_find.union(&6, &7);
        union_find.union(&7, &5);

        assert_eq!(union_find.find(&1), union_find.find(&1));
        assert_eq!(union_find.find(&2), union_find.find(&2));
        assert_eq!(union_find.find(&3), union_find.find(&2));
        assert_eq!(union_find.find(&4), union_find.find(&4));
        assert_eq!(union_find.find(&5), union_find.find(&1));
        assert_eq!(union_find.find(&6), union_find.find(&1));
        assert_eq!(union_find.find(&7), union_find.find(&1));
        assert_eq!(union_find.find(&8), union_find.find(&8));
    }
}
