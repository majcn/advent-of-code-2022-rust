mod interpreter {
    enum Command {
        NOOP,
        ADDX(i32),
    }

    // TODO; malo poglej tele tipe. malo so smesni
    // kako se drgac na simpl handla negativne stevilke pri -1
    pub struct Interpreter<'a> {
        program: Vec<&'a str>,
        program_index: usize,
        current_program_progress: u8,
        current_command: Command,
        pub register_x: i32,
        pub cycle: usize,
        pub halt: bool,
    }

    impl<'a> Interpreter<'a> {
        pub fn new(program: Vec<&'a str>) -> Self {
            let mut interpreter = Interpreter {
                program,
                program_index: 0,
                current_program_progress: 0,
                current_command: Command::NOOP,
                register_x: 1,
                cycle: 0,
                halt: false,
            };

            interpreter.init_next_command();

            interpreter
        }

        fn init_next_command(&mut self) {
            let command = self.program[self.program_index];
            let (command_name, command_value) =
                command.split_once(" ").unwrap_or_else(|| (command, ""));

            match command_name {
                "noop" => {
                    self.current_program_progress = 1;
                    self.current_command = Command::NOOP;
                }
                "addx" => {
                    self.current_program_progress = 2;
                    self.current_command = Command::ADDX(command_value.parse().unwrap());
                }
                _ => {}
            }

            self.program_index += 1;
        }

        fn execute_command(&mut self) {
            self.register_x += match self.current_command {
                Command::NOOP => 0,
                Command::ADDX(v) => v,
            }
        }

        pub fn exec_single_cycle(&mut self) {
            if self.halt {
                return;
            }

            self.cycle += 1;

            if self.current_program_progress == 1 {
                self.execute_command();

                if self.program_index == self.program.len() {
                    self.halt = true;
                    return;
                }

                self.init_next_command();
            } else {
                self.current_program_progress -= 1;
            }
        }
    }
}

fn parse_data(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let mut result = 0;

    let mut p = interpreter::Interpreter::new(data);
    while !p.halt {
        result += match p.cycle + 1 {
            20 | 60 | 100 | 140 | 180 | 220 => (p.cycle as u32 + 1) * (p.register_x as u32),
            _ => 0,
        };

        p.exec_single_cycle();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let data = parse_data(input);

    let mut display = [[false; 40]; 6];

    let mut p = interpreter::Interpreter::new(data);
    while !p.halt {
        let x = p.cycle % 40;
        let y = p.cycle / 40;

        display[y][x] = match p.register_x - x as i32 {
            -1 | 0 | 1 => true,
            _ => false,
        };

        p.exec_single_cycle();
    }

    let mut result = String::with_capacity(display.len() * (display[0].len() + 1));
    for line in display {
        result.extend(line.into_iter().map(|x| if x { '#' } else { ' ' }));
        result.push('\n');
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from(
                r#"
##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     
"#
                .trim_start()
            ))
        );
    }
}
