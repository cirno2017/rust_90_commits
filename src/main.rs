//
fn main() {
    let sensor_id: u32 = 1;
    let mut sample_count: u32 = 0;
    println!("Initial count: {sample_count}");

    let reading: i32 = 253;
    println!("Sensor {sensor_id}, raw: {reading}");

    // TODO：更新已有 sample_count，使其增加 1；这里不要再次使用 let。
    sample_count = sample_count + 1;
    // TODO：使用 let 遮蔽 reading，将其转换为 f64 类型的摄氏温度。
    let reading: f64 = reading as f64 / 10.0;

    println!("Temperature: {reading:.1} C");
    println!("Samples: {sample_count}");

    {
        // TODO：建立同名新绑定 reading，在当前温度上加 0.5。
        let reading: f64 = reading + 0.5;
        println!("Adjusted inside: {reading:.1} C");
    }

    println!("Temperature outside: {reading:.1} C");
}
