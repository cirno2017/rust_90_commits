1. [i32; 5] 与 &[i32] 的区别。[i32; 5]表示类型为i32长度为5的数组，是数组本身。&[i32]不是数组，是数组的一个引用。
2. summarize函数的返回类型是元组(usize, i64, usize)，分别表示元素数量, 读数总和, 报警数量。
3. 各数组的实际输出为：
samples: count=5, sum_mv=8500, alarms=3
samples[1..4]: count=3, sum_mv=5100, alarms=2
samples[2..2]: count=0, sum_mv=0, alarms=0
empty: count=0, sum_mv=0, alarms=0
single: count=1, sum_mv=1700, alarms=1
signed: count=3, sum_mv=0, alarms=2
large: count=2, sum_mv=4294967294, alarms=2
4. 为什么空输入能正常返回，以及故意读取 samples[0] 后发生了什么？空输入表示没有任何输入，此时samples.len()的值是0，sum_mv和alarms都取默认值0，所以可以返回元组(0, 0, 0)，故意读取 samples[0]表示读取空输入的第一个元素，由于空输入没有任何元素，读取第一个元素属于越界，所以会报错。
5. 为什么累计之前需要扩宽为 i64？为了防止i32加法溢出越界，把单个i32的value先转成更宽的i64，再累加进sum_mv: i64，保证总和不会溢出。