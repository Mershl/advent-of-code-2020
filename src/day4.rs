use std::collections::HashMap;

#[derive(Debug)]
pub struct Passport {
    fields: HashMap<String, String>
}

impl Passport {
    const ESSENTIAL_FIELDS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn essentials_present(&self) -> bool {
        Passport::ESSENTIAL_FIELDS.iter().all(|&e_field| self.fields.contains_key(e_field))
    }

    fn is_valid(&self) -> bool {
        if !self.essentials_present() {
            return false;
        }
        self.check_byr() && self.check_iyr() && self.check_eyr() && self.check_hgt() && self.check_hcl() && self.check_ecl() && self.check_pid()
    }

    fn check_byr(&self) -> bool {
        let byr_int: usize = self.fields.get("byr").unwrap().parse().unwrap();
        byr_int >= 1920 && byr_int <= 2002
    }

    fn check_iyr(&self) -> bool {
        let iyr_int: usize = self.fields.get("iyr").unwrap().parse().unwrap();
        iyr_int >= 2010 && iyr_int <= 2020
    }

    fn check_eyr(&self) -> bool {
        let eyr_int: usize = self.fields.get("eyr").unwrap().parse().unwrap();
        eyr_int >= 2020 && eyr_int <= 2030
    }

    fn check_hgt(&self) -> bool {
        let hgt: &String = self.fields.get("hgt").unwrap();
        let height: usize = hgt[0..(hgt.len() - 2)].parse().unwrap_or(0);
        let unit = &hgt[(hgt.len() - 2)..];
        match unit {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => false
        }
    }

    fn check_hcl(&self) -> bool {
        let hcl: &String = self.fields.get("hcl").unwrap();
        hcl.starts_with("#") && hcl.len() == 7 && hcl.chars().skip(1).all(|c| c.is_ascii_hexdigit())
    }

    fn check_ecl(&self) -> bool {
        let ecl: &String = self.fields.get("ecl").unwrap();
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&**ecl)
    }

    fn check_pid(&self) -> bool {
        let pid: &String = self.fields.get("pid").unwrap();
        pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))
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
    // TIL: str.lines() cuts empty lines at end of file.
    passports.push(Passport {
        fields: next_passport_fields
    });
    passports
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|pp| pp.essentials_present()).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
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
