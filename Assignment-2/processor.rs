use std::env;
use std::fmt;
use std::fs;
use std::io::stdin;

#[derive(Debug)]
struct State {
	pc: usize,
	accum: usize,
	mbox: [usize; 100],
	neg_flag: bool,
	reg: [usize; 6],
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PC: {}, accum: {}, neg_flag: {}, reg: {:?}, mbox: ", self.pc, self.accum, self.neg_flag, self.reg)?;
	for i in 0..10 {
		let l = i*10;
		for j in l..l+10 {
			write!(f, "{}:{}\t", j, self.mbox[j]);
		}
		writeln!(f, "");
	}
	Ok(())
    }
}

fn load(file_path: &String) -> State {
	let mut state = State {
		pc: 0,
		accum: 0,
		mbox: [0; 100],
		neg_flag: false,
		reg: [0; 6],
	};
	let content = fs::read_to_string(file_path).expect("Unable to read the file");
	let data = content.lines();
	let mut i = 0;
	for s in data {
		let instruction = s.parse::<usize>().unwrap();
		state.mbox[i] = instruction;
		i += 1;
	}
    return state;
}

/* Returns if the program has finished */
fn run(state: &mut State) -> bool {
	let mut instruction = state.mbox[state.pc];
	while instruction != 0 {
		let opcode = instruction / 100;
		let n: usize = opcode % 10;
		let xx: usize = instruction % 100;
		let mut update_pc: bool = true;
		if opcode == 19 {
			state.accum += state.mbox[xx];
			state.neg_flag = false;
		}
		else if opcode >= 10 && opcode <= 15 && xx == 0 {
			for i in 0..(n + 1) {
				state.accum += state.reg[i];
			}
			state.neg_flag = false;
		}
		else if opcode == 29 {
			if state.mbox[xx] >= state.accum {
				state.accum = state.mbox[xx] - state.accum;
			}
			else {
				state.neg_flag = true;
				state.accum = 0;
			}
		}
		else if opcode >= 20 && opcode <= 25 && xx == 0 {
			let mut sum = 0;
			for i in 0..(n + 1) {
				sum += state.reg[i];
			}
			if state.accum >= sum {
				state.accum -= sum;
			}
			else{
				state.neg_flag = true;
				state.accum = 0;
			}
		}
		else if opcode == 30 {
			state.mbox[xx] = state.accum;
		}
		else if opcode == 59 {
			state.accum = state.mbox[xx];
		}
		else if opcode >= 50 && opcode <= 55 {
			for i in 0..(n + 1) {
				state.reg[i] = state.mbox[xx + i];
			}
		}
		else if opcode == 60 {
			state.pc = xx;
			update_pc = false;
		}
		else if opcode == 70 {
			if state.accum == 0 {
				state.pc = xx;
				update_pc = false;
			}
		}
		else if opcode == 71 {
			if !state.neg_flag {
				state.pc = xx;
				update_pc = false;
			}
		}
		else if instruction == 9001 {
			let mut input = String::new();
			println!("Enter a number: ");
    		stdin().read_line(&mut input).expect("failed to readline");
    		let val: usize = input.trim().parse().expect("invalid input");
    		state.accum = val;
		}
		else if instruction == 9002 {
			println!("Value in accumulator: {}", state.accum);
		}
		else if instruction == 9003 {
			println!("{}", state);
		}
		if update_pc {
			state.pc += 1;
		}
		instruction = state.mbox[state.pc];
	}
	true
}
 
fn main() -> Result<(), String> {
	let args: Vec<String> = env::args().collect();
	let file_path = args.get(1).ok_or("Required file path")?;

	// Load the program
	let mut state = load(file_path);
		
	// Run the program
	run(&mut state);
	
	Ok(())
}