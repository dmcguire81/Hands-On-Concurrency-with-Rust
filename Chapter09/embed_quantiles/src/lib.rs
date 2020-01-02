extern crate quantiles;

use quantiles::ckms::CKMS;

#[no_mangle]
pub extern "C" fn alloc_ckms(error: f64) -> *mut CKMS<f32> {
    let ckms = Box::new(CKMS::new(error));
    Box::into_raw(ckms)
}

/// # Safety
///
/// This function should only be called on a struct allocated with alloc_ckms, and only once.
#[no_mangle]
pub unsafe extern "C" fn free_ckms(ckms: *mut CKMS<f32>) {
    let ckms = Box::from_raw(ckms);
    drop(ckms);
}

#[no_mangle]
pub extern "C" fn ckms_insert(ckms: &mut CKMS<f32>, value: f32) {
    ckms.insert(value)
}

/// # Safety
///
/// This function should only be called on a struct allocated with alloc_ckms.
#[no_mangle]
pub unsafe extern "C" fn query(ckms: &mut CKMS<f32>, q: f64, quant: *mut f32) -> i8 {
    if let Some((_, res)) = ckms.query(q) {
        *quant = res;
        0
    } else {
        -1
    }
}
