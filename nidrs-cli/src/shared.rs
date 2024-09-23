use std::io::BufRead;

pub fn exec_cmd(prefix: &str, cmd: &mut std::process::Command) -> Result<(), anyhow::Error> {
    let mut child = cmd
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    if let Some(stderr) = child.stderr.take() {
        let reader = std::io::BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                line.split("\r").for_each(|line| {
                    eprintln!("[{}] {}", prefix, line);
                });
            }
        }
    }

    if let Some(stdout) = child.stdout.take() {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                line.split("\r").for_each(|line| {
                    println!("[{}] {}", prefix, line);
                });
            }
        }
    }

    let status = child.wait().expect("failed to wait for process");

    // 检查命令是否成功执行
    if !status.success() {
        // 如果失败，打印错误消息
        eprintln!("[{}] command failed with error: {}", prefix, status);
        return Err(anyhow::Error::msg("command failed"));
    }

    // std::io::stdout().flush().expect("Failed to flush stdout");
    // std::io::stderr().flush().expect("Failed to flush stderr");

    Ok(())
}
