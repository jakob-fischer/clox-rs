use std::sync::Arc;

#[repr(u8)]
enum OpCode {
    Constant,
    Return,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
} 

fn as_opcode(bytecode : u8) -> OpCode {
    unsafe {std::mem::transmute::<u8, OpCode>(bytecode)}    
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

    fn append_instruction(&mut self, val : OpCode, line : usize) {
        self.instructions.push(val as u8);
        self.lines.push(line);
    }

    fn append_parameter(&mut self, val : u8, line : usize) {
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

        let instruction =  as_opcode(*(self.instructions.get(offset).unwrap()));

        match instruction {
            OpCode::Return => {
                Self::simple_instruction("OP_RETURN", offset)
            },
            OpCode::Constant => {
                self.constant_instruction("OP_CONSTANT", offset)
            },
            OpCode::Negate => {
                Self::simple_instruction("OP_NEGATE", offset)
            },
            OpCode::Add => {
                Self::simple_instruction("OP_ADD", offset)
            },
            OpCode::Subtract => {
                Self::simple_instruction("OP_SUB", offset)
            },
            OpCode::Multiply => {
                Self::simple_instruction("OP_MUL", offset)
            },
            OpCode::Divide => {
                Self::simple_instruction("OP_DIV", offset)
            },
        }
    }
}

enum InterpretResult{
    Ok,
    CompileError,
    RuntimeError,
}

struct VM<const DebugTrace : bool> {
    chunk : Option<Arc<Chunk>>,
    ip : usize,
    stack : Vec<Value>,
}

impl<const DebugTrace : bool> VM<DebugTrace> {
    fn create() -> Self {
        VM{chunk: Option::None, ip : 0, stack: vec![]}
    }

    fn interpret(&mut self, chunk : Arc<Chunk>) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    fn push(&mut self, v : Value) {
        self.stack.push(v);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn run (&mut self) -> InterpretResult {
        let chunk_ref = self.chunk.as_ref().unwrap().as_ref();

        loop {
            if DebugTrace {
                println!("stack: {:?}", self.stack);
                chunk_ref.disassemble_instruction(self.ip);
            }

            let instruction = as_opcode(*chunk_ref.instructions.get(self.ip).unwrap());
            self.ip += 1;

            match instruction {
                OpCode::Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::Ok
                },
                OpCode::Constant => {
                    let constant_id = *chunk_ref.instructions.get(self.ip).unwrap(); 
                    self.ip += 1;
                    let constant_value : Value = *chunk_ref.constants.get(constant_id as usize).unwrap();
                    self.stack.push(constant_value);
                },
                OpCode::Negate => {
                    let v = self.stack.pop().unwrap();
                    self.stack.push(-v);
                },
                OpCode::Add => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    self.stack.push(lhs+rhs);
                },
                OpCode::Subtract => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    self.stack.push(lhs-rhs);
                },
                OpCode::Multiply => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    self.stack.push(lhs*rhs);
                },
                OpCode::Divide => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    self.stack.push(lhs/rhs);
                },
            }
        }
    }

}

fn main() {
    let mut chunk = Chunk::new();
    let mut vm = VM::<true>::create();

    let c1 = chunk.append_constant(1.2);
    let c2 = chunk.append_constant(3.4);
    let c3 = chunk.append_constant(5.6);

    chunk.append_instruction(OpCode::Constant, 123);
    chunk.append_parameter(c1, 123);
    chunk.append_instruction(OpCode::Constant, 123);
    chunk.append_parameter(c2, 123);
  
    chunk.append_instruction(OpCode::Add, 123);
  
    chunk.append_instruction(OpCode::Constant, 123);
    chunk.append_parameter(c3, 123);
  
    chunk.append_instruction(OpCode::Divide, 123);

    chunk.append_instruction(OpCode::Negate, 123);
    chunk.append_instruction(OpCode::Return, 123);

    let chunk = Arc::new(chunk);
    vm.interpret(chunk.clone());

    chunk.disassemble("test chunk");
}
