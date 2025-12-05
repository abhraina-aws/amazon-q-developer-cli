use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "setup_subcommands", feature = "sanity"))]
fn test_kiro_cli_setup_help__subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli setup --dotfiles ... | Description: Tests the <code> kiro-cli setup --help  </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli setup --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["setup","--help"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Options:"), "Expected 'Options:' in the output");
    assert!(response.contains("dotfiles"), "Expected 'dotfiles' in the output");
    assert!(response.contains("input-method"), "Expected 'input-method' in the output");
    assert!(response.contains("no-confirm"), "Expected 'no-confirm' in the output");
    assert!(response.contains("force"), "Expected 'force' in the output");
    assert!(response.contains("global"), "Expected 'global' in the output");
    assert!(response.contains("verbose"), "Expected 'verbose' in the output");
    assert!(response.contains("help"), "Expected 'help' in the output");

    println!("✅ Kiro Cli setup --help subcommand executed successfully!");
    
    Ok(())
}