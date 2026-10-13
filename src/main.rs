//
fn mv_to_uv(mv: i32) -> i64 {
    // TODO：先把 mv 扩宽为 i64，再乘以 1000。
    // TODO：用尾表达式返回结果。
    mv as i64 * 1000
}

fn main() {
    let input_mv: i32 = 1500;
    let output_uv = mv_to_uv(input_mv);

    println!("mv={input_mv}, uv={output_uv}");
}

#[cfg(test)]
mod tests {
    use super::mv_to_uv;

    #[test]
    fn zero_stays_zero() {
        let actual = mv_to_uv(0);
        let expected: i64 = 0_i64;
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_positive_mv() {
        // TODO：检查 mv_to_uv(1500)。
        let actual = mv_to_uv(1500);
        let expected: i64 = 1_500_000_i64;
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_negative_mv() {
        // TODO：检查 mv_to_uv(-25)。
        let actual = mv_to_uv(-25);
        let expected: i64 = -25_000_i64;
        assert_eq!(actual, expected);
    }

    #[test]
    fn converts_i32_limits() {
        // TODO：检查 mv_to_uv(i32::MIN)。
        // TODO：检查 mv_to_uv(i32::MAX)。
        let actual = mv_to_uv(i32::MIN);
        let expected: i64 = -2_147_483_648_000_i64;
        assert_eq!(actual, expected);

        let actual = mv_to_uv(i32::MAX);
        let expected: i64 = 2_147_483_647_000_i64;
        assert_eq!(actual, expected);
    }
}
