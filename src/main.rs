//use math::round;

// First, we'll just do Ding's reconciliation technique, but later we can do more.
fn ding_hint(x: i32, q: i32) -> i32 {
    let upper_bound = (q-1) / 4;

    let lower_bound = (-1) * upper_bound;

    if x < upper_bound && x > lower_bound {
        return 0;
    }

    1
}


// For the purposes of this operation, it is easier to represent numbers
// modulo 2 as numbers modulo q/2. This divides q in half, which is useful when
// We want numbers to round evenly to either 0 or 1.
//
// For example, if q=31, the following numbers would be rounded to:
// [0, 7]  = [0, 7]
// [8, 15] = [8, 15]
// [16, 23] = [-15, -8]
// [24,30] = [-7, -1]
//
// Assumes q is prime
fn convert_to_q_over_2(q: i32) -> Vec<i32> {

    let mut numbers_modulo_q = Vec::with_capacity(q as usize);
    let mut numbers_modulo_q_over_2 = vec![0; q as usize];

    // first build a list of all numbers mod q
    for x in 1..q {
        numbers_modulo_q.push(x);
    }

    // now, divide these in half
    let size_half_of_q = (q-1) / 2;
    let mut chunks = numbers_modulo_q.chunks(size_half_of_q as usize);

    // the first half are on the positive side of q/2
    for x in chunks.next().unwrap() {
        numbers_modulo_q_over_2[*x as usize] = x % q;
    }

    // the second half are on the negative side of q/2
    for x in chunks.next().unwrap() {
        numbers_modulo_q_over_2[*x as usize] = (x % q) - q;
    }

    numbers_modulo_q_over_2
}

// Extract takes an integer value and a hint, and extracts the underlying real
// value without the error terms
fn extract(x: i32, hint: i32, q: i32) -> i32 {
    let signal = hint * (q-1) / 2;

    (x + signal % q) % 2
}

fn main() {
    println!("testing ding!");

    let res_1 = ding_hint(29, 31);
    print!("29: {:?} \n", res_1);

    let res_2 = ding_hint(30, 31);
    print!("30: {:?} \n", res_2);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_ding_hint() {
        // first quartile
        assert_eq!(0, ding_hint(0, 31));
        assert_eq!(0, ding_hint(1, 31));

        // second quartile
        assert_eq!(1, ding_hint(8, 31));
        assert_eq!(1, ding_hint(9, 31));

        assert_eq!(1, ding_hint(14, 31));
        assert_eq!(1, ding_hint(15, 31));

        // third quartile
        assert_eq!(1, ding_hint(-14, 31));
        assert_eq!(1, ding_hint(-15, 31));

        // fourth quartile
        assert_eq!(0, ding_hint(-2, 31));
        assert_eq!(0, ding_hint(-1, 31));
    }

    #[test]
    fn test_convert_to_q_over_2() {
        let res = convert_to_q_over_2(31);

        // Ensure that the correct q/4 buckets were created, partitioning the
        // number line

        // first quartile
        assert_eq!(res[0], 0);
        assert_eq!(res[1], 1);

        // second quartile
        assert_eq!(res[8], 8);
        assert_eq!(res[9], 9);

        // third quartile
        assert_eq!(res[16], -15);
        assert_eq!(res[17], -14);

        // fourth quartile
        assert_eq!(res[29], -2);
        assert_eq!(res[30], -1);
    }

    #[test]
    fn test_extract_prime_one() {

        // ensure that two independent values round to the same bit
        // Subject to the constraints:
        // The error term |x-y| must be even and less than (q/4) - 2

        // test values that naively would round to different values.

        let q = 31;

        {
            let x = 6;
            let y = 8;

            // test these in both directions
            let x_hint = ding_hint(x, q);
            let y_hint = ding_hint(y, q);
            {
                let rounded_x = extract(x, x_hint, q);
                let rounded_y = extract(y, x_hint, q);
                assert_eq!(rounded_x, rounded_y);
            }
            {
                let rounded_x = extract(x, y_hint, q);
                let rounded_y = extract(y, y_hint, q);
                assert_eq!(rounded_x, rounded_y);
            }
        }
        {
            let x = 5;
            let y = 7;
            let x_hint = ding_hint(x, q);
            let y_hint = ding_hint(y, q);
            {
                let rounded_x = extract(x, x_hint, q);
                let rounded_y = extract(y, x_hint, q);
                assert_eq!(rounded_x, rounded_y);
            }
            {
                let rounded_x = extract(x, y_hint, q);
                let rounded_y = extract(y, y_hint, q);
                assert_eq!(rounded_x, rounded_y);
            }
        }
        {
            let x = 7;
            let y = 11;
            let x_hint = ding_hint(x, q);
            let rounded_x = extract(x, x_hint, q);
            let rounded_y = extract(y, x_hint, q);
            assert_eq!(rounded_x, rounded_y);
        }
    }
}
