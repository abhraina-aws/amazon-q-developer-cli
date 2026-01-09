#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "sub_integrations", feature = "sanity"))]
fn test_kiro_cli_sub_integrations_install_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
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
#[cfg(all(feature = "sub_integrations", feature = "sanity"))]
fn test_kiro_cli_sub_integrations_install_dotfiles_subcommand() -> Result<(), Box<dyn std::error::Error>> {
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
#[cfg(all(feature = "sub_integrations", feature = "sanity"))]
fn test_kiro_cli_sub_integrations_uinstall_dotfiles_subcommand() -> Result<(), Box<dyn std::error::Error>> {
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


#[test]
#[cfg(all(feature = "sub_integrations", feature = "sanity"))]
fn test_kiro_cli_sub_integrations_install_ssh_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations install ssh subcommand... | Description: Tests the <code> kiro-cli integrations install ssh  </code> subcommand to verify installation of ssh.");

    println!("\n🔍 Executing 'kiro-cli integrations install ssh' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "install", "ssh"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    if response.contains("Installed!!") {
         assert!(response.contains("Installed!"),"Expected 'Installed!' in response.");
    } else if response.contains("Already installed") {
        assert!(response.contains("Already installed"), "Expected 'Already installed' in response.");
    }

    println!("✅ Kiro Cli integrations install ssh subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "sub_integrations", feature = "sanity"))]
fn test_kiro_cli_sub_integrations_uninstall_ssh_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations uninstall ssh subcommand... | Description: Tests the <code> kiro-cli integrations uninstall ssh  </code> subcommand to verify uninstallation of ssh.");

    println!("\n🔍 Executing 'kiro-cli integrations install ssh' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "uninstall", "ssh"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    if response.contains("Uninstalled!") {
         assert!(response.contains("Uninstalled!"),"Expected 'Uninstalled!' in response.");
    } else if response.contains("Not installed") {
        assert!(response.contains("Not installed"), "Expected 'Not installed' in response.");
    }

    println!("✅ Kiro Cli integrations uninstall ssh subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "sub_integrations", feature = "sanity"))]
fn test_kiro_cli_sub_integrations_install_vscode_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations install vscode subcommand... | Description: Tests the <code> kiro-cli integrations install vscode  </code> subcommand to verify installation of vscode.");

    println!("\n🔍 Executing 'kiro-cli integrations install ssh' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "install", "vscode"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    if response.contains("Installed!") {
         assert!(response.contains("Installed!"),"Expected 'Installed!' in response.");
    }

    println!("✅ Kiro Cli integrations install vscode subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "sub_integrations", feature = "sanity"))]
fn test_kiro_cli_sub_integrations_uninstall_vscode_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations uninstall vscode subcommand... | Description: Tests the <code> kiro-cli integrations uninstall vscode  </code> subcommand to verify uninstallation of vscode.");

    println!("\n🔍 Executing 'kiro-cli integrations install vscode' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "uninstall", "vscode"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    if response.contains("error") {
         assert!(response.contains("VSCode"), "Expected 'VSCode!' in response.");
         assert!(response.contains("integration"), "Expected 'integration' in response.");
         assert!(response.contains("macOS"), "Expected 'macOS' in response.");
    } else if response.contains("Not installed") {    
        assert!(response.contains("Warning"),"Expected 'Warning' in response.");
        assert!(response.contains("VSCode"), "Expected 'VSCode' in response.");
        assert!(response.contains("automatically"), "Expected 'automatically' in response.");
    }


    println!("✅ Kiro Cli integrations install vscode subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "sub_integrations", feature = "sanity"))]
fn test_kiro_cli_sub_integrations_install_autostart_entry_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations install autostart-entry  subcommand... | Description: Tests the <code> kiro-cli integrations install autostart-entry   </code> subcommand to verify installation of autostart-entry .");

    println!("\n🔍 Executing 'kiro-cli integrations install autostart-entry ' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "install", "autostart-entry"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    if response.contains("error") {
        assert!(response.contains("error:"), "Expected 'error' in response.");
        assert!(response.contains("Installing"), "Expected 'Installing' in response.");
        assert!(response.contains("autostart"), "Expected 'autostart' in response.");
        assert!(response.contains("not supported"), "Expected 'not supported' in response.");
    }
    println!("✅ Kiro Cli integrations install autostart-entry subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "sub_integrations", feature = "sanity"))]
fn test_kiro_cli_sub_integrations_uninstall_autostart_entry_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli integrations uninstall autostart-entry  subcommand... | Description: Tests the <code> kiro-cli integrations uninstall autostart-entry   </code> subcommand to verify uninstallation of autostart-entry .");

    println!("\n🔍 Executing 'kiro-cli integrations uninstall autostart-entry' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["integrations", "uninstall", "autostart-entry"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    if response.contains("error") {
       
    } else if response.contains("error:") {
        assert!(response.contains("The autostart integration is only supported on Linux"), "Expected 'The autostart integration is only supported on Linux' in response.");
    }
    println!("✅ Kiro Cli integrations uninstall autostart-entry subcommand executed successfully!");
    
    Ok(())
}