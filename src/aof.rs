use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader};

use crate::resp::{self, Command};
use crate::store::Store;

pub fn append(command: &Command) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("aof.log")?;

    let serialised = match &command {
        Command::Set(k, v) => format!("*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n{}\r\n", k.len(), k, v.len(), v),
        Command::Del(keys) => {
            let mut serialised = format!("*{}\r\n$3\r\nDEL\r\n", keys.len() + 1);
            for key in keys {
                serialised.push_str(&format!("${}\r\n{}\r\n", key.len(), key));
            }
            serialised
        },
        _ => return Ok(()),
    };

    file.write_all(serialised.as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn replay(store: &Store) -> anyhow::Result<()> {
    let file = match File::open("aof.log") {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    let reader = BufReader::new(file);
    let mut buffer = Vec::new();
    let mut expected_lines = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.starts_with("*") {
            buffer.clear();
            expected_lines = trimmed[1..].parse::<usize>().unwrap_or(0) * 2;
            buffer.push(trimmed.to_string());
        } else {
            buffer.push(trimmed.to_string());

            if buffer.len() != expected_lines + 1 {
                continue;
            }

            let joined = buffer.join("\r\n");
            let command = resp::parse(&joined);

            match &command {
                Command::Set(k, v) => store.set(&k, &v),
                Command::Del(keys) => { store.del(&keys); },
                _ => {},
            }

            buffer.clear();
        }

    }

    Ok(())
}
