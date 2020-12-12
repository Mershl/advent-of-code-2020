use crate::handheld::*;

pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.split_whitespace())
         .map(|mut splits| Instruction {
             op_code: splits.next().unwrap(),
             args: splits.collect(),
         }).collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i64 {
    let instructions = input_generator(input);
    let mut machine = Machine::new();
    machine.exec_with_instruction_list(&*instructions);
    machine.get_acc()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    let instructions = input_generator(input);

    let bugfixed_machine = instructions.iter().enumerate()
                                       .filter(|(_, inst)| inst.op_code == "nop" || inst.op_code == "jmp")
                                       .map(|(idx, inst)| {
                                           let mut instructions_candidate = instructions.to_vec();
                                           instructions_candidate[idx].op_code = match inst.op_code {
                                               "nop" => "jmp",
                                               "jmp" => "nop",
                                               _ => unreachable!()
                                           };
                                           let mut machine = Machine::new();
                                           machine.exec_with_instruction_list(&*instructions_candidate);
                                           machine
                                       }).find(|machine| !machine.endless_loop_detected());
    if let Some(machine) = bugfixed_machine {
        return machine.get_acc();
    }
    panic!("No machine terminated without endless loop.")
}
