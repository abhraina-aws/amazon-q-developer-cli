#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "q_subcommand", feature = "sanity"))]
fn test_q_chat_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro chat subcommand... | Description: Tests the <code> kiro chat </code> subcommand that opens Q terminal for interactive AI conversations.");
    
    println!("\n🔍 Executing 'kiro chat' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("q", &["chat", "\"what is aws?\""])?;

    println!("📝 Chat response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Validate we got a proper AWS response
    assert!(response.contains("Amazon Web Services") || response.contains("AWS"), 
            "Response should contain AWS information");
    assert!(response.len() > 100, "Response should be substantial");
    
    println!("✅ Got substantial AI response ({} bytes)!", response.len());

    println!("✅ Chat subcommand executed!");
    
    println!("✅ q chat subcommand executed successfully!");
    
    Ok(())
}