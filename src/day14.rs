use bit_field::BitField;
use hashbrown::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct MemData {
    address: u64,
    value: u64,
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Mask(Vec<u8>),
    Mem(MemData),
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (op, value) = line.split_once(" = ").ok_or("Couldn't split line!")?;
        if op == "mask" {
            return Ok(Instruction::Mask(value.as_bytes().to_vec()));
        }
        let mem_address: u64 = op[4..op.len() - 1]
            .parse::<u64>()
            .map_err(|_| "Couldn't parse address!")?;
        let mem_value: u64 = value.parse::<u64>().map_err(|_| "Couldn't parse value")?;
        Ok(Instruction::Mem(MemData {
            address: mem_address,
            value: mem_value,
        }))
    }
}

fn get_masked_value(mask: &[u8], mem: u64) -> u64 {
    let mut new_value: u64 = 0;
    for (i, maskbit) in mask.iter().enumerate() {
        let new_bit: bool = match *maskbit {
            b'X' => mem.get_bit(35 - i),
            setbit => setbit == 49, // b'1' is 49
        };
        // The value of a bit is determined by how far it is from the end, hence 35 - i
        new_value.set_bit(35 - i, new_bit);
    }
    new_value
}

pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            line.parse::<Instruction>()
                .unwrap_or_else(|_| panic!("Failed to parse line {}", line))
        })
        .collect()
}

pub fn part_one(data: &[Instruction]) -> u64 {
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".as_bytes().to_vec();
    let mut mem_hash: HashMap<u64, u64> = HashMap::new();
    for line in data {
        match line {
            Instruction::Mask(mask_value) => {
                mask = mask_value.clone();
            }
            Instruction::Mem(memdata) => {
                let new_value: u64 = get_masked_value(&mask, memdata.value);
                mem_hash.insert(memdata.address, new_value);
            }
        }
    }
    mem_hash.values().sum()
}

#[derive(Clone, Debug)]
struct FloatingAddress {
    curval: u32,
    max_iter: u32,
    base_pattern: u64,
    bit_locations: Vec<u8>,
}

impl<'a> FloatingAddress {
    fn new(mask: &[u8], address: u64) -> Self {
        let num_bits: u32 = mask
            .iter()
            .fold(0, |acc, x| if *x == b'X' { acc + 1 } else { acc });
        let max_iter: u32 = 2_u32.pow(num_bits) - 1;
        let bit_locations: Vec<u8> = mask
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if *x == b'X' { Some(i as u8) } else { None })
            .collect();
        let mut base_pattern: u64 = 0;
        for (i, char) in mask.iter().enumerate() {
            match char {
                b'0' => {
                    base_pattern.set_bit(35 - i, address.get_bit(35 - i));
                }
                b'1' => {
                    base_pattern.set_bit(35 - i, true);
                }
                _ => (), // This is an "X" in the pattern and we'll be overwriting it during iteration
            }
        }
        FloatingAddress {
            curval: 0,
            max_iter,
            base_pattern,
            bit_locations,
        }
    }
}

impl Iterator for FloatingAddress {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curval > self.max_iter {
            return None;
        }
        let mut next_item: u64 = self.base_pattern;
        for (i, loc) in self.bit_locations.iter().enumerate() {
            next_item.set_bit(35 - *loc as usize, self.curval.get_bit(i));
        }
        self.curval += 1;
        Some(next_item)
    }
}

pub fn part_two(data: &[Instruction]) -> u64 {
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".as_bytes().to_vec();
    let mut mem_hash: HashMap<u64, u64> = HashMap::new();
    for line in data {
        match line {
            Instruction::Mask(mask_value) => {
                mask = mask_value.clone();
            }
            Instruction::Mem(memdata) => {
                for address in FloatingAddress::new(&mask, memdata.address) {
                    mem_hash.insert(address, memdata.value);
                }
            }
        }
    }
    mem_hash.values().sum()
}
