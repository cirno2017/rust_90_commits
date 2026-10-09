//
fn adc_to_mv(raw: u16) -> u32 {
    // TODO：先扩宽 raw，再计算乘积，保存为 scaled。
    // TODO：用尾表达式返回 scaled / 4095。
    let scaled: u32 = {
        // TODO：定义局部变量 wide，将 raw 扩宽为 u32。
        // TODO：以 wide 与 3300 的乘积作为这个块的值。
        let wide: u32 = raw as u32;
        wide * 3300
    };
    return scaled / 4095;
}

fn mv_to_volts(millivolts: u32) -> f64 {
    // TODO：先转为 f64，再除以 1000.0。
    // 使用尾表达式返回结果。
    millivolts as f64 / 1000.0
}

fn main() {
    let raw: u16 = 2048;

    // TODO：调用 adc_to_mv，将结果保存为 voltage_mv。
    // TODO：调用 mv_to_volts，将结果保存为 voltage_v。
    let voltage_mv: u32 = adc_to_mv(raw);
    let voltage_v: f64 = mv_to_volts(voltage_mv);

    println!("raw={raw}, voltage_mv={voltage_mv}, voltage_v={voltage_v:.3}");
    println!("direct conversion: {:.3}", mv_to_volts(1));
}
