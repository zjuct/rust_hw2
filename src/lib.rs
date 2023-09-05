use std::vec;

// Hw1
pub struct Buffer<T> {
    buf: Vec<T>
}

impl<T: Clone + std::iter::Sum> Buffer<T> {
    pub fn sum(&self) -> T {
        self.buf.iter().cloned().sum()
    }
}

// Hw2
pub fn compareString(x: &str, y: &str) -> bool {
    for (a, b) in x.chars().zip(y.chars()) {
        if a != b {
            return a > b
        }
    }
    x.len() > y.len()
}

// Hw3
pub fn hw3() -> Vec<char> {
    let v = vec!['a', 'b', 'c', 'd', 'e'];
    v.into_iter().map(|c| {
        let b_val = c as u8;
        (b_val + 1) as char
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_test() {
        let buf = Buffer {
            buf: vec![1, 2, 3, 4, 5]
        };
        assert_eq!(buf.sum(), 15);

        let buf = Buffer {
            buf: vec![1.1, 2.2, 3.3]
        };
        assert_eq!(buf.sum(), 6.6);
    }

    #[test]
    fn compare_string_test() {
        assert!(compareString("abc", "abb"));
        assert!(compareString("abcd", "abc"));
        assert!(compareString("你好123", "你好"));

        assert!(!compareString("abc", "def"));
        assert!(!compareString("abc", "abcd"));
        assert!(!compareString("你", "你好"));
    }

    #[test]
    fn hw3_test() {
        assert_eq!(hw3(), vec!['b', 'c', 'd', 'e', 'f']);
    }
}
