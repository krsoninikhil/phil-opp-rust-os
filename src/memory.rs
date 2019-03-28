use x86_64::structures::paging::{PageTable, page_table::FrameError, PhysFrame,
                                 MapperAllSizes, MappedPageTable};
use x86_64::registers::control::Cr3;
use x86_64::{PhysAddr, VirtAddr};

// initialize a new MappedPageTable, a MapperAllSizes implementation
// since return type is not concrete, it can easily switched to RecursivePageTable
pub unsafe fn init(physical_memory_offset: u64) -> impl MapperAllSizes {
    let l4_table = active_level4_table(physical_memory_offset);
    let phys_to_virt = move |frame: PhysFrame| -> *mut PageTable {
        let phys = frame.start_address().as_u64();
        let virt = VirtAddr::new(phys + physical_memory_offset);
        virt.as_mut_ptr()
    }
    MappedPageTable::new(l4_table, phys_to_virt)
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: u64)
                             -> Option<PhysAddr>
{
    translate_addr_internal(addr, physical_memory_offset)  // all logic is separated to limit unsafe code block
}

fn translate_addr_internal(addr: VirtAddr, physical_memory_offset: u64)
                             -> Option<PhysAddr>
{
    let (l4_table_frame, _) = Cr3::read();
    let table_indices = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];

    let mut frame = l4_table_frame;
    for &index in &table_indices {
        let virt = VirtAddr::new(frame.start_address().as_u64() + physical_memory_offset);
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];  // iterate over `table` if want to print all entries
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}


pub unsafe fn active_level4_table(physical_memory_offset: u64)
                                  -> &'static mut PageTable
{
    let (l4_table_frame, _) = Cr3::read();

    let phys = l4_table_frame.start_address();
    let virt = VirtAddr::new(phys.as_u64() + physical_memory_offset);

    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *page_table_ptr  // unsafe
}
