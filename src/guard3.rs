#[derive(Debug)]
pub struct BitFlipGuard3<T>
where
    T: Copy + Eq + PartialEq,
{
    value: T,
    value_ghost1: T,
    value_ghost2: T,
}

impl<T> BitFlipGuard3<T>
where
    T: Copy + Eq + PartialEq,
{
    pub const fn new(value: T) -> Self {
        Self {
            value,
            value_ghost1: value,
            value_ghost2: value,
        }
    }

    pub fn get(&self) -> Option<T> {
        let v = unsafe { core::ptr::read_volatile(&self.value as *const T) };
        let v_ghost1 = unsafe { core::ptr::read_volatile(&self.value_ghost1 as *const T) };
        let v_ghost2 = unsafe { core::ptr::read_volatile(&self.value_ghost2 as *const T) };
        if v == v_ghost1 && v == v_ghost2 {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_majority(&self) -> Option<T> {
        let v = unsafe { core::ptr::read_volatile(&self.value as *const T) };
        let v_ghost1 = unsafe { core::ptr::read_volatile(&self.value_ghost1 as *const T) };
        let v_ghost2 = unsafe { core::ptr::read_volatile(&self.value_ghost2 as *const T) };
        if v == v_ghost1 || v == v_ghost2 {
            Some(v)
        } else if v_ghost1 == v_ghost2 {
            Some(v_ghost1)
        } else {
            None
        }
    }

    pub fn set(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(&mut self.value as *mut T, value) };
        unsafe { core::ptr::write_volatile(&mut self.value_ghost1 as *mut T, value) };
        unsafe { core::ptr::write_volatile(&mut self.value_ghost2 as *mut T, value) };
    }
}

impl<T> BitFlipGuard3<T>
where
    T: Copy + Eq + PartialEq,
{
    pub fn mock_all_one_v(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value as *mut T, 0xFFu8, 1) };
    }

    pub fn mock_all_zero_v(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value as *mut T, 0x00u8, 1) };
    }

    pub fn mock_all_one_ghost1(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value_ghost1 as *mut T, 0xFFu8, 1) };
    }

    pub fn mock_all_zero_ghost1(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value_ghost1 as *mut T, 0x00u8, 1) };
    }

    pub fn mock_all_one_ghost2(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value_ghost2 as *mut T, 0xFFu8, 1) };
    }

    pub fn mock_all_zero_ghost2(&mut self) {
        unsafe { core::ptr::write_bytes(&mut self.value_ghost2 as *mut T, 0x00u8, 1) };
    }
}
