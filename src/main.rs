//
// 本练习只使用下方给定的小数组，累计值保证落在 i64 范围内。
// 返回顺序：(元素数量, 读数总和, 报警数量)
fn summarize(samples: &[i32], threshold_mv: i32) -> (usize, i64, usize) {
    let mut sum_mv: i64 = 0;
    let mut alarms: usize = 0;

    for &value in samples {
        // TODO：先将 value 扩宽为 i64，再累加到 sum_mv。
        sum_mv += value as i64;

        // TODO：value >= threshold_mv 时，将 alarms 加 1。
        if value >= threshold_mv {
            alarms += 1;
        }
    }
    // TODO：用尾表达式返回三个统计值组成的元组。
    (samples.len(), sum_mv, alarms)
}

fn main() {
    let samples: [i32; 5] = [1500, 1600, 1700, 1800, 1900];
    let empty: [i32; 0] = [];
    let single: [i32; 1] = [1700];
    let signed: [i32; 3] = [-100, 0, 100];
    let large: [i32; 2] = [i32::MAX, i32::MAX];
    let threshold_mv: i32 = 1700;

    let (count, sum_mv, alarms) = summarize(&samples, threshold_mv);
    println!("samples: count={count}, sum_mv={sum_mv}, alarms={alarms}");

    let (count, sum_mv, alarms) = summarize(&samples[1..4], threshold_mv);
    println!("samples[1..4]: count={count}, sum_mv={sum_mv}, alarms={alarms}");

    let (count, sum_mv, alarms) = summarize(&samples[2..2], threshold_mv);
    println!("samples[2..2]: count={count}, sum_mv={sum_mv}, alarms={alarms}");

    let (count, sum_mv, alarms) = summarize(&empty, threshold_mv);
    println!("empty: count={count}, sum_mv={sum_mv}, alarms={alarms}");

    let (count, sum_mv, alarms) = summarize(&single, threshold_mv);
    println!("single: count={count}, sum_mv={sum_mv}, alarms={alarms}");

    let (count, sum_mv, alarms) = summarize(&signed, 0);
    println!("signed: count={count}, sum_mv={sum_mv}, alarms={alarms}");

    let (count, sum_mv, alarms) = summarize(&large, threshold_mv);
    println!("large: count={count}, sum_mv={sum_mv}, alarms={alarms}");
}
