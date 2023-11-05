use std::env;
use std::io::Read;
use std::fs::File;
use std::process::exit;

// Lexical instructions. Text converted to code
#[derive(Clone)]
enum LexicalInst
{
    MovRight,
    MovLeft,
    Inc,
    Dec,
    Read,
    Write,
    IniLoop,
    EndLoop
}

// Instructions to be executed
enum Instruction
{
    MovRight,
    MovLeft,
    Inc,
    Dec,
    Read,
    Write,
    Loop(Vec<Instruction>)
}

// Length of the Tape. By default always 30k bytes
const TAPE_LEN: usize = 30000;

fn main() 
{
    let src_code = read_content();
    let lexical_instructions: Vec<LexicalInst> = lexicon_parser(src_code);
    let program = parse_instructions(lexical_instructions);

    let mut tape: Vec<u8> = vec![0; TAPE_LEN];
    let mut pointer: usize = 1000;
    execute(&program, &mut tape, &mut pointer);
    exit(0);
}

/*
    This function is used to read the contents of the file and 
    returns it as a single string of text
*/
fn read_content() -> String
{
    let args: Vec<_> = env::args().collect();

    if args.len() != 2
    {
        println!("Parameter error.\nUsage:\n  bfcompiler <filename.bf>");
    }

    let f_name = &args[1];

    let mut file: File = File::open(&f_name).expect("File not found");
    let mut src_code: String = String::new();
    let _ = file.read_to_string(&mut src_code);

    return src_code;
} 

/*
    This function parses the lexicon of the file.
    Meaning it transforms the text of the file to
    something usable by the program.

    Every character in the program corresponds to a
    single lexical instruction
*/
fn lexicon_parser(src_code: String) -> Vec<LexicalInst>
{
    let mut lexicon: Vec<LexicalInst> = Vec::new();

    for symbol in src_code.chars()
    {
        let var = match symbol
        {
            '>' => LexicalInst::MovRight,
            '<' => LexicalInst::MovLeft,
            '+' => LexicalInst::Inc,
            '-' => LexicalInst::Dec,
            '.' => LexicalInst::Write,
            ',' => LexicalInst::Read,
            '[' => LexicalInst::IniLoop,
            ']' => LexicalInst::EndLoop,
            _ => continue
        };
        lexicon.push(var);
    }
    return lexicon;
}

/*
    This function transforms the lexical instructions
    previously parsed to instructions we are actually going
    to run.

    We need to make this differenciation to be able to loop
*/
fn parse_instructions(lex_inst: Vec<LexicalInst>) -> Vec<Instruction>
{
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, l_inst) in lex_inst.iter().enumerate() {
        if loop_stack == 0 {
            let inst = match l_inst {
                LexicalInst::MovRight => Some(Instruction::MovRight),
                LexicalInst::MovLeft => Some(Instruction::MovLeft),
                LexicalInst::Inc => Some(Instruction::Inc),
                LexicalInst::Dec => Some(Instruction::Dec),
                LexicalInst::Write => Some(Instruction::Write),
                LexicalInst::Read => Some(Instruction::Read),
                LexicalInst::IniLoop => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                },
                LexicalInst::EndLoop => panic!("Loop ending at #{} has no beginning", i),
            };
            match inst {
                Some(inst) => program.push(inst),
                None => ()
            }
        } else {
            match l_inst {
                LexicalInst::IniLoop => {
                    loop_stack += 1;
                },
                LexicalInst::EndLoop => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(
                            Instruction::Loop(
                                parse_instructions(lex_inst[loop_start+1..i].to_vec())
                            )
                        );
                    }
                },
                _ => (),
            }
        }
    }
    if loop_stack != 0 {
        panic!("Loop that starts at #{} has no matching ending!", loop_start);
    }
    return program;
}

fn execute(program: &Vec<Instruction>, tape: &mut Vec<u8>, pointer: &mut usize) {
    for inst in program {
        match inst {
            Instruction::MovRight => *pointer += 1,
            Instruction::MovLeft => *pointer -= 1,
            Instruction::Inc => tape[*pointer] += 1,
            Instruction::Dec => tape[*pointer] -= 1,
            Instruction::Write => print!("{}", tape[*pointer] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("Read instruction failed");
                tape[*pointer] = input[0];
            }
            Instruction::Loop(looped_program) => {
                while tape[*pointer] != 0 {
                    execute(&looped_program, tape, pointer)
                }
            }
        }
    }
}