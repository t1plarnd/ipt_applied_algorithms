#[cfg(test)]
mod tests;
use app::ListSet;

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
