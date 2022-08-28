use std::path::PathBuf;

#[derive(Debug)]
pub enum Instruction {
    /// Copies something from one location to another.
    COPY(PathBuf, PathBuf),
    /// Executes an external command.
    CMD(String, Vec<String>),
    /// Prints a message to the console.
    PRINT(String),
    /// "Owns" a folder or file, allowing it to be removed when the script is reversed.
    OWN(PathBuf),
    /// Attaches another "rope" to this one.
    ATTACH(String),
    /// Runs a specified command then checks it's exit code, if it fails, the script will exit.
    CHECK(String, Vec<String>),
    /// Runs a specified command then checks it's exit code, if it succeeds, the script will exit.
    CHECKERR(String, Vec<String>),
    /// Searchs the path for a file, if it is unfound, the rope will be cut (stop executing).
    REQUIRE(String),
    /// Searchs the path for a file, if it is found, the rope will be cut (stop executing).
    REQUIRENOT(String),
}

impl Instruction {
    pub fn parse(instruction: &str, args: Vec<String>) -> Self {
        match instruction.trim().to_uppercase().as_str() {
            "COPY" => Instruction::COPY(
                args.get(0)
                    .expect("Expected argument")
                    .parse()
                    .expect("Expected Path"),
                args.get(1)
                    .expect("Expected argument")
                    .parse()
                    .expect("Expected Path"),
            ),
            "CMD" => Instruction::CMD(
                args.get(0).expect("Expected command").to_owned(),
                args.split_first().unwrap().1.to_vec(),
            ),
            "OWN" => Instruction::OWN(
                args.get(0)
                    .expect("Expected argument")
                    .parse()
                    .expect("Expected Path"),
            ),
            "PRINT" => Instruction::PRINT(args.get(0).expect("Need something to print").to_owned()),
            "ATTACH" => Instruction::ATTACH(args.get(0).expect("Expected rope name").to_owned()),
            "CHECK" => Instruction::CHECK(
                args.get(0).expect("Expected command").to_owned(),
                args.split_first().unwrap().1.to_vec(),
            ),
            "CHECKERR" => Instruction::CHECKERR(
                args.get(0).expect("Expected command").to_owned(),
                args.split_first().unwrap().1.to_vec(),
            ),
            "REQUIRE" => {
                Instruction::REQUIRE(args.get(0).expect("Expected file to find").to_owned())
            }
            "REQUIRENOT" => {
                Instruction::REQUIRENOT(args.get(0).expect("Expected file to find").to_owned())
            }
            _ => panic!("Invalid instruction {}", instruction),
        }
    }
}
