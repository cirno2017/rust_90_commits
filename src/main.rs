//
fn main() {
    let raw: u16 = 2048;
    let reference_mv: u32 = 3300;
    let adc_max: u32 = 4095;

    // TODO：先把 raw 转为 u32，再乘 reference_mv，
    // 将乘积保存为 scaled，类型为 u32。
    let scaled: u32 = raw as u32 * reference_mv;

    // TODO：用 scaled 除以 adc_max，
    // 将结果保存为 voltage_mv，类型为 u32。
    let voltage_mv: u32 = scaled / adc_max;

    println!("raw={raw}, scaled={scaled}, voltage_mv={voltage_mv}");

    let counter: u8 = 250;
    let increment: u8 = 10;

    let checked = counter.checked_add(increment);
    let wrapped = counter.wrapping_add(increment);
    let saturated = counter.saturating_add(increment);

    println!("counter={counter}, increment={increment}");
    println!("checked={checked:?}");
    println!("wrapped={wrapped}");
    println!("saturated={saturated}");
}
