#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

/// Tests the kiro-cli restart subcommand
#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_restart_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli restart subcommand... | Description: Tests the <code> kiro-cli restart </code> subcommand to restart kiro-cli App.");
    
    println!("\n🛠️ Running 'kiro-cli restart' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["restart"])?;

    println!("📝 Restart response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Validate output contains expected restart messages
    assert!(response.contains("Restart") || response.contains("Launching"), "Should contain 'Restarting Kiro Cli' OR 'Launching Kiro Cli'");
    assert!(response.contains("Open"), "Should contain 'Opening Kiro cli dashboard'");
    
    println!("✅ Kiro Cli restart executed successfully!");
    
    Ok(())
}