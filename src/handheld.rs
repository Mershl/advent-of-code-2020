use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct Instruction<'a> {
    pub(crate) op_code: &'a str,
    pub(crate) args: Vec<&'a str>
}

#[derive(Debug)]
pub struct Machine {
    ic: usize,
    acc: i64,
    executed_ics: HashSet<usize>,
    endless_detected: bool
}

impl Machine {
    pub fn new() -> Self {
        Self {
            ic: 0,
            acc: 0,
            executed_ics: HashSet::new(),
            endless_detected: false
        }
    }

    pub fn exec_with_instruction_list(&mut self, ic_list: &[Instruction]) {
        while !self.endless_detected && self.ic != ic_list.len() {
            self.exec_instruction(ic_list.get(self.ic).unwrap());
        }
    }

    fn exec_instruction(&mut self, instruction: &Instruction) {
        if self.executed_ics.contains(&self.ic) {
            self.endless_detected = true;
            return;
        }

        // jmp instructions shall not increment ic
        let mut inc_ic = true;

        // mark current ic as executed
        self.executed_ics.insert(self.ic);

        // execute instruction
        match instruction.op_code {
            "nop" => {
                // do nothing
            },
            "acc" => {
                let arg0: i64 = instruction.args.get(0).unwrap().parse().unwrap();
                self.acc += arg0;
            },
            "jmp" => {
                let arg0: i64 = instruction.args.get(0).unwrap().parse().unwrap();
                self.ic = (self.ic as i64 + arg0).try_into().unwrap();
                inc_ic = false;
            },
            _ => panic!("Unsupported op_code: {}", instruction.op_code)
        }

        if inc_ic {
            self.ic += 1;
        }
    }

    pub fn get_ic(&self) -> usize {
        self.ic
    }

    pub fn get_acc(&self) -> i64 {
        self.acc
    }

    pub fn endless_loop_detected(&self) -> bool {
        self.endless_detected
    }
}
