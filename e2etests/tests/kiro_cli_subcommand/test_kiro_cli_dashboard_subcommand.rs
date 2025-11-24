#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_dashboard_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli dashboard subcommand... | Description: Tests the <code> kiro-cli dashboard </code> subcommand that open kiro-cli dashboard");

    println!("\n🔍 Executing 'kiro-cli dashboard' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["dashboard"])?;
    
    println!("📝 Response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    assert!(response.contains("Opening"), "Expected 'Opening' message in response");
    assert!(response.contains("Kiro CLI dashboard"), "Expected 'Kiro CLI dashboard' message in response");
    
    println!("✅ Kiro Cli dashboard executed successfully!");
    
    Ok(())
}
