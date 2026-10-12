//
// 返回顺序：(UTF-8 字节数, Unicode 标量值数量)
fn text_stats(text: &str) -> (usize, usize) {
    let mut char_count: usize = 0;

    for _ch in text.chars() {
        // TODO：将 char_count 加 1。
        char_count += 1;
    }
    // TODO：用尾表达式返回两个统计值。
    (text.len(), char_count)
}
//
fn show_units(text: &str) {
    for ch in text.chars() {
        // TODO：每行打印 char={ch}
        println!("char={ch}");
    }

    for byte in text.bytes() {
        // TODO：每行打印 byte={byte}
        println!("byte={byte}");
    }
}
//
fn main() {
    let base: &str = "温度";
    let mut label = String::from(base);

    // TODO：使用 push 追加字符 '-'。
    // TODO：使用 push_str 追加字符串 "A"。
    label.push('-');
    label.push_str("A");

    println!("base={base}");
    println!("label={label}");

    let (strbytes, strchars) = text_stats(base);
    println!("base: byte={strbytes}, chars={strchars}");
    let (strbytes, strchars) = text_stats(&label);
    println!("label: byte={strbytes}, chars={strchars}");
    println!("source_label={label}");
    show_units(&label);

    let string_array: [&str; 5] = ["", "ADC", "温度", "温度-A", "🙂"];
    for str_temp in string_array {
        let (strbytes, strchars) = text_stats(str_temp);
        println!("String \"{str_temp}\": byte={strbytes}, chars={strchars}");
        show_units(str_temp);
    }
}
