use std::{borrow::Borrow, collections::HashMap};
use wasm_bindgen::prelude::*;
mod utils;
use js_sys::Uint32Array;
use utils::set_panic_hook;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn update_reg_file(file: Uint32Array);
}

struct State {
    pc: i32,
    regs: [u32; 32],
    mem: [u8; 1024],
}

impl State {
    fn new() -> State {
        State {
            pc: 0,
            regs: [0; 32],
            mem: [0; 1024],
        }
    }

    pub fn set_reg(&mut self, reg: u32, val: u32) {
        self.regs[reg as usize] = val;
    }

    pub fn write_mem(&mut self, addr: u32, val: u8) {
        self.mem[addr as usize] = val;
    }

    pub fn read_mem(&self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }

    pub fn read_reg(&self, reg: u32) -> u32 {
        self.regs[reg as usize]
    }

    pub fn dump(&self) -> String {
        let mut s: String = String::new();
        s += format!("pc: {}\n", self.pc).as_str();
        let mut i = 0;
        for reg in self.regs {
            s += format!("\tr{} = 0x{:X}\n", i, reg).as_str();
            i += 1;
        }
        s
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, test-wasm!");
}

struct Line {
    label: Option<String>,
    instr: String,
    args: Vec<String>,
}

fn parse_line(line: String) -> Line {
    let mut it = line.split_whitespace();
    let first = it.next().unwrap();
    if first.chars().nth(0).unwrap() == '.' {
        // label
        return Line {
            label: Some(first.to_string()),
            instr: "".to_string(),
            args: vec![],
        };
    }

    match first {
        "add" => {
            let rd = it.next().unwrap();
            let rs = it.next().unwrap();
            let rt = it.next().unwrap();
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![rd.to_string(), rs.to_string(), rt.to_string()],
            }
            // add rd, rs, rt
            // rd = rs + rt
        }
        "sub" => {
            let rd = it.next().unwrap();
            let rs = it.next().unwrap();
            let rt = it.next().unwrap();
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![rd.to_string(), rs.to_string(), rt.to_string()],
            }
            // sub rd, rs, rt
            // rd = rs - rt
        }
        "lw" => {
            let rt = it.next().unwrap();
            let rs = it.next().unwrap();
            let imm = it.next().unwrap();
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![rt.to_string(), rs.to_string(), imm.to_string()],
            }
            // lw rt, imm(rs)
            // rt = mem[rs + imm]
        }
        "sw" => {
            let rt = it.next().unwrap();
            let rs = it.next().unwrap();
            let imm = it.next().unwrap();
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![rt.to_string(), rs.to_string(), imm.to_string()],
            }
            // sw rt, imm(rs)
            // mem[rs + imm] = rt
        }
        "beq" => {
            let rs = it.next().unwrap();
            let rt = it.next().unwrap();
            let label = it.next().unwrap();
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![rs.to_string(), rt.to_string(), label.to_string()],
            }
            // beq rs, rt, label
            // if rs == rt, pc = label
        }
        "bne" => {
            let rs = it.next().unwrap();
            let rt = it.next().unwrap();
            let label = it.next().unwrap();
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![rs.to_string(), rt.to_string(), label.to_string()],
            }
            // bne rs, rt, label
            // if rs != rt, pc = label
        }
        "j" => {
            let label = it.next().unwrap();
            // j label
            // pc = label
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![label.to_string()],
            }
        }
        "jal" => {
            let label = it.next().unwrap();
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![label.to_string()],
            }
            // jal label
            // $ra = pc + 4
        }
        "mov" => {
            let rd = it.next().unwrap();
            let rs = it.next().unwrap();
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![rd.to_string(), rs.to_string()],
            }
            // mov rd, rs
            // rd = rs
        }
        "movi" => {
            let rd = it.next().unwrap();
            let imm = it.next().unwrap();
            Line {
                label: None,
                instr: first.to_string(),
                args: vec![rd.to_string(), imm.to_string()],
            }
            // movi rd, imm
            // rd = imm
        }
        "exit" => Line {
            label: None,
            instr: first.to_string(),
            args: vec![],
        },
        &_ => todo!(),
    }
}

