use buddy::Buddy;
use utils::log2;

mod buddy;
mod utils;

#[test]
fn test_is_power_of_2() {
  assert!(utils::is_power_of_2(1));
  assert!(!utils::is_power_of_2(3));
}

#[test]
fn test_to_power_of_2() {
  assert_eq!(utils::to_power_of_2(1), 1);
  assert_eq!(utils::to_power_of_2(3), 4);
  assert_eq!(utils::to_power_of_2(16), 16);
}

#[test]
fn test_log2() {
  assert_eq!(log2(1), 0);
  assert_eq!(log2(2), 1);
}

#[test]
fn test_buddy_new_with_wrong_size() {
  assert!(Buddy::new(0).is_none());
  assert!(Buddy::new(3).is_none());
}

#[test]
fn test_buddy_new_with_correct_size() {
  let buddy = Buddy::new(16);
  match buddy {
    Some(b) => {
      assert_eq!(b.size, 16);
      assert_eq!(b.at(0), 16);
      assert_eq!(b.at(1), 8);
      assert_eq!(b.at(2), 8);
    }
    None => {
      fail!("Buddy is None when initialized with size 2.");
    },
  }
}

#[test]
fn test_buddy_at() {
  let buddy = Buddy::new(16).unwrap();
  assert_eq!(buddy.at(0), 16);
}

#[test]
fn test_buddy_set() {
  let mut buddy = Buddy::new(16).unwrap();
  buddy.set(0, 8);
  assert_eq!(buddy.at(0), 8);

  buddy.set(0, 0);
  assert_eq!(buddy.at(0), 0);
}

#[test]
fn test_buddy_alloc() {
  let mut buddy = Buddy::new(16).unwrap();
  assert_eq!(buddy.alloc(17), -1);

  assert_eq!(buddy.alloc(4), 0);
  assert_eq!(buddy.at(0), 8);

  assert_eq!(buddy.alloc(2), 4);
  assert_eq!(buddy.at(0), 8);

  assert_eq!(buddy.alloc(4), 8);
  assert_eq!(buddy.at(0), 4);
}

#[test]
fn test_buddy_free() {
  let mut buddy = Buddy::new(16).unwrap();

  let mut offset = buddy.alloc(4);
  buddy.free(offset);
  assert_eq!(buddy.at(0), 16);

  buddy.alloc(4);
  offset = buddy.alloc(8);
  buddy.free(offset);
  assert_eq!(buddy.at(0), 8);
}
