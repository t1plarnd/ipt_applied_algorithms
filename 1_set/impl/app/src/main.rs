pub struct ListSet<T> {
    elements: Vec<T>,
}

impl<T> ListSet<T> {
    pub fn new() -> Self {
        ListSet {
            elements: Vec::new(),
        }
    }

    pub fn search(&self, item: &T) -> bool {
        for element in &self.elements {
            if element == item {
                return true;
            }
        }
        false
    }

    pub fn insert(&mut self, item: T) {
        if !self.search(&item) {
            self.elements.push(item);
        }
    }

    pub fn delete(&mut self, item: &T) {
        let mut found_index = None;

        for i in 0..self.elements.len() {
            if &self.elements[i] == item {
                found_index = Some(i);
                break;
            }
        }

        if let Some(index) = found_index {
            self.elements.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for item in &self.elements {
            if !other.search(item) {
                return false;
            }
        }
        true
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for item in &other.elements {
            result.insert(item.clone());
        }
        result
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = ListSet::new();
        for item in &self.elements {
            if other.search(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    pub fn set_difference(&self, other: &Self) -> Self {
        let mut result = ListSet::new();
        for item in &self.elements {
            if !other.search(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    pub fn sym_difference(&self, other: &Self) -> Self {
        let diff_a = self.set_difference(other);
        let diff_b = other.set_difference(self);
        diff_a.union(&diff_b)
    }
}

fn main() {
    let mut set_a = ListSet::new();
    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    let mut set_b = ListSet::new();
    set_b.insert(3);
    set_b.insert(4);
    set_b.insert(5);

    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);
    println!("Union: {:?}", set_a.union(&set_b));
    println!("Intersection: {:?}", set_a.intersection(&set_b));
    println!("Difference (A \\ B): {:?}", set_a.set_difference(&set_b));
    println!("Sym. Difference: {:?}", set_a.sym_difference(&set_b));
}