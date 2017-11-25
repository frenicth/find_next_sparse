
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
        let mut add;
        while next == true {

            match is_sparse(num) {
                Ok(_number) => next = false,
                Err(shifts) => {
                    add = 1;
                    add <<= shifts;
                    num = num + add;
                }
            }

        }
        return num;
    }

    fn is_sparse(mut num: u32) -> Result<u32, u32> {
        let mut next = false;
        let mut shifts = 0;
        let ret_val;

        while num > 0 {
            if num & 3 == 3 {
                next = true;
                break;
            }
            shifts = shifts + 1;
            num >>= 1;
        }
        if next == true {
            ret_val = Err(shifts);
        } else {
            ret_val = Ok(num);
        }
        return ret_val;
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
