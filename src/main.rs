use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), String> {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => return Err(String::from("Please provide a path to a .wav file")),
    };

    let mut reader = match hound::WavReader::open(&path) {
        Ok(reader) => reader,
        Err(e) => return Err(format!("Failed to open {}: {}", path, e)),
    };

    println!("Loaded file successfully!");

    let mut as_utf_16 = Vec::with_capacity(reader.len() as usize);

    let pb = ProgressBar::new(reader.len() as u64);
    pb.set_style(ProgressStyle::with_template("{msg}{spinner:.green} [{percent}%] [{wide_bar:.green/dark_green}] {human_pos}/{human_len}").unwrap().progress_chars("#>-"));

    for sample in reader.samples::<i16>() {
        let sample = ((sample.unwrap_or(0) as i32) + (1 << 15)) as u16;

        if sample == 96 || sample == 92 {
            as_utf_16.push(92u16);
        }

        as_utf_16.push(sample);

        pb.inc(1);
        pb.tick();
    }

    pb.finish_with_message("Read contents of file!");

    let file_name = Path::new(&path).file_stem().unwrap().to_str().unwrap();

    let mut file = File::create("./".to_owned() + file_name + ".js").unwrap();

    file.write_all(
        format!(
            "// Set sample rate to {}\nt||(c=`",
            reader.spec().sample_rate
        )
        .as_bytes(),
    )
    .unwrap();
    file.write_all(bytemuck::cast_slice(&as_utf_16[..]))
        .unwrap();
    file.write_all("`),c.charCodeAt(t%c.length)/32768-1".as_bytes())
        .unwrap();

    println!("Wrote JS code");

    Ok(())
}
