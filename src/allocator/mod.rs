// use crate::arch;
use crate::*;

// TODO: Find real value
const PAGE_NUMBER: usize = 50;

static mut PageUsage: [UsageFlags; PAGE_NUMBER] = [UsageFlags::Free; PAGE_NUMBER];

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum UsageFlags {
    Used = 1,
    Free = 0,
}

pub fn page_alloc() -> Option<usize> {
    for index in 0..PAGE_NUMBER {
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
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utest::uassert_eq;

    #[test_case]
    fn page_alloc_one_test() {
        let test = page_alloc();
        kassert_eq!(test.is_some(), true, "Page alloc one page test");
    }

    #[test_case]
    fn page_alloc_all_test() {
        while page_alloc().is_some() {
            // At some point we will not have any free pages left
        }

        kassert_eq!(true, true, "Page alloc all pages test");
    }
}
