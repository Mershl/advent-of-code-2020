use std::collections::HashMap;

#[derive(Debug)]
pub struct Passport {
    fields: HashMap<String, String>
}

impl Passport {
    const ESSENTIAL_FIELDS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn is_valid(&self) -> bool {
        // Passport::ESSENTIAL_FIELDS.iter().all(|&e_field| self.fields.contains_key(e_field))
        for &e_field in Passport::ESSENTIAL_FIELDS.iter() {
            if !self.fields.contains_key(e_field) {
                println!("{:?}\nMissing: {}", self, e_field);
                return false;
            }
        }
        true
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut next_passport_fields: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            passports.push(Passport {
                fields: next_passport_fields
            });
            next_passport_fields = HashMap::new();
        } else {
            let splits: Vec<&str> = line.split(' ').collect();
            for split in splits {
                let mut key_value = split.split(':');
                next_passport_fields.insert(key_value.next().unwrap().to_string(), key_value.next().unwrap().to_string());
            }
        }
    }
    passports
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|pp| pp.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(part1(&*input_generator(input)), 2);
    }
}
