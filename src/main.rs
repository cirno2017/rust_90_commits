//
// 调用约定：sample_count 已由 main 检查，范围为 0..=5。
fn run_sampling(sample_count: u32, threshold_mv: u32) -> u32 {
    let mut alarms: u32 = 0;

    for index in 0..sample_count {
        // TODO：根据 index 计算 voltage_mv，类型为 u32。
        let voltage_mv: u32 = 1500 + index * 100;
        // TODO：使用 if 表达式计算 alarm，类型为 u32：
        // 达到或超过 threshold_mv 时为 1，否则为 0。
        let alarm: u32 = if voltage_mv >= threshold_mv { 1 } else { 0 };
        // TODO：将本次 alarm 累加到 alarms。
        alarms += alarm;
        println!("index={index}, voltage_mv={voltage_mv}, alarm={alarm}");
    }

    alarms
}

fn main() {
    let sample_count: u32 = 5;
    let threshold_mv: u32 = 1700;

    // TODO：如果 sample_count > 5，
    // 打印 "invalid sample_count: expected 0..=5"
    // 然后用 return; 退出 main。
    if sample_count > 5 {
        println!("invalid sample_count: expected 0..=5");
        return;
    }

    // TODO：调用 run_sampling，保存返回的报警总数 alarms。
    let alarms: u32 = run_sampling(sample_count, threshold_mv);

    println!("samples={sample_count}, alarms={alarms}");
}
