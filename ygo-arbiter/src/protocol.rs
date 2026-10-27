//! The engine protocol (PROTOCOL.md): AIs as separate processes talking JSON lines over stdio.
//! [`EngineProcess`] is the arbiter-side [`Agent`] for such a process; [`crate::engine`] is the
//! engine side.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::actions::Response;
use crate::game::{Agent, AgentError, InitInfo, SearchInfo};
use crate::messages::Msg;
use crate::state::Snapshot;

/// (the arbiter-side process: an engine ends with it, see protocol._end_with_parent)
const PARENT_ENV: &str = "YGOAI_ENGINE_PARENT";
/// (engines fork: no BLAS thread pools, a fork would copy their state but not their threads)
const SINGLE_THREADED: [(&str, &str); 3] = [("OPENBLAS_NUM_THREADS", "1"), ("OMP_NUM_THREADS", "1"), ("MKL_NUM_THREADS", "1")];
/// Seconds an engine may take to load (one that does not know "isready" is not waited for longer).
const READY_TIMEOUT: f64 = 30.0;
/// Seconds past its clock an engine may take to respond before it is given up on (the clock
/// itself is checked after the response, as the Python arbiter does).
const CLOCK_GRACE: f64 = 10.0;

/// Splits a command line as shlex.split does (quotes and backslashes).
pub fn shlex_split(s: &str) -> Vec<String> {
    let (mut out, mut cur, mut have) = (Vec::new(), String::new(), false);
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                have = true;
                for c in chars.by_ref() {
                    if c == '\'' {
                        break;
                    }
                    cur.push(c);
                }
            }
            '"' => {
                have = true;
                while let Some(c) = chars.next() {
                    match c {
                        '"' => break,
                        '\\' => {
                            if let Some(n) = chars.next() {
                                if !matches!(n, '"' | '\\' | '$' | '`') {
                                    cur.push('\\');
                                }
                                cur.push(n);
                            }
                        }
                        _ => cur.push(c),
                    }
                }
            }
            '\\' => {
                have = true;
                if let Some(n) = chars.next() {
                    cur.push(n);
                }
            }
            c if c.is_whitespace() => {
                if have {
                    out.push(std::mem::take(&mut cur));
                    have = false;
                }
            }
            c => {
                have = true;
                cur.push(c);
            }
        }
    }
    if have {
        out.push(cur);
    }
    out
}

pub struct EngineProcess {
    pub name: String,
    child: Child,
    stdin: Option<ChildStdin>,
    lines: Receiver<String>,
    pending: Vec<Msg>,
    pub last_think_time: f64,
    /// (the last response's "info": the search's value, depth, nodes and seconds)
    info: Option<SearchInfo>,
    /// (sent with each prompt as "nodes": the engine's node limit for that decision)
    node_limit: Option<u64>,
}

fn engine_error(name: &str, what: &str) -> AgentError {
    AgentError::Failed(format!("EngineError: {name} {what}"))
}

