// use crate::arch;
use crate::*;

// TODO: Find real value
const PAGE_NUMBER: usize = 50;

static mut PageUsage: [UsageFlags; PAGE_NUMBER] = [UsageFlags::Free; PAGE_NUMBER];

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum UsageFlags {
    Used = 1,
    Free = 0,
}

pub fn page_alloc() -> Option<usize> {
    let mut index = 0;

    // FIXME: Iterator
    while index < 50 {
        unsafe {
            match PageUsage[index] {
                UsageFlags::Free => {
                    PageUsage[index] = UsageFlags::Used;
                    let addr =
                        (&arch::HEAP_START as *const ()) as usize + index * arch::mmu::PAGE_SIZE;
                    return Some(addr);
                }
                UsageFlags::Used => (),
            }
        }

        index = index + 1;
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utest::uassert_eq;
    #[test_case]
    fn page_alloc_test() {
        let test = page_alloc();
        kassert_eq!(test.is_some(), true, "Page alloc test");
    }
}
