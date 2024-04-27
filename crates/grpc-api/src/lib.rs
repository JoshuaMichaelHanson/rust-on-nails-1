pub mod api {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("api");
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
