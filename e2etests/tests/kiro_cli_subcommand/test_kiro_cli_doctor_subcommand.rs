#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_doctor_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli doctor subcommand... | Description: Tests the <code> kiro-cli doctor </code> subcommand that debugs installation issues");

    println!("\n🔍 Executing 'kiro-cli doctor' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["doctor"])?;
    
    println!("📝 Doctor response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    assert!(response.contains("kiro-cli issue"), "Missing troubleshooting message");
    
    if response.contains("Everything looks good!") {
        println!("✅ Doctor check passed - everything looks good!");
    }
    
    println!("✅ Doctor subcommand output verified!");
    
    Ok(())
}
