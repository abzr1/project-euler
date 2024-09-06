fn main() {
    let mut a = BigInt::new([0; 305]);
    a.digits[0] = 2;

    let mut sum = 0;

    a.pow(1000).digits.iter().for_each(|&d| sum += d as i32);

    println!("{}", sum);
}

#[derive(Debug, Copy, Clone)]
struct BigInt {
    digits: [u8; 305],
}

impl BigInt {
    fn new(digits: [u8; 305]) -> Self {
        Self { digits }
    }

    fn add(self, other: Self) -> Self {
        let mut new: [u8; 305] = [0; 305];

        let mut carry: u8 = 0;

        for (i, (s, o)) in self.digits.iter().zip(other.digits.iter()).enumerate() {
            if s + o + carry > 9 {
                new[i] = (s + o + carry) - 10;
                carry = 1;
            } else {
                new[i] = s + o + carry;
                carry = 0;
            }
        }

        Self { digits: new }
    }

    fn decr(&mut self) -> bool {
        let mut i: usize = 0;

        while i < self.digits.len() {
            if self.digits[i] != 0 {
                self.digits[i] -= 1;
                return true;
            }

            i += 1;
        }

        false
    }

    fn mul(self, mut other: Self) -> Self {
        let mut new = BigInt::new([0; 305]);

        while other.decr() {
            new = new.add(self);
        }

        new
    }

    fn pow(self, index: u32) -> Self {
        let mut new = self;

        for _ in 1..index {
            new = new.mul(self);
        }

        new
    }
}
