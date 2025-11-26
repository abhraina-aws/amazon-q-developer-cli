#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_integrations_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations --help subcommand... | Description: Tests the <code> kiro-cli integrations --help  </code> subcommand to verify different help commands.");

    println!("\n🔍 Executing 'kiro-cli integrations --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "--help"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    assert!(response.contains("Manage system integrations"),"Expected 'Manage system integrations' in response.");
    assert!(response.contains("Usage"), "Expected 'Usage' in response.");
    assert!(response.contains("kiro-cli"), "Expected 'kiro-cli' in response.");
    
    assert!(response.contains("integrations"), "Expected 'integrations' in response.");
    assert!(response.contains("OPTIONS"), "Expected 'OPTIONS' in response.");
    assert!(response.contains("COMMAND"), "Expected 'COMMAND' in response.");
    
    assert!(response.contains("Commands"), "Expected 'Commands' in response.");
    assert!(response.contains("install"), "Expected 'install' in response.");
    assert!(response.contains("uninstall"), "Expected 'uninstall' in response.");
    
    assert!(response.contains("reinstall"), "Expected 'reinstall' in response.");
    assert!(response.contains("status"), "Expected 'status' in response.");
    
    assert!(response.contains("help"), "Expected 'help' in response.");
    assert!(response.contains("verbose"), "Expected 'verbose' in response.");
    assert!(response.contains("--help"), "Expected '--help' in response.");
    
    assert!(response.contains("--verbose"), "Expected '--verbose' in response.");
    assert!(response.contains("-v"), "Expected '-v' in response.");
    assert!(response.contains("-h"), "Expected '-h' in response.");

    println!("✅ Kiro Cli integrations --help subcommand executed successfully!");
    
    Ok(())
}