pub struct Prog {
    lines: Vec<Line>,
    labels: HashMap<String, u32>,
    state: State,
}

fn parse_prog(p: &mut Prog, prog: String) {
    let mut pc = 0;
    for line in prog.lines() {
        let line = parse_line(line.to_string());
        if line.label.is_some() {
            p.labels.insert(line.label.unwrap(), pc);
        } else {
            p.lines.push(line);
            pc += 4;
        }
    }
}

pub fn dump_prog(p: &Prog) {
    for line in p.lines.as_slice() {
        if !line.label.is_some() {
            log(format!("{} {}", line.instr, line.args.join(", ")).as_str());
        }
    }
    for label in p.labels.clone() {
        log(format!("{}: {}", label.1, label.0).as_str())
    }
    log(p.state.dump().as_str());
}

fn get_reg_id_from_name(name: &String) -> u32 {
    if name.chars().nth(0 as usize).unwrap() == 'r' {
        return name[1..].parse::<u32>().unwrap();
    }
    return 1;
}

fn do_line(p: &mut Prog) {
    let cur_pc = p.state.pc.borrow();
    let idx = cur_pc / 4;
    if idx >= (p.lines.len() as i32) {
        log("Prog Done");
        p.state.pc = -1;
        return;
    }
    let cur_line = p.lines[idx as usize].borrow();
    p.state.pc += 4;
    match cur_line.instr.as_str() {
        "mov" => {
            let rd = &cur_line.args[0];
            let rs = &cur_line.args[1];
            let rd = get_reg_id_from_name(rd);
            let rs = get_reg_id_from_name(rs);
            let val = p.state.read_reg(rs);
            p.state.set_reg(rd, val);
        }
        "add" => {
            let rd = &cur_line.args[0];
            let rs = &cur_line.args[1];
            let rt = &cur_line.args[2];
            let rd = get_reg_id_from_name(rd);
            let rs = get_reg_id_from_name(rs);
            let rt = get_reg_id_from_name(rt);
            let val = p.state.read_reg(rs) + p.state.read_reg(rt);
            p.state.set_reg(rd, val);
        }
        "movi" => {
            let rd = &cur_line.args[0];
            let imm = &cur_line.args[1];
            let rd = get_reg_id_from_name(rd);
            let imm = imm.parse::<u32>().unwrap();
            p.state.set_reg(rd, imm);
        }
        "j" => {
            let label = &cur_line.args[0];
            let label = p.labels.get(label).unwrap();
            p.state.pc = *label as i32;
        }
        "exit" => {
            log("Exiting gracefully");
            p.state.pc = -1;
        }
        "lw" => {
            let rt = &cur_line.args[0];
            let rs = &cur_line.args[1];
            let imm = &cur_line.args[2];
            let rt = get_reg_id_from_name(rt);
            let rs = get_reg_id_from_name(rs);
            let imm = imm.parse::<u32>().unwrap();
            let addr = p.state.read_reg(rs) + imm;
            let val = p.state.read_mem(addr);
            p.state.set_reg(rt, val as u32);
        }
        "sw" => {
            let rt = &cur_line.args[0];
            let rs = &cur_line.args[1];
            let imm = &cur_line.args[2];
            let rt = get_reg_id_from_name(rt);
            let rs = get_reg_id_from_name(rs);
            let imm = imm.parse::<u32>().unwrap();
            let addr = p.state.read_reg(rs) + imm;
            let val = p.state.read_reg(rt);
            p.state.write_mem(addr, val as u8);
        }
        &_ => todo!(),
    }
}
#[wasm_bindgen]
pub fn run_prog(prog: String) {
    set_panic_hook();
    let mut p = Prog {
        lines: vec![],
        labels: HashMap::new(),
        state: State::new(),
    };
    parse_prog(&mut p, prog);
    log(format!("prog_len {}", p.lines.len()).as_str());
    loop {
        if p.state.pc < 0 {
            return;
        }
        do_line(&mut p);
        update_reg_file(Uint32Array::from(p.state.regs.as_slice()));
    }
}
