use crate::Prog;

#[derive(Clone)]
pub struct Line {
    pub label: Option<String>,
    pub instr: String,
    pub args: Vec<String>,
}

pub fn parse_prog(p: &mut Prog, prog: String) {
    let mut pc = 0;
    for line in prog.lines() {
        let line = parse_line(line.to_string());
        match line {
            Some(l) => {
                match l.clone().label {
                    Some(l) => {
                        p.labels.insert(l, pc);
                    }
                    None => {}
                }
                match l.clone().instr.as_str() {
                    "" => {} // line with label, no instr
                    &_ => {
                        p.lines.push(l);
                        pc += 4;
                    }
                }
            }
            None => {}
        }
    }
}

fn parse_line(line: String) -> Option<Line> {
    let mut it = line.split_whitespace();
    let mut first = it.next()?;
    let label = {
        if first.chars().nth(0).unwrap() == '.' {
            // label in line
            let rv = Some(first[1..].to_string());
            first = match it.next() {
                Some(s) => s,
                None => {
                    return Some(Line { // label, but not instr
                        label: rv,
                        instr: String::from(""),
                        args: vec![],
                    })
                }
            };
            rv // label & instr in line
        } else {
            None
        }
    };

    match first {
        "add" => {
            let rd = it.next().unwrap();
            let rs = it.next().unwrap();
            let rt = it.next().unwrap();
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![rd.to_string(), rs.to_string(), rt.to_string()],
            })
            // add rd, rs, rt
            // rd = rs + rt
        }
        "sub" => {
            let rd = it.next().unwrap();
            let rs = it.next().unwrap();
            let rt = it.next().unwrap();
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![rd.to_string(), rs.to_string(), rt.to_string()],
            })
            // sub rd, rs, rt
            // rd = rs - rt
        }
        "lw" => {
            let rt = it.next().unwrap();
            let rs = it.next().unwrap();
            let imm = it.next().unwrap();
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![rt.to_string(), rs.to_string(), imm.to_string()],
            })
            // lw rt, imm(rs)
            // rt = mem[rs + imm]
        }
        "sw" => {
            let rt = it.next().unwrap();
            let rs = it.next().unwrap();
            let imm = it.next().unwrap();
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![rt.to_string(), rs.to_string(), imm.to_string()],
            })
            // sw rt, imm(rs)
            // mem[rs + imm] = rt
        }
        "beq" => {
            let rs = it.next().unwrap();
            let rt = it.next().unwrap();
            let label2 = it.next().unwrap();
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![rs.to_string(), rt.to_string(), label2.to_string()],
            })
            // beq rs, rt, label
            // if rs == rt, pc = label
        }
        "bne" => {
            let rs = it.next().unwrap();
            let rt = it.next().unwrap();
            let label2 = it.next().unwrap();
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![rs.to_string(), rt.to_string(), label2.to_string()],
            })
            // bne rs, rt, label
            // if rs != rt, pc = label
        }
        "j" => {
            let label2 = it.next().unwrap();
            // j label
            // pc = label
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![label2.to_string()],
            })
        }
        "jal" => {
            let label2 = it.next().unwrap();
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![label2.to_string()],
            })
            // jal label
            // $ra = pc + 4
        }
        "mov" => {
            let rd = it.next().unwrap();
            let rs = it.next().unwrap();
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![rd.to_string(), rs.to_string()],
            })
            // mov rd, rs
            // rd = rs
        }
        "movi" => {
            let rd = it.next().unwrap();
            let imm = it.next().unwrap();
            Some(Line {
                label: label,
                instr: first.to_string(),
                args: vec![rd.to_string(), imm.to_string()],
            })
            // movi rd, imm
            // rd = imm
        }
        "exit" => Some(Line {
            label: None,
            instr: first.to_string(),
            args: vec![],
        }),
        &_ => todo!(),
    }
}
