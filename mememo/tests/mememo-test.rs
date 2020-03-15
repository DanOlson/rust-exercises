use mememo::*;

#[test]
fn test_empty_array() {
    let numbers = [];
    let stats = mememo(&numbers);
    assert_eq!(stats.mean, None);
    assert_eq!(stats.median, None);
    assert_eq!(stats.mode, None);
}

#[test]
fn test_array_with_one_entry() {
    let numbers = [1];
    let stats = mememo(&numbers);
    assert_eq!(stats.mean, Some(1.0));
    assert_eq!(stats.median, Some(1.0));
    assert_eq!(stats.mode, Some(1));
}

#[test]
fn test_array_with_odd_number_of_entries() {
    let numbers = [2, 3, 4];
    let stats = mememo(&numbers);
    assert_eq!(stats.mean, Some(3.0));
    assert_eq!(stats.median, Some(3.0));
    assert_eq!(stats.mode, Some(4));
}

#[test]
fn test_array_with_repeated_elements() {
    let numbers = [1, 2, 3, 3, 3];
    let stats = mememo(&numbers);
    assert_eq!(stats.mean, Some(2.4));
    assert_eq!(stats.median, Some(3.0));
    assert_eq!(stats.mode, Some(3));
}

#[test]
fn test_array_with_several_repeated_elements() {
    let numbers = [1, 2, 2, 3, 3];
    let stats = mememo(&numbers);
    assert_eq!(stats.mean, Some(2.2));
    assert_eq!(stats.median, Some(2.0));
    assert_eq!(stats.mode, Some(3));
}
