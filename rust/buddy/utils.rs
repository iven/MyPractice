pub fn is_power_of_2(num: uint) -> bool {
  num != 0 && ((num & (num - 1)) == 0)
}

pub fn to_power_of_2(mut num: uint) -> uint {
  let mut bit = 0;

  if is_power_of_2(num) { return num; }

  while num > 0 {
    num >>= 1;
    bit += 1;
  }
  return 1 << bit;
}
pub fn uint_to_u8(num: uint) -> u8 {
  if num == 0 { 0 } else { log2(num) as u8 + 1 }
}

pub fn u8_to_uint(num: u8) -> uint {
  let power = pow2(num);
  if power == 1 { 0 } else { power / 2 }
}

pub fn log2(num: uint) -> i8 {
  let mut result = -1i8;
  let mut num = num;

  while num > 0 {
    num >>= 1;
    result += 1;
  }

  result
}

pub fn pow2(num: u8) -> uint {
  1 << num
}
