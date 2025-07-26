use std::{collections::HashSet, sync::Arc};

const RNG_STATE_SIZE: usize = 16;

#[derive(Clone)]
pub struct RNG {
    random_poly: u32,
    index: usize,
    pub num_calls: usize,
    state: [u32; RNG_STATE_SIZE],
}

// pub type PrecomputedRNG = RNG;

#[derive(Clone)]
pub struct PrecomputedRNG {
    values: Arc<Vec<u32>>,
    pub ptr: usize,
}

impl Eq for PrecomputedRNG {}

impl PartialEq for PrecomputedRNG {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.values, &other.values) && self.ptr == other.ptr
    }
}

impl PrecomputedRNG {
    pub fn new(
        seed: u32,
        seeds_15bit: bool,
        seeds_signed: bool,
        use_old_random_poly: bool,
        num_values: usize,
    ) -> Self {
        let mut rng = RNG::new(seed, seeds_15bit, seeds_signed, use_old_random_poly);

        let mut values: Vec<u32> = Vec::with_capacity(num_values);
        for _ in 0..num_values {
            values.push(rng.next_u32());
        }

        Self {
            values: values.into(),
            ptr: 0,
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        let r = self.values[self.ptr];
        self.ptr += 1;

        r
    }

    // maybe cache this
    pub fn next_f64(&mut self, range: f64) -> f64 {
        return (self.next_u32() as f64) * 2.3283064365386963e-10 * range;
    }

    pub fn action_move(&mut self, dir: &str) {
        if dir.as_bytes().contains(&49) {
            // Call irandom until it lands on a byte that's '1' rather than '0'
            loop {
                let index = (self.next_u32() % 9) as usize;
                if dir.as_bytes()[index] == 49 {
                    break;
                }
            }
        }
    }
}

struct RNGIterator {
    rng: RNG,
    range: f64,
}

impl Iterator for RNGIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.next_f64(self.range))
    }
}

impl PartialEq for RNG {
    fn eq(&self, other: &Self) -> bool {
        self.random_poly == other.random_poly
            && self.index == other.index
            && self.state == other.state
    }
}
impl Eq for RNG {}
impl RNG {
    pub fn new(
        mut seed: u32,
        seeds_15bit: bool,
        seeds_signed: bool,
        use_old_random_poly: bool,
    ) -> RNG {
        let mut rng: RNG = RNG {
            random_poly: if use_old_random_poly {
                0xda442d20
            } else {
                0xda442d24
            },
            index: 0,
            num_calls: 0,
            state: [0; RNG_STATE_SIZE],
        };

        // Generate initial state
        rng.index = 0;
        if seeds_15bit {
            for i in 0..RNG_STATE_SIZE {
                seed = u32::wrapping_shr(
                    u32::wrapping_add(u32::wrapping_mul(seed, 0x343fd), 0x269ec3),
                    16,
                ) & 0x7fff;
                rng.state[i] = seed;
            }
        } else if seeds_signed {
            let mut signed_seed = seed as i32;
            for i in 0..RNG_STATE_SIZE {
                signed_seed = i32::wrapping_shr(
                    i32::wrapping_add(i32::wrapping_mul(signed_seed, 0x343fd), 0x269ec3),
                    16,
                ) & 0x7fffffff;
                rng.state[i] = signed_seed as u32;
            }
        } else {
            let mut signed_seed = seed as i32;
            signed_seed = i32::wrapping_shr(
                i32::wrapping_add(i32::wrapping_mul(signed_seed, 0x343fd), 0x269ec3),
                16,
            ) & 0x7fffffff;
            rng.state[0] = signed_seed as u32;
            for i in 1..RNG_STATE_SIZE {
                signed_seed = u32::wrapping_shr(
                    i32::wrapping_add(i32::wrapping_mul(signed_seed, 0x343fd), 0x269ec3) as u32,
                    16,
                ) as i32;
                rng.state[i] = signed_seed as u32;
            }
        }

        rng
    }

    pub fn action_move(&mut self, dir: &str) {
        if dir.as_bytes().contains(&49) {
            // Call irandom until it lands on a byte that's '1' rather than '0'
            loop {
                let index = (self.next_u32() % 9) as usize;
                if dir.as_bytes()[index] == 49 {
                    break;
                }
            }
        }
    }

    pub fn into_range_iter(self, range: f64) -> impl Iterator<Item = f64> {
        return RNGIterator { range, rng: self };
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut a: u32 = self.state[self.index];
        let mut b: u32 = self.state[(self.index + 13) & 15];
        let c: u32 = a ^ b ^ u32::wrapping_shl(a, 16) ^ u32::wrapping_shl(b, 15);
        b = self.state[(self.index + 9) & 15];
        b ^= u32::wrapping_shr(b, 11);
        a = c ^ b;
        self.state[self.index] = a;
        let d: u32 = a ^ (u32::wrapping_shl(a, 5) & self.random_poly);
        self.index = (self.index + 15) & 15;
        a = self.state[self.index];
        self.state[self.index] = a
            ^ c
            ^ d
            ^ u32::wrapping_shl(a, 2)
            ^ u32::wrapping_shl(c, 18)
            ^ u32::wrapping_shl(b, 28);
        self.num_calls += 1;

        self.state[self.index]
    }

    pub fn next_f64(&mut self, range: f64) -> f64 {
        return (self.next_u32() as f64) * 2.3283064365386963e-10 * range;
    }

    pub fn calculate_unique_seeds(seeds_15bit: bool, seeds_signed: bool) -> Vec<u32> {
        let unique_state_count: usize = if seeds_15bit { 32768 } else { 65536 };
        let mut unique_seeds_list: Vec<u32> = Vec::with_capacity(unique_state_count);
        let mut unique_states: HashSet<u32> = HashSet::with_capacity(unique_state_count);

        if seeds_15bit {
            let mut curr_seed: u32 = 0;
            while unique_states.len() < unique_state_count {
                let state: u32 = u32::wrapping_shr(
                    u32::wrapping_add(u32::wrapping_mul(curr_seed, 0x343fd), 0x269ec3),
                    16,
                ) & 0x7fff;
                if unique_states.insert(state) {
                    unique_seeds_list.push(curr_seed);
                }
                curr_seed += 1;
            }
        } else if seeds_signed {
            let mut curr_seed: u32 = 0;
            while unique_states.len() < unique_state_count {
                let state: u32 = (i32::wrapping_shr(
                    i32::wrapping_add(i32::wrapping_mul(curr_seed as i32, 0x343fd), 0x269ec3),
                    16,
                ) & 0x7fffffff) as u32;
                if unique_states.insert(state) {
                    unique_seeds_list.push(curr_seed);
                }
                curr_seed += 1;
            }
        } else {
            // TODO: is this correct?
            let mut curr_seed: u32 = 0;
            while unique_states.len() < unique_state_count {
                let state: u32 = u32::wrapping_shr(
                    u32::wrapping_add(u32::wrapping_mul(curr_seed, 0x343fd), 0x269ec3),
                    16,
                );
                if unique_states.insert(state) {
                    unique_seeds_list.push(curr_seed);
                }
                curr_seed += 1;
            }
        }

        unique_seeds_list
    }
}
