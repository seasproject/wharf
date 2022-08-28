use std::{
    env::set_current_dir,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

mod lang;
use colored::Colorize;
use compression::prelude::{Action, DecodeExt, EncodeExt, GZipDecoder, GZipEncoder};
use lang::Instruction::{self, *};

pub fn run(path: PathBuf) {
    let mut path = path;
    if path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .ends_with(".ship")
    {
        println!("{}: Extracting ship", "Info".blue());
        extract(&path);
        path = path.with_extension("").join("build.rope");
    }
    let parsed = parse(&path);
    if path.canonicalize().unwrap().parent().is_some() {
        let path = path
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .replace("\\\\?\\", "");
        set_current_dir(path).unwrap();
    }
    for instruction in parsed.iter() {
        match instruction {
            COPY(src, dst) => {
                if src.is_dir() {
                    copy_dir(src, dst);
                } else {
                    fs::copy(src, dst).expect("Failed to copy the file");
                }
            }
            CMD(cmd, args) => {
                let mut command = Command::new("cmd.exe");
                command.args(&["/C", cmd]);
                args.iter().for_each(|arg| {
                    if !arg.eq("--ignore") {
                        command.arg(arg);
                    }
                });
                command.spawn().expect("Failed to execute the command");
            }
            OWN(_) => {}
            PRINT(s) => {
                println!("{}", s);
            }
            ATTACH(rope) => {
                let mut rope = rope.clone();
                rope.push_str(".rope");
                let path = PathBuf::from(rope);
                run(path);
            }
            CHECK(cmd, args) => {
                let mut command = Command::new(cmd);
                args.iter().for_each(|arg| {
                    command.arg(arg);
                });
                command.stdin(Stdio::null()).stdout(Stdio::null());
                let status = command.status();
                if status.is_err() {
                    // Command has failed
                    return;
                }
                let status = status.unwrap();
                if !status.success() {
                    return;
                }
            }
            CHECKERR(cmd, args) => {
                let mut command = Command::new(cmd);
                args.iter().for_each(|arg| {
                    command.arg(arg);
                });
                command.stdin(Stdio::null()).stdout(Stdio::null());
                let status = command.status();
                if status.is_err() {
                } else {
                    let status = status.unwrap();
                    if status.success() {
                        return;
                    }
                }
            }
            REQUIRE(file) => {
                let status = Command::new("where")
                    .arg(file)
                    .stdout(Stdio::null())
                    .stdin(Stdio::null())
                    .status()
                    .expect("Failed to execute command");
                if !status.success() {
                    return;
                }
            }
            REQUIRENOT(file) => {
                let status = Command::new("where")
                    .arg(file)
                    .stdout(Stdio::null())
                    .stdin(Stdio::null())
                    .status()
                    .expect("Failed to execute command");
                if status.success() {
                    return;
                }
            }
        }
    }
}

pub fn reverse(path: PathBuf) {
    let mut path = path;
    if path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .ends_with(".ship")
    {
        println!("{}: Extracting ship", "Info".blue());
        extract(&path);
        path = path.with_extension("").join("build.rope");
    }
    let parsed = parse(&path);
    if path.canonicalize().unwrap().parent().is_some() {
        let path = path
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .replace("\\\\?\\", "");
        set_current_dir(path).unwrap();
    }
    parsed
        .iter()
        .rev()
        .for_each(|instruction| match instruction {
            COPY(src, dst) => {
                if src.is_dir() {
                    fs::remove_dir_all(dst).expect("Failed to remove the directory");
                } else {
                    fs::remove_file(dst).expect("Failed to remove the file");
                }
            }
            CMD(cmd, args) => {
                if !args.contains(&"--ignore".to_string()) {
                    println!("{}: External commands cannot be reversed, if you are the developer, we strongly recommend you use the provided commands that are reversable, or use the OWN command to tell wharf the outputs of the command ({} {})", "Warning".yellow(), cmd, args.join(" "))
                }
            }
            OWN(path) => {
                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove the directory");
                } else {
                    fs::remove_file(path).expect("Failed to remove the file");
                }
            }
            // Cannot reverse a print
            PRINT(_) => {}
            // Reverse the attached rope
            ATTACH(rope) => {
                let path = PathBuf::from(format!("{rope}.rope"));
                reverse(path);
            }
            // Cannot reverse a check
            CHECK(_, _) => {}
            CHECKERR(_, _) => {}
            REQUIRE(_) => {}
            REQUIRENOT(_) => {}
        })
}

pub fn package(path: PathBuf) {
    if !path.is_dir() {
        println!("{}: The path provided is not a directory", "Error".red());
        return;
    }

    // Bundle all the files into a tarball
    let mut data: Vec<u8> = Vec::new();
    let mut archive = tar::Builder::new(&mut data);
    // Iterate over the files in selected directory.
    // for entry in fs::read_dir(path).expect("Failed to read the specified directory") {
    //     let entry = entry.unwrap();
    //     let path = entry.path();
    //     if path.is_dir() {
    //         archive
    //             .append_dir_all(path.to_str().unwrap(), &path)
    //             .unwrap();
    //     } else {
    //         archive
    //             .append_file(path.to_str().unwrap(), &mut fs::File::open(&path).unwrap())
    //             .unwrap();
    //     }
    // }
    archive
        .append_dir_all(".", &path)
        .expect("Failed to wrap tarball");
    archive.finish().unwrap();
    let data = archive.into_inner().unwrap();
    let data = data
        .to_owned()
        .encode(&mut GZipEncoder::new(), Action::Finish)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let name = path.file_name().unwrap().to_str().unwrap();

    fs::write(
        &path
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .join(format!("{}.ship", &name)),
        data,
    )
    .expect("Failed to write the tarball");
}

fn parse(path: &PathBuf) -> Vec<Instruction> {
    let contents = fs::read_to_string(path).expect("Failed to read the file");
    let instructions_strs = contents.lines();
    instructions_strs
        .filter(|line| line.trim().len() > 0)
        .filter(|line| !line.starts_with("#"))
        .filter(|line| !line.starts_with("//"))
        .map(|line| lang::read_line(line))
        .collect()
}

fn extract(path: &PathBuf) {
    let name = path.file_stem();
    let tar_gz = fs::read(path).expect("Failed to read the tarball");
    let data = tar_gz
        .decode(&mut GZipDecoder::new())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut archive = tar::Archive::new(&data[..]);
    archive
        .unpack(
            &path
                .canonicalize()
                .unwrap()
                .parent()
                .unwrap()
                .join(name.unwrap()),
        )
        .unwrap();
}

// Helper function to copy directories
fn copy_dir(src: &PathBuf, dst: &PathBuf) {
    fs::create_dir_all(dst).expect("Failed to create the directory");
    for entry in fs::read_dir(src).expect("Failed to read the directory") {
        let entry = entry.expect("Failed to read the entry");
        let path = entry.path();
        let dst_path = dst.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_dir(&path, &dst_path);
        } else {
            fs::copy(&path, &dst_path).expect("Failed to copy the file");
        }
    }
}
