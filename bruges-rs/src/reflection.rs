//! Reflection Module
//! Tony Hallam 2021

/// Reflection Module

/// Calculate the critical angle at an interface
#[no_mangle]
pub fn critical_angle(vel1: f64, vel2: f64) -> f64 {
    if vel1 < vel2 {
        return (vel1 / vel2).asin();
    } else {
        return -1.0;
    }
}

#[cfg(test)]
mod test_reflection {

    use super::*;

    fn test_critical_anlge() {
        assert_eq!(-1.0, critical_angle(10.0, 10.0))
    }
}
