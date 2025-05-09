use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input.csv>", args[0]);
        std::process::exit(1);
    }
    let input_path = &args[1];

    // Create output path by replacing .csv with .txt
    let output_path = Path::new(input_path)
        .with_extension("txt")
        .to_str()
        .unwrap()
        .to_string();

    // Instruction to hex mapping
    let instructions: HashMap<&str, u8> = HashMap::from([
        // scalar add
        ("AddC1", 0x00),
        ("AddC2", 0x01),
        ("AddRegs", 0x02),
        ("AddCts", 0x03),
        // scalar subtraction
        ("SubC1", 0x04),
        ("SubC2", 0x05),
        ("SubRegs", 0x06),
        ("SubCts", 0x07),
        // Scalar multiplication
        ("MulC1", 0x08),
        ("MulC2", 0x09),
        ("MulRegs", 0x0A),
        ("MulCts", 0x0B),
        // Scalar division
        ("DivC1", 0x0C),
        ("DivC2", 0x0D),
        ("DivRegs", 0x0E),
        ("DivCts", 0x0F),
        // Scalar comparison
        ("CompC1", 0x10),
        ("CompC2", 0x11),
        ("CompRegs", 0x12),
        ("CompCts", 0x13),
        // Logical AND
        ("AndC1", 0x20),
        ("AndC2", 0x21),
        ("AndRegs", 0x22),
        ("AndCts", 0x23),
        // Logical OR
        ("OrC1", 0x24),
        ("OrC2", 0x25),
        ("OrRegs", 0x26),
        ("OrCts", 0x27),
        // Clear / Zero
        ("Clear", 0x30),
        // Jump:
        ("Jump", 0xFF),
        // No operation
        ("Nop", 0xFE), // Nop is now at FE because at FF the multiplexer is set to read data from
        // both counters, which causes the overflow register to go nuts, even if it doesn't write
        ("Load", 0x40),
        ("Store", 0x41),
        ("VecLoad", 0xC0),
        ("VecStore", 0xC1),
        // Vector Functions
        ("VecAdd", 0x82),
        ("VecSub", 0x84),
        ("VecDiv", 0x8C),
        ("VecMul", 0xA0),
        // Dot Product
        ("Dot", 0xB0),
        // Transfer Function
        ("TFunct", 0xE0),
        // No operation
        ("Nop", 0xFE),
    ]);

    // Open input file
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    // Create output file
    let mut output_file = File::create(&output_path)?;
    writeln!(output_file, "v2.0 raw")?;

    // Store all converted lines and remember the first one
    let mut converted_lines = Vec::new();
    let mut first_line: Option<String> = None;

    // Process each line from input
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() != 4 {
            eprintln!("Invalid line format: {}", line);
            continue;
        }

        let instruction = parts[0];
        let numbers: Vec<u8> = parts[1..4]
            .iter()
            .map(|&s| {
                s.parse().unwrap_or_else(|_| {
                    eprintln!("Invalid number: {}", s);
                    0
                })
            })
            .collect();

        // Get instruction code
        let instr_code = instructions.get(instruction).unwrap_or_else(|| {
            eprintln!("Unknown instruction: {}", instruction);
            &0xFF
        });

        // Format as 8 hex digits
        let hex_line = format!(
            "{:02X}{:02X}{:02X}{:02X}",
            instr_code, numbers[0], numbers[1], numbers[2]
        );

        // Store the first line for padding
        if first_line.is_none() {
            first_line = Some(hex_line.clone());
        }
        converted_lines.push(hex_line);
    }

    // Ensure we have at least one line to pad with
    let padding_line = first_line.unwrap_or_else(|| "00000000".to_string());

    // Write all lines and pad to 256 if needed
    for i in 0..256 {
        let line = if i < converted_lines.len() {
            &converted_lines[i]
        } else {
            &padding_line
        };
        writeln!(output_file, "{}", line)?;
    }

    println!(
        "Conversion complete. Output written to {} with 256 lines",
        output_path
    );
    Ok(())
}
