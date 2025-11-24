#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_chat_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli chat subcommand... | Description: Tests the <code> kiro-cli chat </code> subcommand that opens kiro-cli terminal for interactive AI conversations.");
    
    println!("\n🔍 Executing 'kiro-cli chat' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["chat", "\"what is aws?\""])?;

    println!("📝 Chat response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Validate we got a proper AWS response
    assert!(response.contains("Amazon Web Services") || response.contains("AWS"), 
            "Response should contain AWS information");
    assert!(response.len() > 100, "Response should be substantial");
    
    println!("✅ kiro-cli chat subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli subcommand... | Description: Tests the <code> kiro-cli </code> subcommand that opens kiro-cli terminal for interactive AI conversations.");
    
    println!("\n🔍 Executing 'kiro-cli' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &[])?;

    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Validate we got help output
    //check if mcp present
    if response.contains("mcp") {
        assert!(response.contains("loaded"), "Expected 'loaded' in reponse");
        assert!(response.contains("in"), "Expected 'in' in reponse");
    }
    assert!(response.contains("Did you know"), "Expected 'Did you know' in reponse.");
    assert!(response.contains("Model"), "Expected 'Model' in reponse.");
    assert!(response.contains("Auto"), "Expected 'Auto' in reponse.");
    
    println!("✅ kiro-cli subcommand executed successfully!");
    
    Ok(())
}