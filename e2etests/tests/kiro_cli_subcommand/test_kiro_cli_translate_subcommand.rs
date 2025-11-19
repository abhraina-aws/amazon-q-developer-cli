#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_translate_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli translate subcommand... | Description: Tests the <code> kiro-cli translate </code> subcommand for Natural Language to Shell translation");
    
    println!("\n🔍 Executing 'kiro-cli translate' subcommand with input 'hello'...");
    
    // Use stdin function for translate subcommand
    let response = q_chat_helper::execute_q_subcommand_with_stdin("kiro-cli", &["translate"], Some("hello"))?;
    
    println!("📝 Translate response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify translation output contains shell subcommand
    assert!(response.contains("echo") || response.contains("Shell"), "Missing shell subcommand in translation");
    
    println!("✅ Translate subcommand executed successfully!");
    
    Ok(())
}
