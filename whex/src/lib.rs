use pyo3::prelude::*;
use std::fs::File;
use std::io::{Read};

#[pyfunction]
#[pyo3(signature = (filename, row_size=16, max_lines=None))]
fn create_dump(filename: String, row_size: usize, max_lines: Option<usize>) -> PyResult<Vec<String>> {
    let mut file = File::open(filename)?;
    let mut buffer = vec![0u8; row_size];
    let mut offset = 0;
    let mut lines_printed = 0;

    let mut result_lines = Vec::new();

    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 { break; }
        if let Some(limit) = max_lines {
            if lines_printed >= limit { break; }
        }

        let mut line = format!("{:08X}: ", offset);

        for j in 0..row_size {
            if j < bytes_read {
                line.push_str(&format!("{:02X} ", buffer[j]));
            } else {
                line.push_str("   ");
            }
        }

        line.push_str(" | ");

        for j in 0..bytes_read {
            let byte = buffer[j];
            if byte >= 32 && byte <= 126 {
                line.push(byte as char);
            } else {
                line.push('.');
            }
        }

        result_lines.push(line);
        offset += bytes_read;
        lines_printed += 1;
    }

    Ok(result_lines)
}

#[pymodule]
fn whex(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_dump, m)?)?;
    Ok(())
}