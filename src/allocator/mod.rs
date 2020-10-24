use crate::arch;

// TODO: Find real value
const PAGE_NUMBER: usize = 50;

static mut PageUsage: [UsageFlags; PAGE_NUMBER] = [UsageFlags::Free; PAGE_NUMBER];

#[repr(u8)]
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
