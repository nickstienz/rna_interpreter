#[derive(Debug)]
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
    let rna = String::from("AUG GGA CUA CAA CUA GGG AAG CAG CUA GAU CUA CAA AAA GCA AUA UGA");
    let rna = rna.replace(" ", "");

    // Turn it into Codons
    let mut codons = vec![];
    let chunks = rna.as_str().chars().collect::<Vec<_>>();
    for chunk in chunks.chunks(3) {
        let chunk: String = chunk.iter().collect();
        codons.push(chunk);
    }

    // Turn Codons into Instructions
    let codon_max_index = codons.len() - 1;
    let mut codon_index = 0;
    let mut instr_vec = vec![];

    while codon_index < codon_max_index {
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
            "ACC" | "AAU" => {
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
            "UAA" | "UAG" | "UGA" => {
                instr_vec.push(Instructions::Stop);
                codon_index += 1;
            }
            // NOT FOUND
            _ => panic!("Could not find symbol: {}", codon),
        }
    }

    println!("{:?}", instr_vec);
}