impl EngineProcess {
    /// Starts the engine and waits until it is ready (loading is not on its clock).
    pub fn start(command: &[String], name: &str) -> Result<EngineProcess, AgentError> {
        let mut cmd = Command::new(&command[0]);
        cmd.args(&command[1..]).stdin(Stdio::piped()).stdout(Stdio::piped());
        for (k, v) in SINGLE_THREADED {
            if std::env::var_os(k).is_none() {
                cmd.env(k, v);
            }
        }
        cmd.env(PARENT_ENV, std::process::id().to_string());
        let mut child = cmd.spawn().map_err(|e| engine_error(name, &format!("cannot start: {e}")))?;
        let stdin = child.stdin.take();
        let stdout = child.stdout.take().unwrap();
        let (tx, lines) = channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
        let mut e = EngineProcess { name: name.into(), child, stdin, lines, pending: Vec::new(), last_think_time: 0.0, info: None, node_limit: None };
        e.send(&json!({"type": "isready"}))?;
        match e.lines.recv_timeout(Duration::from_secs_f64(READY_TIMEOUT)) {
            Ok(line) => {
                let reply: Value = serde_json::from_str(&line).map_err(|_| engine_error(name, &format!("sent {line:?}")))?;
                if reply.get("ready") != Some(&Value::Bool(true)) {
                    return Err(engine_error(name, &format!("sent {line} instead of ready")));
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return Err(engine_error(name, "exited")),
        }
        Ok(e)
    }

    fn send(&mut self, v: &Value) -> Result<(), AgentError> {
        let mut line = serde_json::to_string(v).expect("json");
        line.push('\n');
        let stdin = self.stdin.as_mut().ok_or_else(|| engine_error(&self.name, "closed its input"))?;
        stdin.write_all(line.as_bytes()).and_then(|_| stdin.flush()).map_err(|_| engine_error(&self.name, "closed its input"))
    }

    fn recv(&mut self, timeout: Option<f64>) -> Result<Value, AgentError> {
        let line = match timeout {
            None => self.lines.recv().map_err(|_| engine_error(&self.name, "exited"))?,
            Some(t) => match self.lines.recv_timeout(Duration::from_secs_f64(t.max(0.0))) {
                Ok(l) => l,
                Err(RecvTimeoutError::Timeout) => return Err(AgentError::Forfeit("time_forfeit".into())),
                Err(RecvTimeoutError::Disconnected) => return Err(engine_error(&self.name, "exited")),
            },
        };
        serde_json::from_str(&line).map_err(|e| AgentError::Failed(format!("JSONDecodeError: {e}")))
    }

    /// Asks the engine to quit (killed if it does not within 5 s).
    pub fn close(&mut self) {
        if matches!(self.child.try_wait(), Ok(None)) {
            let _ = self.send(&json!({"type": "quit"}));
            self.stdin = None;
            let t = Instant::now();
            while t.elapsed() < Duration::from_secs(5) {
                if !matches!(self.child.try_wait(), Ok(None)) {
                    return;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            let _ = self.child.kill();
        }
        let _ = self.child.wait();
    }
}

impl Drop for EngineProcess {
    fn drop(&mut self) {
        self.close();
    }
}

impl Agent for EngineProcess {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn wants_state(&self) -> bool {
        true
    }
    fn start(&mut self, _player: u32, init: &InitInfo) -> Result<(), AgentError> {
        self.pending.clear();
        let mut msg = serde_json::to_value(init).expect("json");
        msg.as_object_mut().unwrap().insert("type".into(), json!("init"));
        self.send(&msg)
    }
    fn observe(&mut self, msg: &Msg) {
        self.pending.push(msg.clone());
    }
    fn respond(&mut self, prompt: &Msg, state: Option<&Snapshot>, clock: Option<(f64, f64)>) -> Result<Response, AgentError> {
        let t0 = Instant::now();
        let mut msg = json!({"type": "prompt", "obs": std::mem::take(&mut self.pending), "prompt": prompt, "state": state});
        if let Some((remaining, increment)) = clock {
            msg["clock"] = json!({"remaining": remaining, "increment": increment});
        }
        if let Some(n) = self.node_limit {
            msg["nodes"] = json!(n);
        }
        self.send(&msg)?;
        let reply = self.recv(clock.map(|(remaining, _)| remaining + CLOCK_GRACE))?;
        self.last_think_time = t0.elapsed().as_secs_f64();
        let Some(resp) = reply.get("response") else {
            return Err(engine_error(&self.name, &format!("sent {reply} instead of a response")));
        };
        self.info = reply.get("info").and_then(|i| {
            Some(SearchInfo {
                value: i.get("value")?.as_f64()?,
                depth: i.get("depth")?.as_u64()? as u32,
                nodes: i.get("nodes").and_then(Value::as_u64).unwrap_or(0),
                secs: i.get("secs").and_then(Value::as_f64).unwrap_or(0.0),
            })
        });
        serde_json::from_value(resp.clone()).map_err(|e| AgentError::Failed(format!("InvalidResponse: {e}")))
    }
    fn search_info(&mut self) -> Option<SearchInfo> {
        self.info.take()
    }
    fn set_node_limit(&mut self, nodes: Option<u64>) {
        self.node_limit = nodes;
    }
    fn end(&mut self, result: &serde_json::Map<String, Value>) {
        let msg = json!({"type": "end", "obs": std::mem::take(&mut self.pending), "result": result});
        let _ = self.send(&msg);
    }
}
