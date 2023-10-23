/**
 * This mini-module is pulled from https://github.com/achingbrain/uint8-varint/blob/master/src/index.ts
 *
 * The encoding is:
 *
 * * unsigned integers are serialized 7 bits at a time, starting with the least significant bits
 * * the most significant bit (msb) in each output byte indicates if there is a continuation byte (msb = 1)
 * * there are no signed integers
 * * integers are minimally encoded
 * See https://github.com/multiformats/unsigned-varint for more information.
 */
// pub mod unsigned_varint {
  const N1: u64 = 2u64.pow(7);
  const N2: u64 = 2u64.pow(14);
  const N3: u64 = 2u64.pow(21);
  const N4: u64 = 2u64.pow(28);
  const N5: u64 = 2u64.pow(35);
  const N6: u64 = 2u64.pow(42);
  const N7: u64 = 2u64.pow(49);

  /** Most significant bit of a byte */
  const MSB: u8 = 0x80;
  /** Rest of the bits in a byte */
  const REST: u8 = 0x7f;

pub fn encoding_length(value: u64) -> usize {
  if value < N1 {
    return 1;
  }

  if value < N2 {
    return 2;
  }

  if value < N3 {
    return 3;
  }

  if value < N4 {
    return 4;
  }

  if value < N5 {
    return 5;
  }

  if value < N6 {
    return 6;
  }

  if value < N7 {
    return 7;
  }

  // TODO: if it's too large for javascript, it shouldn't get to us here.
  // if let Some(max_safe_int) = std::u64::MAX {
  //   if value > max_safe_int as u64 {
  //     panic!("Could not encode varint");
  //   }
  // }

  8
}

  pub fn decode_uint8_array_optimized(buf: &[u8], offset: usize) -> u64 {
    let mut res = 0;
    let mut shift = 0;
    let mut i = offset;

    loop {
      let b = buf[i];
      res |= ((b & REST) as u64) << shift;
      if b < MSB {
        break;
      }
      shift += 7;
      i += 1;
      if i - offset == 8 {
        panic!("Could not decode varint");
      }
    }

    res
  }

  pub fn decode_uint8_array(buf: &[u8], offset: usize) -> u64 {
    let mut b = buf[offset];
    let mut res = 0;

    res += (b & REST) as u64;
    if b < MSB {
      return res;
    }

    b = buf[offset + 1];
    res += ((b & REST) as u64) << 7;
    if b < MSB {
      return res;
    }

    b = buf[offset + 2];
    res += ((b & REST) as u64) << 14;
    if b < MSB {
      return res;
    }

    b = buf[offset + 3];
    res += ((b & REST) as u64) << 21;
    if b < MSB {
      return res;
    }

    b = buf[offset + 4];
    res += ((b & REST) as u64) * N4;
    if b < MSB {
      return res;
    }

    b = buf[offset + 5];
    res += ((b & REST) as u64) * N5;
    if b < MSB {
      return res;
    }

    b = buf[offset + 6];
    res += ((b & REST) as u64) * N6;
    if b < MSB {
      return res;
    }

    b = buf[offset + 7];
    res += ((b & REST) as u64) * N7;
    if b < MSB {
      return res;
    }

    panic!("Could not decode varint");
  }

// }
