/// kernel virtual memory region
#[derive(Copy, Clone)]
pub struct region_t {
    pub start: usize,
    pub end: usize,
}

/// physical virtual memory region
#[derive(Copy, Clone, Debug)]
pub struct p_region_t {
    pub start: usize,
    pub end: usize,
}

/// user virtual memory region
#[derive(Copy, Clone)]
pub struct v_region_t {
    pub start: usize,
    pub end: usize,
}
