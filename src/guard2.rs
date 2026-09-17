#[derive(Debug)]
pub struct BitFlipGuard2<T>
where
    T: Copy + Eq + PartialEq,
{
    value: T,
    value_ghost: T,
}

impl<T> BitFlipGuard2<T>
where
    T: Copy + Eq + PartialEq,
{
    pub const fn new(value: T) -> Self {
        Self {
            value,
            value_ghost: value,
        }
    }

    pub fn get(&self) -> Option<T> {
        let v = unsafe { core::ptr::read_volatile(&self.value as *const T) };
        let v_ghost = unsafe { core::ptr::read_volatile(&self.value_ghost as *const T) };
        if v == v_ghost { Some(v) } else { None }
    }

    pub fn set(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(&mut self.value as *mut T, value) };
        unsafe { core::ptr::write_volatile(&mut self.value_ghost as *mut T, value) };
    }
}

impl<T> BitFlipGuard2<T>
where
    T: Copy + Eq + PartialEq,
{
    pub fn mock_all_one_v(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value as *mut T, 0xFFu8, 1) };
    }

    pub fn mock_all_zero_v(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value as *mut T, 0x00u8, 1) };
    }

    pub fn mock_all_one_ghost(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value_ghost as *mut T, 0xFFu8, 1) };
    }

    pub fn mock_all_zero_ghost(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value_ghost as *mut T, 0x00u8, 1) };
    }
}
