/// ["a", "b"] -> ["1. a", "2. b"]
pub fn numbered(items: &[&str]) -> Vec<String> {
    // TODO: number every item "n. name", counting from 1
    todo!()
}

/// [1, 2, 3] . [4, 5, 6] = 1*4 + 2*5 + 3*6 = 32
pub fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    // TODO: multiply matching items and add the products up
    todo!()
}

/// Page numbers start at 0. page(&[1,2,3,4,5], 1, 2) is [3, 4].
pub fn page(items: &[i32], page: usize, size: usize) -> Vec<i32> {
    // TODO: take one page of `size` items starting at `page * size`
    todo!()
}

/// How many times does a number come right after a smaller one? [3, 5, 4, 8] -> 2
pub fn count_rises(numbers: &[i32]) -> usize {
    // TODO: look at each neighbouring pair of numbers and count the ones that go up
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbered_uses_enumerate() {
        assert_eq!(numbered(&["a", "b"]), vec!["1. a", "2. b"]);
        assert_eq!(numbered(&[]), Vec::<String>::new());
    }

    #[test]
    fn dot_product_zips_two_slices() {
        assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32);
        assert_eq!(dot_product(&[1, 2, 3], &[10, 10]), 30, "stops at the shorter slice");
    }

    #[test]
    fn page_skips_then_takes() {
        let items = [1, 2, 3, 4, 5];
        assert_eq!(page(&items, 0, 2), vec![1, 2]);
        assert_eq!(page(&items, 1, 2), vec![3, 4]);
        assert_eq!(page(&items, 2, 2), vec![5], "last page can be short");
        assert_eq!(page(&items, 9, 2), Vec::<i32>::new());
    }

    #[test]
    fn count_rises_uses_windows() {
        assert_eq!(count_rises(&[3, 5, 4, 8]), 2);
        assert_eq!(count_rises(&[5, 4, 3]), 0);
        assert_eq!(count_rises(&[7]), 0);
        assert_eq!(count_rises(&[]), 0);
    }
}
