pub(crate) trait IntoUsize: Copy {
    fn into_usize(self) -> usize;
}

impl IntoUsize for u16 {
    fn into_usize(self) -> usize {
        self as usize
    }
}

impl IntoUsize for u32 {
    fn into_usize(self) -> usize {
        self as usize
    }
}
