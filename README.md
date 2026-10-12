1. String代表字符串本身，&str代表字符串的引用;
2. 5组输出分别为:
String "": byte=0, chars=0
String "ADC": byte=3, chars=3
char=A
char=D
char=C
byte=65
byte=68
byte=67
String "温度": byte=6, chars=2
char=温
char=度
byte=230
byte=184
byte=169
byte=229
byte=186
byte=166
String "温度-A": byte=8, chars=4
char=温
char=度
char=-
char=A
byte=230
byte=184
byte=169
byte=229
byte=186
byte=166
byte=45
byte=65
String "🙂": byte=4, chars=1
char=🙂
byte=240
byte=159
byte=153
byte=130
3. 为什么 "温度-A" 是 8 个字节、4 个 Unicode 标量值? 因为Unicode字符格式，每2个字节表示1个字符，"温度-A"是4个字符，所以是8个字节，Unicode 标量值表示的是实际可读的字符数，所以是4个。
4. println!("{}", label[0]);故障，是因为字符串不支持整数索引;println!("{}", &label[0..1]);故障，是因为&label[0..1]表示取第1个字节，但是这是Unicode格式，2个字节表示1个字符，不能单独取1个字节。
5. chars() 数量不保证等于用户看到的完整字符数量。是因为chars() 计算的是实际占用字节数，一旦碰到多字节字符格式，就会发生chars() 数量不等于实际看到的字符数。