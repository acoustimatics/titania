//! A table of names and associated values.

pub struct Item<T> {
    pub name: String,
    pub value: T,
}

impl<T> Item<T> {
    fn new(name: &str, value: T) -> Self {
        let name = name.to_owned();
        Self { name, value }
    }
}

pub struct Table<T> {
    pub items: Vec<Item<T>>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, name: &str, value: T) {
        let symbol = Item::new(name, value);
        self.items.push(symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<&T> {
        self.items
            .iter()
            .rev()
            .find(|item| item.name == name)
            .map(|item| &item.value)
    }
}
