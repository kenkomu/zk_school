use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Student ID (private data that won't be revealed)
    let student_id: u32 = 12345; // Example valid student ID
    // Uncomment the below line to test with an invalid ID
    // let student_id: u32 = 10000; // Even number, should be invalid

    println!("Generating proof for student ID verification...");
    
    // Create the executor environment with the private student ID
    let env = ExecutorEnv::builder()
        .write(&student_id)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover
    let prover = default_prover();

    // Generate the proof
    println!("Proving...");
    let prove_info = prover
        .prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)
        .unwrap();

    // Extract the receipt
    let receipt = prove_info.receipt;

    // Retrieve the result (true if ID is valid, false otherwise)
    let is_valid: bool = receipt.journal.decode().unwrap();
    
    // Display the result as requested
    println!("\n=== VERIFICATION RESULT ===");
    if is_valid {
        println!("✅ student");
    } else {
        println!("❌ not student");
    }
    println!("=========================\n");

    // Verify the receipt
    println!("Verifying the proof...");
    receipt
        .verify(GUEST_CODE_FOR_ZK_PROOF_ID)
        .unwrap();
        
    println!("The proof has been verified successfully!");
    println!("Note: The actual student ID was never revealed during this process.");
}