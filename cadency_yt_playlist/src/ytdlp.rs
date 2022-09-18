use std::{
    io,
    process::Command,
    process::{Output, Stdio},
};

const YOUTUBE_DL_COMMAND: &str = "yt-dlp";

pub(crate) struct YtDlp {
    command: Command,
    args: Vec<String>,
}

impl YtDlp {
    pub fn new() -> Self {
        let mut cmd = Command::new(YOUTUBE_DL_COMMAND);
        cmd.env("LC_ALL", "en_US.UTF-8")
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped());
        Self {
            command: cmd,
            args: vec![],
        }
    }

    pub fn arg(&mut self, arg: String) -> &mut Self {
        self.args.push(arg);
        self
    }

    pub fn execute(&mut self) -> io::Result<Output> {
        self.command
            .args(self.args.clone())
            .spawn()
            .and_then(|child| child.wait_with_output())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_build_new_instance() {
        let some_res = YtDlp::new()
            .arg("--version".to_string())
            .execute()
            .expect("To execute yt-dlp");
        let status_code = some_res.status.code().unwrap();
        assert!(
            some_res.status.success(),
            "Status code is actually: {status_code}"
        );
    }
}
