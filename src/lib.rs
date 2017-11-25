
/// This is a small library for finding sparse numbers.
/// A number is sparse if there are no consecutive ones
/// in its binary representation.
///
pub mod sparse {

    //! Returns the next sparse number after
    //! @param num. If num is sparse, num will
    //! be returned.
    pub fn next_sparse_num(mut num: u32) -> u32 {
        let mut next = true;
        let mut shifts;
        let mut add;
        while next == true {

            let tuple = is_sparse(num);
            next = tuple.0;
            shifts = tuple.1;

            if next == true {
                add = 1;
                add <<= shifts - 1;
                num = num + add;
            } else {
                return num;
            }
        }
        return 0;
    }

    fn is_sparse(mut num: u32) -> (bool, u32) {
        let mut next = false;
        let mut sequential_ones = 0;
        let mut shifts = 0;

        while num > 0 {
            if num & 1 == 1 {
                sequential_ones = sequential_ones + 1;
                if sequential_ones == 2 {
                    next = true;
                    break;
                }
            } else {
                sequential_ones = 0;
            }
            shifts = shifts + 1;
            num >>= 1;
        }
        if next == true {
            return (true, shifts);
        } else {
            return (false, num);
        }
    }
}

#[test]
fn test_next_sparse_num_4() {
    let val = sparse::next_sparse_num(4);
    assert_eq!(val, 4, "{} != {}", val, 4);
}

#[test]
fn test_next_sparse_num_8() {
    let val = sparse::next_sparse_num(6);
    assert_eq!(val, 8, "{} != {}", val, 8);
}

#[test]
fn test_next_sparse_num_40() {
    let val = sparse::next_sparse_num(38);
    assert_eq!(val, 40, "{} != {}", val, 40);
}

#[test]
fn test_next_sparse_num_64() {
    let val = sparse::next_sparse_num(44);
    assert_eq!(val, 64, "{} != {}", val, 64);
}
