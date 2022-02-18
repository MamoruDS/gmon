use crate::docker::{container::Container, Docker};
use crate::nv_gpu;
use std::collections::HashMap;
use sysinfo::{Pid, PidExt, Process, ProcessExt, System, SystemExt};

pub struct GProcess<'a> {
    pub gpu_id: u8,
    pub pid: u32,
    pub uid: u32,
    pub gid: u32,
    pub ctr_pid: Option<u32>,
    pub gpu_info: &'a nv_gpu::ProcessInfo,
}

impl<'a> GProcess<'a> {
    pub fn container(&self, pd: &'a ProcessData) -> Option<&'a Container> {
        if pd.ctr_map.is_some() && self.ctr_pid.is_some() {
            let map = &pd.ctr_map.as_ref().unwrap();
            map.get(&self.ctr_pid.unwrap())
        } else {
            None
        }
    }
    pub fn process_info(&self, pd: &'a ProcessData) -> &'a Process {
        let pid = Pid::from(i32::try_from(self.pid).unwrap());
        pd.sys.process(pid).unwrap()
    }
    pub fn gpu_usage(&self) -> &'a nv_gpu::ProcessInfo {
        self.gpu_info
    }
}

pub struct ProcessData<'a> {
    sys: System,
    pub gpu_procs: Vec<GProcess<'a>>,
    ctr_map: Option<HashMap<u32, Container>>,
    pub gpu_info: &'a nv_gpu::GPUInfo,
}

fn get_process(
    sys: &System,
    pid: u32,
    ctr_map: &Option<HashMap<u32, Container>>,
) -> (u32, u32, Option<u32>) {
    let mut uid = 0;
    let mut gid = 0;
    let mut ctr_pid = None;
    //
    let mut cur_pid = Pid::from(i32::try_from(pid).unwrap());
    let proc = sys.process(cur_pid);
    //
    let mut cur_p = proc;
    loop {
        match cur_p {
            Some(p) => {
                // TODO:
                uid = if p.uid != 0 { p.uid } else { uid };
                gid = if p.uid != 0 { p.gid } else { gid };
                if ctr_map.is_some() && ctr_pid.is_none() {
                    let cur_pid_u32 = cur_pid.as_u32();
                    if ctr_map.as_ref().unwrap().contains_key(&cur_pid_u32) {
                        ctr_pid = Some(cur_pid_u32);
                        break;
                    }
                }
                match p.parent() {
                    Some(ppid) => {
                        cur_pid = ppid;
                        cur_p = sys.process(cur_pid);
                    }
                    None => break,
                }
            }
            None => break,
        }
    }
    (uid, gid, ctr_pid)
}
impl<'a> ProcessData<'a> {
    pub fn load(gpu_info: &'a nv_gpu::GPUInfo, docker_support: bool) -> Self {
        let mut pd = ProcessData {
            sys: System::new_all(),
            gpu_procs: vec![],
            ctr_map: if docker_support {
                let docker = Docker::new();
                let ctr_vec = docker.inspect_all();
                let mut ctr_map: HashMap<u32, Container> = HashMap::new();
                for ctr in ctr_vec {
                    ctr_map.insert(ctr.state.pid, ctr);
                }
                Some(ctr_map)
            } else {
                None
            },
            gpu_info,
        };
        for (i, gpu) in pd.gpu_info.gpus.iter().enumerate() {
            match &gpu.processes.items {
                Some(items) => {
                    for proc in items.iter() {
                        let pid = proc.pid;
                        let (uid, gid, ctr_pid) = get_process(&pd.sys, pid, &pd.ctr_map);
                        pd.gpu_procs.push(GProcess {
                            gpu_id: u8::try_from(i).unwrap(),
                            pid,
                            uid,
                            gid,
                            ctr_pid,
                            gpu_info: proc,
                        })
                    }
                }
                _ => (),
            }
        }
        pd
    }

    pub fn items(&self) -> &Vec<GProcess> {
        &self.gpu_procs
    }
}
