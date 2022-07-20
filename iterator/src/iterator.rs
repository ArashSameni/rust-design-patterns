pub struct Container<T> {
    data: Vec<T>,
}

impl<T> Container<T> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn add_item(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::<T> {
            container: self,
            index: 0,
        }
    }
}

pub struct Iter<'a, T: 'a> {
    container: &'a Container<T>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.container.data.get(self.index)?;
        self.index += 1;
        Some(current)
    }
}
