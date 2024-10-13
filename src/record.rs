use std::fs::File;
use std::io::Write;

pub fn record_iter(file: &str, iter: i32, value: f64) -> std::io::Result<()> {
    let file = File::open(file)?;
    let mut writer = std::io::BufWriter::new(file);
    writeln!(writer, "{}, {}", iter, value)?;
    writer.flush()?;
    Ok(())
}
