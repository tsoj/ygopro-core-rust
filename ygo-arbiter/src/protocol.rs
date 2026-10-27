//! The engine protocol (PROTOCOL.md): AIs as separate processes talking JSON lines over stdio.
//! [`EngineProcess`] is the arbiter-side [`Agent`] for such a process; [`crate::engine`] is the
//! engine side.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::actions::Response;
use crate::game::{Agent, AgentError, InitInfo, SearchInfo};
use crate::messages::Msg;
use crate::state::Snapshot;

/// The version of the protocol this arbiter speaks (sent with "hello").
pub const PROTOCOL_VERSION: u32 = 1;
/// Seconds an engine may take to answer "hello", and to load (to answer "isready").
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

/// An engine's identity (the "id" of its "hello" reply): its name, and further members (author,
/// version, ...) whose values are strings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    #[serde(flatten)]
    pub more: Map<String, Value>,
}

/// An option an engine declares (in its "hello" reply).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum EngineOption {
    /// true or false
    Check { name: String, default: bool },
    /// an integer from min to max
    Spin { name: String, default: i64, min: i64, max: i64 },
    /// one of values
    Combo { name: String, default: String, values: Vec<String> },
    /// any text
    String { name: String, default: String },
}

impl EngineOption {
    pub fn name(&self) -> &str {
        match self {
            EngineOption::Check { name, .. } | EngineOption::Spin { name, .. } | EngineOption::Combo { name, .. } | EngineOption::String { name, .. } => name,
        }
    }

    pub fn default_value(&self) -> Value {
        match self {
            EngineOption::Check { default, .. } => json!(default),
            EngineOption::Spin { default, .. } => json!(default),
            EngineOption::Combo { default, .. } | EngineOption::String { default, .. } => json!(default),
        }
    }

    /// The value `text` (from a command line) stands for, if it is a valid value of the option.
    pub fn parse(&self, text: &str) -> Result<Value, String> {
        let v = match self {
            EngineOption::Check { .. } => match text {
                "true" => json!(true),
                "false" => json!(false),
                _ => return Err(format!("option {}: {text:?} is not true or false", self.name())),
            },
            EngineOption::Spin { .. } => json!(text.parse::<i64>().map_err(|_| format!("option {}: {text:?} is not an integer", self.name()))?),
            EngineOption::Combo { .. } | EngineOption::String { .. } => json!(text),
        };
        self.check(&v)?;
        Ok(v)
    }

    /// Whether `v` is a valid value of the option: of its type, in its range or among its values.
    pub fn check(&self, v: &Value) -> Result<(), String> {
        let ok = match self {
            EngineOption::Check { .. } => v.is_boolean(),
            EngineOption::Spin { min, max, .. } => v.as_i64().is_some_and(|x| (*min..=*max).contains(&x)),
            EngineOption::Combo { values, .. } => v.as_str().is_some_and(|x| values.iter().any(|c| c == x)),
            EngineOption::String { .. } => v.is_string(),
        };
        if ok { Ok(()) } else { Err(format!("option {}: {v} is not a valid value ({})", self.name(), serde_json::to_string(self).unwrap())) }
    }
}

/// An engine's "hello" reply: its identity and its options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hello {
    pub id: Identity,
    #[serde(default)]
    pub options: Vec<EngineOption>,
}

pub struct EngineProcess {
    pub name: String,
    /// its identity and options
    pub hello: Hello,
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
    /// Starts the engine, sets its `options` (name and value as text) and waits until it is ready
    /// (loading is not on its clock).
    pub fn start(command: &[String], name: &str, options: &[(String, String)]) -> Result<EngineProcess, AgentError> {
        let mut cmd = Command::new(&command[0]);
        cmd.args(&command[1..]).stdin(Stdio::piped()).stdout(Stdio::piped());
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
        let hello = Hello { id: Identity { name: String::new(), more: Map::new() }, options: Vec::new() };
        let mut e = EngineProcess { name: name.into(), hello, child, stdin, lines, pending: Vec::new(), last_think_time: 0.0, info: None, node_limit: None };
        e.send(&json!({"type": "hello", "protocol": PROTOCOL_VERSION}))?;
        let line = e.handshake_line("hello")?;
        e.hello = serde_json::from_str(&line).map_err(|err| engine_error(name, &format!("sent {line} instead of its id and options: {err}")))?;
        if let Some((k, v)) = e.hello.id.more.iter().find(|(_, v)| !v.is_string()) {
            return Err(engine_error(name, &format!("id: {k} is {v}, not a string")));
        }
        for (key, text) in options {
            let Some(option) = e.hello.options.iter().find(|o| o.name() == key) else {
                let names: Vec<&str> = e.hello.options.iter().map(EngineOption::name).collect();
                let has = if names.is_empty() { "none".to_string() } else { names.join(", ") };
                return Err(engine_error(name, &format!("has no option {key} (its options: {has})")));
            };
            let value = option.parse(text).map_err(|err| engine_error(name, &err))?;
            e.send(&json!({"type": "setoption", "name": key, "value": value}))?;
        }
        e.send(&json!({"type": "isready"}))?;
        let line = e.handshake_line("isready")?;
        let reply: Value = serde_json::from_str(&line).map_err(|_| engine_error(name, &format!("sent {line:?}")))?;
        if reply.get("ready") != Some(&Value::Bool(true)) {
            return Err(engine_error(name, &format!("sent {line} instead of ready")));
        }
        Ok(e)
    }

    /// The engine's reply to `what` (a message of the handshake).
    fn handshake_line(&mut self, what: &str) -> Result<String, AgentError> {
        match self.lines.recv_timeout(Duration::from_secs_f64(READY_TIMEOUT)) {
            Ok(line) => Ok(line),
            Err(RecvTimeoutError::Timeout) => Err(engine_error(&self.name, &format!("did not answer {what} within {READY_TIMEOUT} s"))),
            Err(RecvTimeoutError::Disconnected) => Err(engine_error(&self.name, "exited")),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_values() {
        let check = EngineOption::Check { name: "Ponder".into(), default: false };
        assert_eq!(check.parse("true"), Ok(json!(true)));
        assert!(check.parse("1").is_err());
        let spin = EngineOption::Spin { name: "Hash".into(), default: 16, min: 1, max: 64 };
        assert_eq!(spin.parse("64"), Ok(json!(64)));
        assert!(spin.parse("65").is_err() && spin.parse("x").is_err() && spin.check(&json!("16")).is_err());
        let combo = EngineOption::Combo { name: "Style".into(), default: "a".into(), values: vec!["a".into(), "b".into()] };
        assert_eq!(combo.parse("b"), Ok(json!("b")));
        assert!(combo.parse("c").is_err());
        let string = EngineOption::String { name: "Net".into(), default: String::new() };
        assert_eq!(string.parse("x.onnx"), Ok(json!("x.onnx")));
    }

    #[test]
    fn hello_reply() {
        // (members this version does not know are ignored; the id keeps its own)
        let line = r#"{"id": {"name": "e", "author": "a"}, "options": [{"type": "spin", "name": "Hash", "default": 16, "min": 1, "max": 64, "unit": "MB"}], "later": 1}"#;
        let hello: Hello = serde_json::from_str(line).unwrap();
        assert_eq!(hello.id.name, "e");
        assert_eq!(hello.id.more["author"], json!("a"));
        assert_eq!(hello.options, vec![EngineOption::Spin { name: "Hash".into(), default: 16, min: 1, max: 64 }]);
        let bare: Hello = serde_json::from_str(r#"{"id": {"name": "e"}}"#).unwrap();
        assert!(bare.options.is_empty());
    }
}
