use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[macro_export]
macro_rules! debug {
    ( $($a:expr),* ) => {
        #[cfg(debug_assertions)]
        eprint!("{} - {}: ", file!(), line!());
        #[cfg(debug_assertions)]
        eprintln!(concat!($(stringify!($a), "={:?}    "),*), $(&$a),*);
    };
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
