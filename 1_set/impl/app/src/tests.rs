use super::*;

#[test]
fn test_insert_and_search() {
    let mut set = ListSet::new();
    assert!(!set.search(&10));

    set.insert(10);
    assert!(set.search(&10));

    set.insert(10);
    assert_eq!(set.elements.len(), 1);
}

#[test]
fn test_delete() {
    let mut set = ListSet::new();
    set.insert(1);
    set.insert(2);

    set.delete(&1);

    assert!(!set.search(&1));
    assert!(set.search(&2));
    assert_eq!(set.elements.len(), 1);
}

#[test]
fn test_clear() {
    let mut set = ListSet::new();
    set.insert(1);
    set.insert(2);

    set.clear();

    assert_eq!(set.elements.len(), 0);
    assert!(!set.search(&1));
}

#[test]
fn test_is_subset() {
    let mut set_a = ListSet::new();
    set_a.insert(1);
    set_a.insert(2);

    let mut set_b = ListSet::new();
    set_b.insert(1);
    set_b.insert(2);
    set_b.insert(3);

    assert!(set_a.is_subset(&set_b));
    assert!(!set_b.is_subset(&set_a));
}

#[test]
fn test_union() {
    let mut set_a = ListSet::new();
    set_a.insert(1);
    set_a.insert(2);

    let mut set_b = ListSet::new();
    set_b.insert(2);
    set_b.insert(3);

    let union_set = set_a.union(&set_b);

    assert!(union_set.search(&1));
    assert!(union_set.search(&2));
    assert!(union_set.search(&3));
    assert_eq!(union_set.elements.len(), 3);
}

#[test]
fn test_intersection() {
    let mut set_a = ListSet::new();
    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    let mut set_b = ListSet::new();
    set_b.insert(2);
    set_b.insert(3);
    set_b.insert(4);

    let intersection_set = set_a.intersection(&set_b);

    assert!(intersection_set.search(&2));
    assert!(intersection_set.search(&3));
    assert!(!intersection_set.search(&1));
    assert!(!intersection_set.search(&4));
    assert_eq!(intersection_set.elements.len(), 2);
}

#[test]
fn test_set_difference() {
    let mut set_a = ListSet::new();
    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);

    let mut set_b = ListSet::new();
    set_b.insert(2);
    set_b.insert(4);

    let diff_set = set_a.set_difference(&set_b);

    assert!(diff_set.search(&1));
    assert!(diff_set.search(&3));
    assert!(!diff_set.search(&2));
    assert_eq!(diff_set.elements.len(), 2);
}

#[test]
fn test_sym_difference() {
    let mut set_a = ListSet::new();
    set_a.insert(1);
    set_a.insert(2);

    let mut set_b = ListSet::new();
    set_b.insert(2);
    set_b.insert(3);

    let sym_diff_set = set_a.sym_difference(&set_b);

    assert!(sym_diff_set.search(&1));
    assert!(sym_diff_set.search(&3));
    assert!(!sym_diff_set.search(&2));
    assert_eq!(sym_diff_set.elements.len(), 2);
}
