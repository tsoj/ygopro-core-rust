//! An agent as an engine process on stdio (PROTOCOL.md): JSON lines from the arbiter
//! ("isready", "init", "prompt", "end", "quit"), a response line per prompt (with "info": the
//! search's value, depth, nodes and seconds, if it searched). A prompt's "nodes" is the node limit
//! for that decision. A new game ("init") gets a new agent, except the first one, which "isready"
//! created and warmed up.

use std::io::{BufRead, Write};

use serde_json::{Value, json};

use crate::game::{Agent, InitInfo};
use crate::messages::Msg;
use crate::state::Snapshot;

/// Exits when the process's parent is gone (the arbiter died): a watcher thread.
fn end_with_parent() {
    let parent = std::os::unix::process::parent_id();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if std::os::unix::process::parent_id() != parent {
                std::process::exit(0);
            }
        }
    });
}

fn field<T: serde::de::DeserializeOwned>(msg: &Value, key: &str) -> Result<T, String> {
    serde_json::from_value(msg.get(key).cloned().unwrap_or(Value::Null)).map_err(|e| format!("{key}: {e}"))
}

/// Serves agents made by `factory` until "quit" or the end of the input.
pub fn serve(mut factory: impl FnMut() -> Result<Box<dyn Agent>, String>) -> Result<(), String> {
    end_with_parent();
    let stdin = std::io::stdin();
    let mut out = std::io::stdout().lock();
    let mut send = |v: &Value| -> Result<(), String> {
        let mut line = serde_json::to_string(v).map_err(|e| e.to_string())?;
        line.push('\n');
        out.write_all(line.as_bytes()).and_then(|_| out.flush()).map_err(|e| e.to_string())
    };
    let (mut agent, mut fresh): (Option<Box<dyn Agent>>, bool) = (None, false);
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        let msg: Value = serde_json::from_str(&line).map_err(|e| format!("bad input {line:?}: {e}"))?;
        match msg.get("type").and_then(Value::as_str).unwrap_or("") {
            "isready" => {
                if agent.is_none() {
                    agent = Some(factory()?);
                    fresh = true;
                }
                send(&json!({"ready": true}))?;
            }
            "init" => {
                if agent.is_none() || !fresh {
                    agent = Some(factory()?);
                }
                fresh = false;
                let init: InitInfo = serde_json::from_value(msg.clone()).map_err(|e| format!("init: {e}"))?;
                let player: u32 = field(&msg, "player")?;
                agent.as_mut().unwrap().start(player, &init).map_err(|e| format!("{e:?}"))?;
            }
            "prompt" => {
                let a = agent.as_mut().ok_or("a prompt before init")?;
                for o in field::<Vec<Msg>>(&msg, "obs")? {
                    a.observe(&o);
                }
                let clock = msg.get("clock").filter(|c| !c.is_null()).map(|c| (c["remaining"].as_f64().unwrap_or(0.0), c["increment"].as_f64().unwrap_or(0.0)));
                a.set_node_limit(msg.get("nodes").and_then(Value::as_u64));
                let prompt: Msg = field(&msg, "prompt")?;
                let state: Option<Snapshot> = field(&msg, "state")?;
                let resp = a.respond(&prompt, state.as_ref(), clock).map_err(|e| format!("{e:?}"))?;
                match a.search_info() {
                    Some(i) => send(&json!({"response": resp, "info": {"value": i.value, "depth": i.depth, "nodes": i.nodes, "secs": i.secs}}))?,
                    None => send(&json!({"response": resp}))?,
                }
            }
            "end" => {
                if let Some(a) = agent.as_mut() {
                    for o in field::<Vec<Msg>>(&msg, "obs")? {
                        a.observe(&o);
                    }
                    let result: serde_json::Map<String, Value> = field(&msg, "result").unwrap_or_default();
                    a.end(&result);
                }
            }
            "quit" => break,
            t => return Err(format!("unknown message type {t:?}")),
        }
    }
    Ok(())
}
