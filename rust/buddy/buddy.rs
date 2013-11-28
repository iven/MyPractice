use std::num;
use std::vec;

mod utils;

struct Buddy {
  size: uint,
  mem: ~[u8],
}

impl Buddy {
  pub fn new(size: uint) -> Option<Buddy> {
    if size < 1 || !utils::is_power_of_2(size) {
      return None;
    }

    let mem = do vec::build(Some(size * 2 - 1)) |push| {
      let mut node_size = 2 * size;

      for i in range(0, size * 2 - 1) {
        if utils::is_power_of_2(i + 1) {
          node_size /= 2;
        }
        push(utils::uint_to_u8(node_size));
      }
    };

    Some(Buddy {
      size: size,
      mem: mem,
    })
  }

  pub fn alloc(&mut self, mut size: uint) -> int {
    let mut node_size = self.size;
    let mut index = 0;

    size = utils::to_power_of_2(size);

    if size == 0 || size > self.at(index) { return -1 }

    while node_size != size {
      let left = self.at(Buddy::left_index(index));
      let right = self.at(Buddy::right_index(index));

      index = if left >= size && right >= size {
        if left > right {
          Buddy::right_index(index)
        } else {
          Buddy::left_index(index)
        }
      } else if left >= size {
        Buddy::left_index(index)
      } else {
        Buddy::right_index(index)
      };

      node_size /= 2;
    }

    self.set(index, 0);

    let offset = ((index as int) - ((self.size / node_size) as int) + 1) * (node_size as int);

    while index != 0 {
      index = Buddy::parent_index(index);
      let left = self.at(Buddy::left_index(index));
      let right = self.at(Buddy::right_index(index));
      let max_child = num::max(left, right);
      self.set(index, max_child);
    }

    offset
  }

  pub fn free(&mut self, offset: int) {
    if offset < 0 { return; }

    let mut index = offset as uint + self.size - 1;
    let mut node_size = 1;
    while self.at(index) != 0 {
      node_size *= 2;
      index = Buddy::parent_index(index);
      if index == 0 {
        return;
      }
    }
    self.set(index, node_size);

    while index != 0 {
      index = Buddy::parent_index(index);
      node_size *= 2;

      let left = self.at(Buddy::left_index(index));
      let right = self.at(Buddy::right_index(index));

      if left + right == node_size {
        self.set(index, node_size);
      } else {
        self.set(index, num::max(left, right));
      }
    }
  }

  pub fn at(&self, index: uint) -> uint {
    utils::u8_to_uint(self.mem[index])
  }

  pub fn set(&mut self, index: uint, value: uint) {
    self.mem[index] = utils::uint_to_u8(value);
  }

  fn parent_index(index: uint) -> uint {
    (index + 1) / 2 - 1
  }

  fn left_index(index: uint) -> uint {
    2 * (index + 1) - 1
  }

  fn right_index(index: uint) -> uint {
    2 * (index + 1)
  }
}
