//! Python's `random.Random` (MT19937 with CPython's seeding and its algorithms for getrandbits,
//! random, shuffle, choice, sample, ...): a game record, the card creation order and the random
//! bot's choices then come out exactly as with the Python package.

const N: usize = 624;
const M: usize = 397;

#[derive(Clone)]
pub struct PyRandom {
    mt: [u32; N],
    index: usize,
}

impl PyRandom {
    /// `random.Random(seed)` for an int seed (its absolute value is used).
    pub fn new(seed: i128) -> PyRandom {
        let mut a = seed.unsigned_abs();
        let mut key = Vec::new();
        while a > 0 {
            key.push(a as u32);
            a >>= 32;
        }
        if key.is_empty() {
            key.push(0);
        }
        let mut r = PyRandom { mt: [0; N], index: N };
        r.init_by_array(&key);
        r
    }

    fn init_genrand(&mut self, s: u32) {
        self.mt[0] = s;
        for i in 1..N {
            self.mt[i] = 1812433253u32.wrapping_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30)).wrapping_add(i as u32);
        }
        self.index = N;
    }

    fn init_by_array(&mut self, key: &[u32]) {
        self.init_genrand(19650218);
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..N.max(key.len()) {
            self.mt[i] = (self.mt[i] ^ (self.mt[i - 1] ^ (self.mt[i - 1] >> 30)).wrapping_mul(1664525))
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= N {
                self.mt[0] = self.mt[N - 1];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..N - 1 {
            self.mt[i] = (self.mt[i] ^ (self.mt[i - 1] ^ (self.mt[i - 1] >> 30)).wrapping_mul(1566083941)).wrapping_sub(i as u32);
            i += 1;
            if i >= N {
                self.mt[0] = self.mt[N - 1];
                i = 1;
            }
        }
        self.mt[0] = 0x8000_0000;
    }

    fn genrand(&mut self) -> u32 {
        if self.index >= N {
            for k in 0..N {
                let y = (self.mt[k] & 0x8000_0000) | (self.mt[(k + 1) % N] & 0x7fff_ffff);
                let mag = if y & 1 != 0 { 0x9908_b0df } else { 0 };
                self.mt[k] = self.mt[(k + M) % N] ^ (y >> 1) ^ mag;
            }
            self.index = 0;
        }
        let mut y = self.mt[self.index];
        self.index += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^ (y >> 18)
    }

    /// `random()`: a float in [0, 1) with 53 random bits.
    pub fn random(&mut self) -> f64 {
        let a = (self.genrand() >> 5) as f64;
        let b = (self.genrand() >> 6) as f64;
        (a * 67108864.0 + b) / 9007199254740992.0
    }

    /// `getrandbits(k)` for k <= 128.
    pub fn getrandbits(&mut self, k: u32) -> u128 {
        if k == 0 {
            return 0;
        }
        if k <= 32 {
            return (self.genrand() >> (32 - k)) as u128;
        }
        let (mut out, mut left, mut shift) = (0u128, k, 0);
        while left > 0 {
            let mut r = self.genrand();
            if left < 32 {
                r >>= 32 - left;
            }
            out |= (r as u128) << shift;
            shift += 32;
            left = left.saturating_sub(32);
        }
        out
    }

    /// `_randbelow(n)`: an int in [0, n), for n > 0.
    pub fn randbelow(&mut self, n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let k = 64 - n.leading_zeros();
        loop {
            let r = self.getrandbits(k) as u64;
            if r < n {
                return r;
            }
        }
    }

    /// `randrange(n)`
    pub fn randrange(&mut self, n: usize) -> usize {
        assert!(n > 0, "empty range for randrange");
        self.randbelow(n as u64) as usize
    }

    /// `randint(a, b)`: an int in [a, b].
    pub fn randint(&mut self, a: i64, b: i64) -> i64 {
        assert!(b >= a, "empty range for randint");
        a + self.randbelow((b - a + 1) as u64) as i64
    }

    /// `choice(seq)`
    pub fn choice<'a, T>(&mut self, seq: &'a [T]) -> &'a T {
        assert!(!seq.is_empty(), "cannot choose from an empty sequence");
        &seq[self.randbelow(seq.len() as u64) as usize]
    }

    /// `shuffle(x)`
    pub fn shuffle<T>(&mut self, x: &mut [T]) {
        for i in (1..x.len()).rev() {
            let j = self.randbelow(i as u64 + 1) as usize;
            x.swap(i, j);
        }
    }

    /// `sample(population, k)`
    pub fn sample<T: Clone>(&mut self, population: &[T], k: usize) -> Vec<T> {
        let n = population.len();
        assert!(k <= n, "sample larger than population");
        let mut setsize = 21;
        if k > 5 {
            setsize += 4usize.pow(log4_ceil(k * 3));
        }
        let mut result = Vec::with_capacity(k);
        if n <= setsize {
            let mut pool = population.to_vec();
            for i in 0..k {
                let j = self.randbelow((n - i) as u64) as usize;
                result.push(pool[j].clone());
                pool[j] = pool[n - i - 1].clone();
            }
        } else {
            let mut selected = std::collections::HashSet::new();
            for _ in 0..k {
                let mut j = self.randbelow(n as u64) as usize;
                while selected.contains(&j) {
                    j = self.randbelow(n as u64) as usize;
                }
                selected.insert(j);
                result.push(population[j].clone());
            }
        }
        result
    }
}

/// `_ceil(_log(x, 4))` as Python computes it (log(x) / log(4) in floats).
fn log4_ceil(x: usize) -> u32 {
    ((x as f64).ln() / 4f64.ln()).ceil() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The values CPython gives for the same calls.
    #[test]
    fn matches_python() {
        let mut r = PyRandom::new(42);
        assert_eq!(r.getrandbits(32), 2746317213);
        assert_eq!(r.random(), 0.11133106816568039);
        assert_eq!(r.getrandbits(64), 5073395517033431291);
        assert_eq!(r.getrandbits(70), 151691464061534419856);
        let mut r = PyRandom::new(-(1i128 << 64));
        let mut x: Vec<u32> = (0..30).collect();
        r.shuffle(&mut x);
        assert_eq!(x, [18, 10, 22, 20, 8, 12, 17, 1, 26, 29, 24, 27, 6, 5, 21, 9, 7, 14, 0, 28, 13, 3, 16, 25, 4, 19, 15, 2, 11, 23]);
        assert_eq!(r.sample(&(0..100).collect::<Vec<u32>>(), 10), [37, 22, 74, 99, 98, 42, 97, 66, 92, 43]);
        assert_eq!(
            r.sample(&(0..1000).collect::<Vec<u32>>(), 30),
            [622, 443, 621, 797, 504, 633, 800, 741, 57, 406, 305, 644, 923, 110, 41, 108, 738, 819, 267, 118, 973, 664, 222, 152, 337, 596, 357, 323, 471, 344]
        );
        assert_eq!(r.randint(1, 3), 2);
        assert_eq!(*r.choice(&['a', 'b', 'c']), 'a');
    }
}
