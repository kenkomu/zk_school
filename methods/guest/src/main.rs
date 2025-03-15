use risc0_zkvm::guest::env;

fn main() {
    // Read the student ID (private data)
    let student_id: u32 = env::read();
    
    // For simplicity, let's say valid student IDs are:
    // 1. Between 10000 and 99999
    // 2. Must be odd numbers
    let is_valid = (student_id >= 10000 && student_id <= 99999) && (student_id % 2 == 1);
    
    // Only commit the validation result, not the actual ID
    env::commit(&is_valid);
}