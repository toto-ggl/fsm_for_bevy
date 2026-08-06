/// Compare two enums by comparing their variant regardless of the data they possibly carry.
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
