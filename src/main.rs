use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start rtl_fm
    println!("Starting rtl_fm");
    let mut rtl = Command::new("rtl_fm")
        .args([
            "-f", "169.65M", 
            "-M", "fm", 
            "-s", "22050"
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    // Take rtl_fm's stdout
    let rtl_stdout = rtl.stdout.take().expect("Failed to capture rtl_fm stdout");

    // Start multimon-ng, feeding rtl_fm's stdout into its stdin
    println!("Starting multimon_ng");
    let mut multimon = Command::new("multimon-ng")
    .args([
        "-a", "FLEX",
        "-t", "raw",
        "/dev/stdin"
    ])
    .stdin(Stdio::from(rtl_stdout))
    .stdout(Stdio::piped())
    .spawn()?;

    let stdout = multimon.stdout.take().expect("Failed to capture multimon stdout");

    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        println!("{}", line?);
    }

    // Clean up child processes
    multimon.wait()?;
    rtl.wait()?;

    // Or: if the program exits early:
    // let _ = multimon.kill();
    // let _ = rtl.kill();

    Ok(())

}


