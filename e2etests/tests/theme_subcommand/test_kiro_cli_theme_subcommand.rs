#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "theme_subcommand", feature = "sanity"))]
fn test_kiro_cli_theme_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli theme --help ... | Description: Tests the <code> kiro-cli theme --help </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli theme --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["theme", "--help"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Usage:"), "Expected 'Usage:' in the output");
    assert!(response.contains("Options:"), "Expected 'Options:' in the output");
    assert!(response.contains("-h"), "Expected '-h' in the output");
    assert!(response.contains("--help"), "Expected '--help' in the output");

    println!("✅ Kiro Cli theme --help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "theme_subcommand", feature = "sanity"))]
fn test_kiro_cli_theme_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli theme ... | Description: Tests the <code> kiro-cli theme </code> subcommand to verify current theme.");

    println!("\n🔍 Executing 'kiro-cli theme' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["theme"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(!response.is_empty(), "Expected non-empty output");
    assert!(!response.contains("Error"), "Should not contain error messages");

    println!("✅ Kiro Cli theme  subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "theme_subcommand", feature = "sanity"))]
fn test_kiro_cli_theme_list_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli theme --list ... | Description: Tests the <code> kiro-cli theme --list</code> subcommand to list all themes.");

    println!("\n🔍 Executing 'kiro-cli theme' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["theme","--list"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("the-unnamed"), "Expected 'the-unnamed' in the output");
    assert!(response.contains("palenight"), "Expected 'palenight' in the output");
    assert!(response.contains("solarized-light"), "Expected 'solarized-light' in the output");
    assert!(response.contains("dracula"), "Expected 'dracula' in the output");
    assert!(response.contains("github-dark"), "Expected 'github-dark' in the output");
    assert!(response.contains("nord"), "Expected 'nord' in the output");
    assert!(response.contains("gruvbox"), "Expected 'gruvbox' in the output");
    assert!(!response.is_empty(), "Expected non-empty output");
    assert!(!response.contains("Error"), "Should not contain error messages");

    println!("✅ Kiro Cli theme --list subcommand executed successfully!");
    
    Ok(())
}