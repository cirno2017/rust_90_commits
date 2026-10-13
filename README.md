1. 换算函数的输入单位是mv(毫伏)，输出单位是uv(微伏)，输入范围就是i32的范围，从-2_147_483_648_000到2_147_483_647_000，输出范围则是乘以1000，是-2_147_483_648_000_000到2_147_483_647_000_000.
2. 五组输入: 输入0，输出预期结果0; 输入1500，输出预期结果1_500_000; 输入-25，输出预期结果-25_000; 输入i32::MIN，输出预期结果-2_147_483_648_000; 输入i32::MAX，输出预期结果2_147_483_647_000.
3. cargo run是运行正式的代码，不会运行#[test]这样的测试单元代码，cargo test是运行测试单元代码，不运行正式的代码.
4. 为什么零值测试没有发现比例错误? 因为零值测试是输入0，不管与什么相乘最终都输出0，和函数内部的相乘系数无关，所以不能检测出比例错误.
5. 修复后的测试输出为
running 4 tests
test tests::converts_i32_limits ... ok
test tests::converts_negative_mv ... ok
test tests::converts_positive_mv ... ok
test tests::zero_stays_zero ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s