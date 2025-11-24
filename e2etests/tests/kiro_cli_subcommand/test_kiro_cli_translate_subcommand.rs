#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_translate_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli translate subcommand... | Description: Tests the <code> kiro-cli translate </code> subcommand for Natural Language to Shell translation");
    
    println!("\n🔍 Executing 'kiro-cli translate' subcommand with input 'hello'...");
    
    // Use stdin function for translate subcommand
    let response = q_chat_helper::execute_q_subcommand_with_stdin("kiro-cli", &["translate"], Some("Create a project directory named demoproject."))?;
    
    println!("📝 Translate response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify translation output contains shell subcommand
    assert!(response.contains("Shell"), "Missing shell subcommand in translation");
    assert!(response.contains("mkdir"), "Missing mkdir command");
    assert!(response.contains("demoproject"), "Missing demoproject name");

    // now I want to delete the demoproject directory
    q_chat_helper::execute_q_subcommand_with_stdin("kiro-cli", &["translate"], Some("Delete the demoproject directory."))?;

    println!("📝 Translate response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify translation output contains shell subcommand
    assert!(response.contains("Shell"), "Missing shell subcommand in translation");
    assert!(response.contains("rm -rf "), "Missing rm -rf command");
    assert!(response.contains("demoproject"), "Missing demoproject name");
    assert!(response.contains("Warning"), "Missing Warning message");
    
    println!("✅ Translate subcommand executed successfully!");
    
    Ok(())
}
