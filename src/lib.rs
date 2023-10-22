// pub(crate)
pub fn round_up(x: usize, to: usize) -> usize {
    let m = x % to;
    if m == 0 {
        x
    } else {
        x - m + to
    }
}

// pub(crate)
pub fn page_aligned(size: usize) -> usize {
    round_up(size, 4096)
}
///
/// `MemAlign` represents metadata for a page aligned allocation. 
///
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct MemAlign<T> {
    byte_size: usize,
    capacity: usize,
    remainder: usize,
    phantom: std::marker::PhantomData<T>,
}

impl<T> MemAlign<T> {
    pub fn element_size() -> usize {
        std::mem::size_of::<T>()
    }

    pub fn byte_size(&self) -> usize {
        self.byte_size
    }

    /// Capacity in instances
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// remainder in bytes
    pub fn remainder(&self) -> usize {
        self.remainder
    }

    pub fn is_valid(&self) -> bool {
        (Self::element_size() * self.capacity) + self.remainder == self.byte_size
    }

    pub fn new(capacity: usize) -> Self {
        let element_size = Self::element_size();
        assert!(element_size != 0, "ZST are not supported");
        let size = element_size * capacity;

        let byte_size = page_aligned(size);
        let remainder = byte_size % element_size;
        assert!((byte_size - remainder) % element_size == 0);
        let capacity = (byte_size - remainder) / element_size;
        assert!(byte_size != 0);

        Self {
            byte_size,
            capacity,
            remainder,
            phantom: <_>::default(),
        }
    }
}


#[test]
fn test_roundup() {
    // assert!(round_up(0, 4096) == 4096);
    // println!("{}", round_up(0, 4096));
    assert!(round_up(1, 4096) == 4096);
    assert!(round_up(4095, 4096) == 4096);
    assert!(round_up(4096, 4096) == 4096);
    assert!(round_up(4097, 4096) == 2 * 4096);
    assert!(round_up(2 * 4096 + 1, 4096) == 3 * 4096);
}

#[test]
fn test_paged_alloc() {
    #[repr(C)]
    struct TestStruct {
        data: [u8; 18],
    }

    let element_size: usize = std::mem::size_of::<TestStruct>();
    assert!(element_size == 18);
    let count = 10;
    // let page_size = 4096;
    let alloc = MemAlign::<TestStruct>::new(count);
    assert!(alloc.capacity() == 227);
    assert!(alloc.remainder() == 10);
    assert!(alloc.byte_size() == 4096);
    // println!("{}", alloc.is_valid());

    // dbg!("{}", alloc);
}
