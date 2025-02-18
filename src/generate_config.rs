use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

#[derive(Serialize, Deserialize, Debug)]
struct StarkFri {
    fri_step_list: Vec<u32>,
    last_layer_degree_bound: u32,
    n_queries: u32,
    proof_of_work_bits: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stark {
    fri: StarkFri,
    log_n_cosets: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Template {
    field: String,
    channel_hash: String,
    commitment_hash: String,
    n_verifier_friendly_commitment_layers: u32,
    pow_hash: String,
    statement: Value,
    stark: Stark,
    use_extension_field: bool,
    verifier_friendly_channel_updates: bool,
    verifier_friendly_commitment_hash: String,
}

fn calculate_fri_step_list(n_steps: u32, degree_bound: u32) -> Vec<u32> {
    let fri_degree = ((n_steps as f64 / degree_bound as f64).log(2.0).round() as u32) + 4;
    let mut steps = vec![0];
    steps.extend(vec![4; (fri_degree / 4) as usize]);
    if fri_degree % 4 != 0 {
        steps.push(fri_degree % 4);
    }
    steps
}

fn update_template_and_save_to_file(
    template: &mut Template,
    fri_step_list: Vec<u32>,
    file_path: &str,
) -> Result<(), String> {
    template.stark.fri.fri_step_list = fri_step_list;
    let mut file: File = File::create(file_path).map_err(|e| e.to_string())?;
    let json_string = serde_json::to_string_pretty(template).expect("Failed to serialize JSON");
    file.write_all(json_string.as_bytes())
        .map_err(|e| e.to_string())
}

fn read_json_from_file(file_path: &str) -> Result<Value, String> {
    let mut buffer = String::new();
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    file.read_to_string(&mut buffer)
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&buffer).map_err(|e| e.to_string())
}

pub fn generate(input_file: &str, output_file: &str) {
    let program_public_input: Value = match read_json_from_file(input_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: Invalid JSON input. {}", err);
            process::exit(1);
        }
    };

    let n_steps = match program_public_input["n_steps"].as_u64() {
        Some(val) => val as u32,
        None => {
            eprintln!("Error: 'n_steps' is missing or not an integer.");
            process::exit(1);
        }
    };

    let mut template = Template {
        field: "PrimeField0".to_string(),
        channel_hash: "poseidon3".to_string(),
        commitment_hash: "blake256_masked160_lsb".to_string(),
        n_verifier_friendly_commitment_layers: 9999,
        pow_hash: "keccak256".to_string(),
        statement: serde_json::json!({ "page_hash": "pedersen" }),
        stark: Stark {
            fri: StarkFri {
                fri_step_list: vec![0, 4, 4, 4],
                last_layer_degree_bound: 128,
                n_queries: 16,
                proof_of_work_bits: 30,
            },
            log_n_cosets: 3,
        },
        use_extension_field: false,
        verifier_friendly_channel_updates: true,
        verifier_friendly_commitment_hash: "poseidon3".to_string(),
    };

    let last_layer_degree_bound = template.stark.fri.last_layer_degree_bound;

    let fri_step_list = calculate_fri_step_list(n_steps, last_layer_degree_bound);
    let _ = update_template_and_save_to_file(&mut template, fri_step_list, output_file);
}
