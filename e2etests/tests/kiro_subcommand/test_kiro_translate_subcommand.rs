#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_subcommand", feature = "sanity"))]
fn test_kiro_translate_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro translate subcommand... | Description: Tests the <code> kiro translate </code> subcommand for Natural Language to Shell translation");
    
    println!("\n🔍 Executing 'kiro translate' subcommand with input 'hello'...");
    
    // Use stdin function for translate subcommand
    let response = q_chat_helper::execute_q_subcommand_with_stdin("q", &["translate"], Some("hello"))?;
    
    println!("📝 Translate response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify translation output contains shell subcommand
    assert!(response.contains("echo") || response.contains("Shell"), "Missing shell subcommand in translation");
    println!("✅ Found shell subcommand translation");
    
    println!("✅ Translate subcommand executed successfully!");
    
    Ok(())
}
