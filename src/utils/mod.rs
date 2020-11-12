//! Collection of useful utilites to help goOSe development

/// Convert an external symbol (defined by the linker for example) to a value usable
/// in the code
pub fn value_from_external_symbol(sym: ()) -> usize {
    (&sym as *const ()) as usize
}
