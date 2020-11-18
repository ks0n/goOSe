//! Collection of useful utilites to help goOSe development

/// Convert an external symbol (defined by the linker for example) to a value usable
/// in the code
pub fn external_symbol_address<T>(sym: T) -> usize {
    (&sym as *const T) as usize
}
