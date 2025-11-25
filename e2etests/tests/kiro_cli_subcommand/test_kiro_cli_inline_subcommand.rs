#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline subcommand... | Description: Tests the <code> kiro-cli inline </code> subcommand for inline shell completion");   
    
    println!("\n🔍 Executing 'kiro-cli inline' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert that kiro-cli inline shows inline shell completions help
    assert!(response.contains("Inline shell completions"), "Response should contain 'Inline shell completions'");
    assert!(response.contains("enable"), "Response should show 'enable' command");
    assert!(response.contains("disable"), "Response should show 'disable' command");
    assert!(response.contains("status"), "Response should show 'status' command");
    
    println!("✅ kiro-cli inline subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline --help subcommand... | Description: Tests the <code> kiro-cli inline --help</code> subcommand for inline shell completion");   
    
    println!("\n🔍 Executing 'kiro-cli inline --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand_with_stdin("kiro-cli", &["inline"], Some("--help"))?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert that kiro-cli inline shows inline shell completions help
    assert!(response.contains("Inline shell completions"), "Response should contain 'Inline shell completions'");
    assert!(response.contains("enable"), "Response should show 'enable' command");
    assert!(response.contains("disable"), "Response should show 'disable' command");
    assert!(response.contains("status"), "Response should show 'status' command");
    
    println!("✅ kiro-cli inline help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_disable_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline disable subcommand... | Description: Tests the <code> kiro-cli inline disable</code> subcommand for disabling inline");   
    
    println!("\n🔍 Executing 'kiro-cli inline disable' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "disable"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert that kiro-cli inline disable shows success message
    assert!(response.contains("Inline disabled"), "Response should contain 'Inline disabled'");
    
    println!("✅ kiro-cli inline disable subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_disable_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline disable --help subcommand... | Description: Tests the <code> kiro-cli inline disable --help</code> subcommand to show help for disabling inline");   
    
    println!("\n🔍 Executing 'kiro-cli inline disable --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "disable", "--help"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("kiro-cli inline disable"), "Response should contain 'kiro-cli inline disable'");
    
    println!("✅ kiro-cli inline disable help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_enable_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline enable subcommand... | Description: Tests the <code> kiro-cli inline enable</code> subcommand for enabling inline");   
    
    println!("\n🔍 Executing 'kiro-cli inline enable' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "enable"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert that kiro-cli inline enable shows success message
    assert!(response.contains("Inline enabled"), "Response should contain 'Inline enabled'");
    
    println!("✅ kiro-cli inline enable subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_enable_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline enable --help subcommand... | Description: Tests the <code> kiro-cli inline enable --help</code> subcommand to show help for enabling inline");   
    
    println!("\n🔍 Executing 'kiro-cli inline enable --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "enable", "--help"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("kiro-cli inline enable"), "Response should contain 'kiro-cli inline enable'");
    
    println!("✅ kiro-cli inline enable help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_status_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline status subcommand... | Description: Tests the <code> kiro-cli inline status</code> subcommand for showing inline status");
    
    println!("\n🔍 Executing 'kiro-cli inline status' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "status"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert that kiro-cli inline status shows available customizations
    assert!(response.contains("Inline is enabled"), "Response should contain 'Inline is enabled'");

    println!("\n🔍 Executing 'kiro-cli setting all' subcommand to verify settings...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["setting", "all"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("inline.enabled"), "inline.enabled setting should be present in configuration");

    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["settings", "inline.enabled", "--delete"])?;

    assert!(response.contains("Removing") || response.contains("inline.enabled"), "Response should confirm deletion or non-existence of the setting");
    
    println!("✅ kiro-cli inline status subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_status_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline status --help subcommand... | Description: Tests the <code> kiro-cli inline status --help</code> subcommand to show help for inline status");   
    
    println!("\n🔍 Executing 'kiro-cli inline status --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "status", "--help"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("kiro-cli inline status"), "Response should contain 'kiro-cli inline status'");
    
    println!("✅ kiro-cli inline status help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_show_customizations_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline show-customizations subcommand... | Description: Tests the <code> kiro-cli inline show-customizations</code> that show the available customizations");   
    
    println!("\n🔍 Executing 'kiro-cli inline show-customizations' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "show-customizations"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert that kiro-cli inline show-customizations shows available customizations
    if response.contains("No customizations found") {
        assert!(response.contains("No customizations found"), "'No customizations found' message should be displayed");
    } else{
        assert!(response.contains("Amazon-Internal-V1"), "Response should contain 'Amazon-Internal-V1'");
        assert!(response.contains("Amazon-Aladdin-V1"), "Response should contain 'Amazon-Aladdin-V1'");
    }
    println!("✅ kiro-cli inline show-customizations subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_show_customizations_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline show-customizations --help subcommand... | Description: Tests the <code> kiro-cli inline show-customizations --help</code> to show help for showing customizations");   
    
    println!("\n🔍 Executing 'kiro-cli inline show-customizations --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "show-customizations", "--help"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert that kiro-cli inline show-customizations --help shows available customizations
    assert!(response.contains("kiro-cli inline show-customizations"), "Response should contain 'kiro-cli inline show-customizations'");
    
    println!("✅ kiro-cli inline show-customizations --help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_set_customization_subcommand() -> Result<(), Box<dyn std::error::Error>> {
   println!("\n🔍 Testing kiro-cli inline set-customization subcommand... | Description: Tests the <code> kiro-cli inline set-customization</code> interactive menu for selecting customizations");
    
    let response1 = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "set-customization"])?;

    println!("📝 Debug response: {} bytes", response1.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response1);
    println!("📝 END OUTPUT");
    
    if response1.contains("No customizations found") {
        println!("✅ No customizations available message printed");
        assert!(false  , "No customization available to set");

    } else {
        // Use helper function to select second option (Amazon-Internal-V1)
        let response = q_chat_helper::execute_interactive_menu_selection("kiro-cli", &["inline", "set-customization"], 1)?;
        
        println!("📝 Debug response: {} bytes", response.len());
        println!("📝 FULL OUTPUT:");
        println!("{}", response);
        println!("📝 END OUTPUT");
        
        // Just verify that the command executed (may select first option by default)
        assert!(response.contains("Customization")  && response.contains("selected"), "Should show selection confirmation");
        println!("✅ kiro-cli inline set-customization subcommand executed successfully!");
    }
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_unset_customization_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline unset customization... | Description: Tests the <code> kiro-cli inline set-customization</code> interactive menu for selecting 'None' to unset customization");
    
    // Get the interactive menu to find None position (always at last line)
    let menu_response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "set-customization"])?;
    //TODO : Fix logic none_index may not present
    let none_index = menu_response.lines().count();
    if menu_response.contains("No customizations found") {
        
        println!("📝 Debug response: {} bytes", menu_response.len());
        println!("📝 FULL OUTPUT:");
        println!("{}", menu_response);
        println!("📝 END OUTPUT");
        println!("✅ No customizations available message printed");

        assert!(false  , "Expected : 'None' option in interactive menu but got 'No customizations available' ");

    } else {
        println!("none_index={}", none_index);
        let response = q_chat_helper::execute_interactive_menu_selection("kiro-cli", &["inline", "set-customization"], none_index)?;
        
        println!("📝 Debug response: {} bytes", response.len());
        println!("📝 FULL OUTPUT:");
        println!("{}", response);
        println!("📝 END OUTPUT");
        
        // Verify that None was selected (customization unset)
        assert!(response.contains("Customization") && response.contains("unset"), "Should show None selection or unset confirmation");
        
        println!("✅ kiro-cli inline unset customization executed successfully!");
    }
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_inline_set_customization_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli inline set-customization --help subcommand... | Description: Tests the <code> kiro-cli inline set-customization --help</code> to show help for setting customizations");
    
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["inline", "set-customization", "--help"])?;
    
    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Assert that kiro-cli inline set-customization --help shows available customizations
    assert!(response.contains("kiro-cli inline set-customization"), "Response should contain 'set-customization'");
    
    println!("✅ kiro-cli inline set-customization --help subcommand executed successfully!");
    
    Ok(())
}

