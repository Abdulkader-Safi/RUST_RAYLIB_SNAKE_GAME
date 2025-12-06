use raylib::ffi::GetRandomValue;

pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { GetRandomValue(min, max) }
}
