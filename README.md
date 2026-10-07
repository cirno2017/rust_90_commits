在 sensor_id 定义后加入 sensor_id = 2;报错原因是let sensor_id: u32 = 1;说明sensor_id是一个不可变变量，sensor_id = 2是修改原本的值，违反了不可变变量的定义，所以报错。
在 sample_count 定义后加入 sample_count = 1.5;报错原因是let mut sample_count: u32 = 0;定义了sample_count是32位整数类型，sample_count = 1.5;用f64类型对sample_count进行赋值，类型不符合，所以报错。
let reading: f64 = reading as f64 / 10.0;可以成立是因为对reading进行了重新定义，把原本的reading遮蔽掉了，后面使用reading都按照新的类型新的值进行，所以可以成立。