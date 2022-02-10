use std::io::Error;
use std::process::Command;

pub fn exec(prog: &str, args: Option<Vec<&str>>) -> Result<String, Error> {
    let mut cmd = Command::new(prog);
    if args.is_some() {
        cmd.args(args.unwrap());
    }
    let out = cmd.output()?;
    Ok(String::from_utf8(out.stdout).unwrap())
}

