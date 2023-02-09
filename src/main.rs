#[derive(Debug, PartialEq)]
enum Instructions {
    Output(u32),
    Decrement(u32),
    JumpNotZero(u32),
    Addition(u32, u32),
    Subtraction(u32, u32),
    JumpIfZero(u32),
    Break,
    Move(u32, u32),
    Increment(u32),
    Start,
    Stop,
}

fn codons_to_number(codons: &Vec<String>, index: usize) -> (u32, usize) {
    let mut number = String::from("0");
    let mut current_index = index;
    loop {
        if current_index >= codons.len() {
            break;
        }
        number.push(match codons[current_index].as_str() {
            "AUA" | "AUC" | "AUU" => '0',
            "CUA" | "CUC" | "CUG" | "CUU" | "UUA" | "UUG" => '1',
            "AAA" | "AAG" => '2',
            "UUU" | "UUC" => '3',
            "CCA" | "CCC" | "CCG" | "CCU" => '4',
            "AGC" | "AGU" | "UCA" | "UCC" | "UCG" | "UCU" => '5',
            "ACC" | "ACU" | "UGC" | "UGU" => '6',
            "UGG" => '7',
            "UAU" | "UAC" => '8',
            "GUA" | "GUC" | "GUG" | "GUU" => '9',
            _ => {
                break;
            }
        });
        current_index += 1;
    }

    (number.parse::<u32>().unwrap(), current_index)
}

fn main() {
    println!("RNA Interpreter by Nicholas Stienz");

    // Grab the String from the file
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        panic!("Usage: rna_interpreter <rna>");
    }
    let rna = args[1].clone();
    let rna = rna.replace(" ", "");
    println!("RNA: {}", rna);

    // Turn it into Codons
    let mut codons = vec![];
    let chunks = rna.as_str().chars().collect::<Vec<_>>();
    for chunk in chunks.chunks(3) {
        let chunk: String = chunk.iter().collect();
        codons.push(chunk);
    }
    println!("Codons: {:?}", codons);

    // Turn Codons into Instructions
    let codon_max_index = codons.len() - 1;
    let mut codon_index = 0;
    let mut instr_vec = vec![];

    while codon_index <= codon_max_index {
        let codon = &codons[codon_index];

        match codon.as_str() {
            // OUT(u32)
            "GCA" | "GCC" | "GCG" | "GCU" => {
                let (number, index) = codons_to_number(&codons, codon_index + 1);
                codon_index = index;
                instr_vec.push(Instructions::Output(number));
            }
            // DEC(u32)
            "AGA" | "AGG" | "CGA" | "CGC" | "CGG" | "CGU" => {
                let (number, index) = codons_to_number(&codons, codon_index + 1);
                codon_index = index;
                instr_vec.push(Instructions::Decrement(number));
            }
            // JNZ(u32)
            "AAC" | "AAU" => {
                let (number, index) = codons_to_number(&codons, codon_index + 1);
                codon_index = index;
                instr_vec.push(Instructions::JumpNotZero(number));
            }
            // ADD(u32, u32)
            "GAC" | "GAU" => {
                let (number1, index1) = codons_to_number(&codons, codon_index + 1);
                codon_index = index1;
                let (number2, index2) = codons_to_number(&codons, codon_index + 1);
                codon_index = index2;
                instr_vec.push(Instructions::Addition(number1, number2));
            }
            // SUB(u32, u32)
            "UGC" | "UGU" => {
                let (number1, index1) = codons_to_number(&codons, codon_index + 1);
                codon_index = index1;
                let (number2, index2) = codons_to_number(&codons, codon_index + 1);
                codon_index = index2;
                instr_vec.push(Instructions::Subtraction(number1, number2));
            }
            // JZ(u32)
            "GAA" | "GAG" => {
                let (number, index) = codons_to_number(&codons, codon_index + 1);
                codon_index = index;
                instr_vec.push(Instructions::JumpIfZero(number));
            }
            // BREAK
            "CAA" | "CAG" => {
                instr_vec.push(Instructions::Break);
                codon_index += 1;
            }
            // MOV(u32, u32)
            "GGA" | "GGC" | "GGG" | "GGU" => {
                let (number1, index1) = codons_to_number(&codons, codon_index + 1);
                codon_index = index1;
                let (number2, index2) = codons_to_number(&codons, codon_index + 1);
                codon_index = index2;
                instr_vec.push(Instructions::Move(number1, number2));
            }
            // INC(u32)
            "CAC" | "CAU" => {
                let (number, index) = codons_to_number(&codons, codon_index + 1);
                codon_index = index;
                instr_vec.push(Instructions::Increment(number));
            }
            // START
            "AUG" => {
                instr_vec.push(Instructions::Start);
                codon_index += 1;
            }
            // STOP
            "UAA" | "UAG" | "UGA" => {
                instr_vec.push(Instructions::Stop);
                codon_index += 1;
            }
            // NOT FOUND
            _ => panic!("Could not find symbol: {}", codon),
        }
    }

    println!("Instructions: {:?}\n", instr_vec);

    // Instruction checks
    if instr_vec.len() < 2 {
        panic!("Program must have more then 1 instruction!");
    }

    if instr_vec[0] != Instructions::Start {
        panic!("Program must start with the start amino acid!");
    }

    if instr_vec[instr_vec.len() - 1] != Instructions::Stop {
        panic!("Program must end with a stop amino acid!");
    }

    // Execution
    let mut i_pointer = 0;
    let mut array: [i32; 9] = [0; 9];
    let instructions = instr_vec;

    loop {
        match instructions[i_pointer] {
            Instructions::Start => i_pointer += 1,
            Instructions::Break => i_pointer += 1,
            Instructions::Stop => break,
            Instructions::Output(x) => {
                println!("Output: {}", array[x as usize]);
                i_pointer += 1;
            }
            Instructions::Addition(x, y) => {
                array[0] = array[x as usize] + array[y as usize];
                i_pointer += 1;
            }
            Instructions::Subtraction(x, y) => {
                array[0] = array[x as usize] - array[y as usize];
                i_pointer += 1;
            }
            Instructions::Increment(x) => {
                array[x as usize] += 1;
                i_pointer += 1;
            }
            Instructions::Decrement(x) => {
                array[x as usize] -= 1;
                i_pointer += 1;
            }
            Instructions::JumpIfZero(x) => {
                if array[1] == 0 {
                    i_pointer = x as usize;
                } else {
                    i_pointer += 1;
                }
            }
            Instructions::JumpNotZero(x) => {
                if array[1] != 0 {
                    i_pointer = x as usize;
                } else {
                    i_pointer += 1;
                }
            }
            Instructions::Move(x, y) => {
                array[x as usize] = y as i32;
                i_pointer += 1;
            }
        }
    }
}
