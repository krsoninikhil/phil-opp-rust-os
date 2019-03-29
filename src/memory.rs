use x86_64::registers::control::Cr3;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{
    PageTable,
    page_table::FrameError,
    PhysFrame,
    MapperAllSizes,
    MappedPageTable,
    Page,
    Size4KiB,
    Mapper,
    FrameAllocator,
    PageTableFlags
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

// initialize a new MappedPageTable, a MapperAllSizes implementation
// since return type is not concrete, it can easily switched to RecursivePageTable
pub unsafe fn init(physical_memory_offset: u64) -> impl MapperAllSizes {
    let l4_table = active_level4_table(physical_memory_offset);
    let phys_to_virt = move |frame: PhysFrame| -> *mut PageTable {
        let phys = frame.start_address().as_u64();
        let virt = VirtAddr::new(phys + physical_memory_offset);
        virt.as_mut_ptr()
    };
    MappedPageTable::new(l4_table, phys_to_virt)
}

unsafe fn active_level4_table(physical_memory_offset: u64)
                                  -> &'static mut PageTable
{
    let (l4_table_frame, _) = Cr3::read();

    let phys = l4_table_frame.start_address();
    let virt = VirtAddr::new(phys.as_u64() + physical_memory_offset);

    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *page_table_ptr  // unsafe
}

// map given to frame containing VGA physical address
pub fn create_example_mapping(
    page: Page,
    mapper: &mut impl Mapper<Size4KiB>,  // not using MapperAllSizes as we'll only call it for 4KiB pages
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let map_to_result = unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();  // is page is mapped, flush it from TLB
}

// Frame allocator for creating intermediate level page tables while creating new mapping
pub struct BootInfoFrameAllocator<I> where I: Iterator<Item = PhysFrame> {
    frames: I,
}

impl<I> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<I>
    where I: Iterator<Item = PhysFrame>
{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.frames.next()
    }
}

// create frames from memory map provided by bootloader to initialize frame allocator
pub fn init_frame_allocator(
    memory_map: &'static MemoryMap
) -> BootInfoFrameAllocator<impl Iterator<Item = PhysFrame>> {
    // get usabel memory region ranges and extract 4KiB frame addresses
    let regions = memory_map.iter().filter(|r| {
        r.region_type == MemoryRegionType::Usable
    });
    let addr_ranges = regions.map(|r| {
        r.range.start_addr()..r.range.end_addr()
    });
    let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

    // create physical frames from addresses
    let frames = frame_addresses.map(|addr| {
        PhysFrame::containing_address(PhysAddr::new(addr))
    });

    BootInfoFrameAllocator{ frames }
}

// this function is not used in favour of `translate_addr` provided by
// MapperAllSizes returned from above `init` function
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
