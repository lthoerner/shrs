use std::{
    fmt::Display,
    io::{stderr, stdout, BufWriter, Write},
};

use crossterm::{
    style::{Print, PrintStyledContent},
    QueueableCommand,
};
use shrs_utils::styled_buf::StyledBuf;

pub struct OutputWriter {
    stdout: BufWriter<std::io::Stdout>,
    stderr: BufWriter<std::io::Stderr>,
    collecting: bool,
    out: String,
    err: String,
}
impl Default for OutputWriter {
    fn default() -> Self {
        Self {
            stdout: BufWriter::new(stdout()),
            stderr: BufWriter::new(stderr()),
            collecting: false,
            out: String::new(),
            err: String::new(),
        }
    }
}
impl OutputWriter {
    pub fn begin_collecting(&mut self) {
        self.collecting = true;
    }
    pub fn eprint<T: Display>(&mut self, s: T) -> anyhow::Result<()> {
        if self.collecting {
            self.err.push_str(s.to_string().as_str());
        }

        self.stderr.queue(Print(s))?;
        self.stderr.flush()?;
        Ok(())
    }
    pub fn eprintln<T: Display>(&mut self, s: T) -> anyhow::Result<()> {
        self.eprint(s)?;
        self.eprint("\r\n")?;
        Ok(())
    }

    pub fn print<T: Display>(&mut self, s: T) -> anyhow::Result<()> {
        if self.collecting {
            self.out.push_str(s.to_string().as_str());
        }
        self.stdout.queue(Print(s))?;
        self.stdout.flush()?;
        Ok(())
    }
    pub fn println<T: Display>(&mut self, s: T) -> anyhow::Result<()> {
        self.print(s)?;
        self.print("\r\n")?;
        Ok(())
    }
    pub fn end_collecting(&mut self) -> (String, String) {
        self.collecting = false;
        (self.out.drain(..).collect(), self.err.drain(..).collect())
    }
    pub fn print_buf(&mut self, buf: StyledBuf) -> anyhow::Result<()> {
        let lines = buf.lines();

        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                self.print("\r\n")?;
            }
            for span in line {
                self.stdout.queue(PrintStyledContent(span.clone()))?;
            }
        }

        Ok(())
    }
}
