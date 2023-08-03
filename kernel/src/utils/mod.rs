pub mod lock;

pub fn external_symbol_value<T>(sym: &T) -> usize {
    (sym as *const T) as usize
}
