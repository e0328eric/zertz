// Copyright (c) 2022 Sungbae Jeong
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[inline]
pub fn usize_to_coord(n: usize) -> (usize, usize) {
    (n % 9, n / 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usize_to_coord() {
        let to_test = vec![
            (0, 0),
            (1, 0),
            (2, 2),
            (0, 3),
            (0, 0),
            (4, 2),
            (1, 3),
            (3, 1),
        ];

        for (x, y) in &to_test {
            assert_eq!(usize_to_coord(*x + *y * 9), (*x, *y));
        }
    }
}
