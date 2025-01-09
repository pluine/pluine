use crate::*;

pub struct Identifier(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_not_a_valid_number() {
        // +i and -i and ifnan
        // 〈infnan〉 −→ +inf.0 | -inf.0 | +nan.0 | -nan.0
        // (i is valid)
        // numbers may be both upper and lower case
    }
}
