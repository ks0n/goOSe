pub mod init_cell;
pub mod init_once;
pub mod lock;

// pub use init_cell;
// pub use lock;

pub fn external_symbol_value<T>(sym: &T) -> usize {
    (sym as *const T) as usize
}
