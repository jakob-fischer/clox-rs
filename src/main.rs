#[repr(u8)]
enum OpCode {
    Return = 1,
    Constant = 0,
}

type Value = f64;

struct Chunk {
    instructions : Vec<u8>,
    lines : Vec<usize>,
    constants : Vec<Value>,
}

impl Chunk {
    fn new() -> Self {
        Chunk{instructions : vec![], lines : vec![], constants : vec![]}
    }

    fn append_instruction(&mut self, val : u8, line : usize) {
        self.instructions.push(val);
        self.lines.push(line);
    }

    fn append_constant(&mut self, val : Value) -> u8 {
        self.constants.push(val);
        (self.constants.len()-1) as u8
    }

    fn disassemble(&self, name : &str) {
        println!("== {name} ==");

        let mut offset : usize = 0;

        while offset < self.instructions.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn simple_instruction(name : &str, offset : usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn print_value(value : Value) {
        print!("{value}");
    }

    fn constant_instruction(&self, name : &str, offset : usize) -> usize {
        let constant = *self.instructions.get(offset+1).unwrap();
        let value = *self.constants.get(constant as usize).unwrap();
        print!("{} {:04} '", name, constant);
        Self::print_value(value);
        print!("'\n");
        offset + 2
    }

    fn disassemble_instruction(&self, offset : usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 &&
            self.lines.get(offset).unwrap() == self.lines.get(offset).unwrap()  {
          print!("   | ");
        } else {
          print!("{:04} ", self.lines.get(offset).unwrap());
        }

        let instruction = unsafe {
            std::mem::transmute::<u8, OpCode>(*(self.instructions.get(offset).unwrap()))
        };

        match instruction {
            OpCode::Return => {
                Self::simple_instruction("OP_RETURN", offset)
            },
            OpCode::Constant => {
                self.constant_instruction("OP_CONSTANT", offset)
            }
        }
    }
}

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.append_constant(1.2);
    chunk.append_instruction(OpCode::Constant as u8, 123);
    chunk.append_instruction(constant, 123);
    chunk.append_instruction(OpCode::Return as u8, 123);

    chunk.disassemble("test chunk");
}
