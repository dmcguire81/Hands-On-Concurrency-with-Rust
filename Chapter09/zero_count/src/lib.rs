/// # Safety
/// 
/// This function needs to be supplied with an arr parameter that is at least as long
/// as the len supplied, lest a buffer overrun will result
#[no_mangle]
pub unsafe extern "C" fn tail_zero_count(arr: *const u16, len: usize) -> u64 {
    let mut zeros: u64 = 0;
    for i in 0..len {
        zeros += (*arr.offset(i as isize)).trailing_zeros() as u64
    }
    zeros
}
