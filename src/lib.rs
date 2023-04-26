pub(crate) use std::time;
use std::u128;

/// Send the given hex string along to the output_pin_control- true when it should be on, false when off.
pub fn send_ir_hex_nec_protocol<F>(hex_string: &str, mut output_pin_control: F)
where
    F: FnMut(bool),
{
    let without_prefix = hex_string.trim_start_matches("0x");
    // Number of bits to process = 4 per hex val
    let num_bits = without_prefix.len() * 4;
    let mut input_bits = u128::from_str_radix(without_prefix, 16).unwrap();

    output_pin_control(true);
    wait_for_time(9000.0);
    output_pin_control(false);
    wait_for_time(4500.0);

    for _n in 0..num_bits {
        match get_next_value_and_move_pointer(&mut input_bits) {
            true => send_one(&mut output_pin_control),
            false => send_zero(&mut output_pin_control)
        }
    }
}

fn send_zero<F>( pin_control: &mut F) where F: FnMut(bool) {
    pin_control(true);
    wait_for_time(562.5);
    pin_control(false);
    wait_for_time(562.5);
}

fn send_one<F>( pin_control: &mut F) where F: FnMut(bool) {
    pin_control(true);
    wait_for_time(562.5);
    pin_control (false);
    wait_for_time(1687.5);
}

/// read first bit and shift right by 1.
fn get_next_value_and_move_pointer(input: &mut u128) -> bool {
    let val = (*input & 1) != 0;
    *input = *input >> 1;
    val
}

/// A BLOCKING process that sets the given function to true, waits the allotted time, and sets to false.
fn wait_for_time(time_in_microseconds: f64) {
    let start = time::Instant::now();
            while start.elapsed().as_nanos() < (time_in_microseconds * 1000.0) as u128 {
                continue;
            }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wait_for_time_waits_exact_amount_of_time() {
        const RUN_TIME: f64 = 100.0;
        let start = time::Instant::now();
        wait_for_time(RUN_TIME);
        let passed = start.elapsed().as_micros();
        assert!(passed >= RUN_TIME as u128 && passed < RUN_TIME as u128 + 2);
    }
}
