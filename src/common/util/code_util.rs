use rand::{Rng, rng};

pub struct CodeUtil;

impl CodeUtil {
    /// 生成指定长度的数字组成的字符串
    pub fn generate_num_code(len: u32) -> String {
        if len == 0 {
            return String::new();
        }
        let mut rng = rng();

        let mut builder = String::new();
        for _i in 0..len {
            let rand = rng.random_range(0..10);
            builder.push_str(&rand.to_string());
        }
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        for i in 1..20 {
            let code = CodeUtil::generate_num_code(i);

            // println!("code -> {}", code);
            assert_eq!(code.len(), i as usize);
        }
    }
}
