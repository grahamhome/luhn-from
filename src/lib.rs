mod tests;

pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(sum, count), v| {
                v.to_digit(10)
                    .map(|n| if count % 2 == 1 { n * 2 } else { n })
                    .map(|n| if n > 9 { n - 9 } else { n })
                    .map(|n| (sum + n, count + 1))
            })
            .map_or(false, |(sum, count)| count > 1 && sum % 10 == 0)
    }
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
