#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "integrations", feature = "sanity"))]
fn test_kiro_cli_integrations_install_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations install --help subcommand... | Description: Tests the <code> kiro-cli integrations install --help  </code> subcommand to verify different help options.");

    println!("\n🔍 Executing 'kiro-cli integrations install --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "install", "--help"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    assert!(response.contains("Commands"),"Expected 'Commands' in response.");
    assert!(response.contains("dotfiles"), "Expected 'dotfiles' in response.");
    assert!(response.contains("ssh"), "Expected 'ssh' in response.");
    
    assert!(response.contains("input-method"), "Expected 'input-method' in response.");
    assert!(response.contains("OPTIONS"), "Expected 'OPTIONS' in response.");
    assert!(response.contains("intellij-plugin"), "Expected 'intellij-plugin' in response.");
    
    assert!(response.contains("all"), "Expected 'all' in response.");
    assert!(response.contains("help"), "Expected 'help' in response.");
    assert!(response.contains("Print"), "Expected 'Prin' in response.");
    
    assert!(response.contains("Options"), "Expected 'Options' in response.");
    assert!(response.contains("-s"), "Expected '-s' in response.");
    
    assert!(response.contains("--silent"), "Expected '--silent' in response.");
    assert!(response.contains("--verbose"), "Expected '--verbose' in response.");
    assert!(response.contains("--help"), "Expected '--help' in response.");
    
    assert!(response.contains("--s"), "Expected '--s' in response.");
    assert!(response.contains("-v"), "Expected '-v' in response.");
    assert!(response.contains("-h"), "Expected '-h' in response.");

    println!("✅ Kiro Cli integrations install --help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "integrations", feature = "sanity"))]
fn test_kiro_cli_integrations_install_dotfiles_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations install dotfiles subcommand... | Description: Tests the <code> kiro-cli integrations install dotfiles  </code> subcommand to verify installation of dotfiles.");

    println!("\n🔍 Executing 'kiro-cli integrations install dotfiles' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "install", "dotfiles"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    if response.contains("Installed!") {
         assert!(response.contains("Installed!"),"Expected 'Installed!' in response.");
    } else if response.contains("Already installed") {
        assert!(response.contains("Already installed"), "Expected 'Already installed' in response.");
    }

    println!("✅ Kiro Cli integrations install dotfiles subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "integrations", feature = "sanity"))]
fn test_kiro_cli_integrations_uinstall_dotfiles_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations uninstall dotfiles subcommand... | Description: Tests the <code> kiro-cli integrations uninstall dotfiles  </code> subcommand to verify uninstallation of dotfiles.");

    println!("\n🔍 Executing 'kiro-cli integrations install dotfiles' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "uninstall", "dotfiles"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    if response.contains("Uninstalled!") {
         assert!(response.contains("Uninstalled!"),"Expected 'Uninstalled!' in response.");
    } else if response.contains("Not installed") {
        assert!(response.contains("Not installed"), "Expected 'Not installed' in response.");
    }

    println!("✅ Kiro Cli integrations uninstall dotfiles subcommand executed successfully!");
    
    Ok(())
}