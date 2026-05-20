use std::cmp::Ordering;

pub fn compare_version(a: &str, b: &str) -> Ordering {
    // 将分割、解析、非数字降级融为一体
    let a_parts: Vec<i32> = a.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let b_parts: Vec<i32> = b.split('.').map(|s| s.parse().unwrap_or(0)).collect();

    // 取两者的最大长度，对应 Go 里面的 n = max(len(as), len(bs))
    let max_len = std::cmp::max(a_parts.len(), b_parts.len());

    for i in 0..max_len {
        // 使用 .get() 安全地按索引取值，如果越界则直接 fallback 到 0
        let ai = a_parts.get(i).copied().unwrap_or(0);
        let bi = b_parts.get(i).copied().unwrap_or(0);

        match ai.cmp(&bi) {
            Ordering::Equal => continue,
            other => return other, // 只要分出大小，立刻熔断返回
        }
    }

    Ordering::Equal
}
