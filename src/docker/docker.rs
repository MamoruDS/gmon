use crate::docker::container;
use crate::utils::exec as _exec;
use std::io::{Error, ErrorKind::NotFound};
use std::process::{Command, Stdio};

fn exec(args: Vec<&str>) -> Result<String, Error> {
    _exec("docker", Some(args))
}

pub type Id = String;
pub type ShortId = String;

#[derive(Debug)]
pub struct Docker {
    // containerd_shim_pid: u32
}

#[allow(dead_code)]
impl Docker {
    pub fn new() -> Self {
        Docker {}
    }

    fn load_ctr_from_inspect(&self, out: &str) -> container::Container {
        let inf: Vec<&str> = out.split(" ").collect();
        container::Container {
            id: String::from(inf[0]),
            state: container::State {
                pid: inf[1].parse::<u32>().unwrap(),
            },
            name: {
                let mut name = String::from(inf[2]);
                if name.starts_with('/') {
                    name.remove(0);
                }
                name
            },
        }
    }

    pub fn ps(&self) -> Vec<ShortId> {
        let mut out = exec(vec!["ps", "-q"]).unwrap();
        if out.ends_with("\n") {
            out.pop();
        }
        let mut ids: Vec<String> = vec![];
        for id in out.split("\n").collect::<Vec<&str>>() {
            ids.push(String::from(id))
        }
        ids
    }

    pub fn inspect(&self, id: &str) -> container::Container {
        // TODO:
        let mut out = exec(vec![
            "inspect",
            id,
            "--format",
            r#"{{.Id}} {{.State.Pid}} {{.Name}}"#,
        ])
        .unwrap();
        if out.ends_with("\n") {
            out.pop();
        }
        self.load_ctr_from_inspect(&out)
    }

    pub fn inspect_all(&self) -> Vec<container::Container> {
        let mut cmd = match Command::new("docker")
            .args(["ps", "-q"])
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(cmd) => cmd,
            Err(ref e) if e.kind() == NotFound => {
                return vec![];
            }
            Err(e) => {
                println!("{:?}", e);
                return vec![];
            }
        };
        let out_q = cmd.stdout.take().unwrap();
        let mut cmd = Command::new("xargs");
        let cmd = cmd.args([
            "docker",
            "inspect",
            "--format",
            r#"'{{.ID}} {{.State.Pid}} {{.Name}}"#,
        ]);
        let cmd_out = cmd.stdin(out_q).output().unwrap();
        let mut out = String::from_utf8(cmd_out.stdout).unwrap();
        if out.ends_with("\n") {
            out.pop();
        }
        let out_arr: Vec<&str> = out.split('\n').collect();
        let mut ctr_arr: Vec<container::Container> = vec![];
        for out in out_arr {
            if out.len() == 0 {
                continue;
            }
            ctr_arr.push(self.load_ctr_from_inspect(out))
        }
        ctr_arr
    }
}
