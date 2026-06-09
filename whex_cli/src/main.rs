use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufRead};

const COLOR_OFFSET: &str = "\x1b[90m";
const COLOR_HEX: &str = "\x1b[32m";
const COLOR_ASCII: &str = "\x1b[34m";
const COLOR_RESET: &str = "\x1b[0m";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: whex <path_to_file> [--lines <N>] [--size <N>] [--output <dump.txt>]");
        
        return Ok(());
    }
    let filename = &args[1];

    if filename.ends_with(".txt") {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let line = line_result?;

            if let Some(colon_idx) = line.find(":") {
                if let Some(pipe_idx) = line.find("|") {
                    let offset_part = &line[..colon_idx + 1];
                    let hex_part = &line[colon_idx + 1..pipe_idx + 1];
                    let ascii_part = &line[pipe_idx + 1..];

                    // Выводим каждую часть в своем цвете
                    print!("{}{}{}", COLOR_OFFSET, offset_part, COLOR_RESET);
                    print!("{}{}{}", COLOR_HEX, hex_part, COLOR_RESET);
                    println!("{}{}{}", COLOR_ASCII, ascii_part, COLOR_RESET);
                    continue;
                }
            }
            println!("{}", line);
        }
        return Ok(());
    }

    let mut max_lines: Option<usize> = None;
    let mut row_size: usize = 16;
    let mut output_filename: Option<String> = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--lines" => {
                if i + 1 < args.len() {
                    if let Ok(num) = args[i + 1].parse::<usize>() {
                        max_lines = Some(num);
                    }
                    i += 2;
                } else {
                    println!("Error: value for --lines is missing");
                    return Ok(());
                }
            }
            "--size" => {
                if i + 1 < args.len() {
                    if let Ok(num) = args[i + 1].parse::<usize>() {
                        if num > 0 { row_size = num; }
                    }
                    i += 2;
                } else {
                    println!("Error: value for --size is missing");
                    return Ok(());
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    output_filename = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    println!("Error: value for --output is missing");
                    return Ok(());
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                return Ok(());
            }
        }
    }

    let mut file = File::open(filename)?;

    let mut out_writer: Box<dyn Write> = match &output_filename {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };

    let use_colors = output_filename.is_none();

    let mut buffer = vec![0u8; row_size];
    let mut offset = 0;
    let mut lines_printed = 0;

    while let Ok(bytes_read) = file.read(&mut buffer){
        if bytes_read == 0{
            break;
        }
        if let Some(limit) = max_lines {
            if lines_printed >= limit {break;}
        }

        if use_colors { write!(out_writer, "{}", COLOR_OFFSET)?; }
        write!(out_writer, "{:08X}: ", offset)?;
        if use_colors { write!(out_writer, "{}", COLOR_RESET)?; }

        if use_colors { write!(out_writer, "{}", COLOR_HEX)?; }
        for j in 0..row_size {
            if j < bytes_read {
                write!(out_writer, "{:02X} ", buffer[j])?;
            } else {
                write!(out_writer, "   ")?;
            }
        }
        if use_colors { write!(out_writer, "{}", COLOR_RESET)?; }

        write!(out_writer, " | ")?;

        if use_colors { write!(out_writer, "{}", COLOR_ASCII)?; }
        for j in 0..bytes_read {
            let byte = buffer[j];
            if byte >= 32 && byte <= 126 {
                write!(out_writer, "{}", byte as char)?;
            } else {
                write!(out_writer, ".")?;
            }
        }
        if use_colors { write!(out_writer, "{}", COLOR_RESET)?; }

        writeln!(out_writer)?;

        offset += bytes_read;
        lines_printed += 1;
    }

    if let Some(path) = output_filename {
        println!("The hex dump has been successfully saved to a file: {}", path);
    }

    Ok(())
}
