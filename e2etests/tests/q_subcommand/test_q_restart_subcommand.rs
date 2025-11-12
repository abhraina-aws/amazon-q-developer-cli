#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

/// Tests the q restart subcommand
#[test]
#[cfg(all(feature = "q_subcommand", feature = "sanity"))]
fn test_q_restart_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro restart subcommand... | Description: Tests the <code> kiro restart </code> subcommand to restart Amazon Q.");
    
    println!("\n🛠️ Running 'kiro restart' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("q", &["restart"])?;

    println!("📝 Restart response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Validate output contains expected restart messages
    assert!(response.contains("Restart") || response.contains("Launching"), "Should contain 'Restarting Krio Cli' OR 'Launching Kiro Cli'");
    assert!(response.contains("Open"), "Should contain 'Opening Kiro cli dashboard'");
    
    println!("✅ Kiro Cli restart executed successfully!");
    
    Ok(())
}