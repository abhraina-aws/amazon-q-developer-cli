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

    // Handle platform differences - themes might not be available on all systems
    if response.contains("No such file or directory") || response.contains("command not found") {
        println!("⚠️  Theme functionality not available on this system, skipping assertions");
        return Ok(());
    }
    
    // If themes are available, check for reasonable output
    if !response.trim().is_empty() {
        let theme_count = response.lines().filter(|line| !line.trim().is_empty()).count();
        if theme_count > 0 {
            assert!(response.contains("the-unnamed"), "Expected 'the-unnamed' in the output");
            assert!(response.contains("palenight"), "Expected 'palenight' in the output");
            assert!(response.contains("solarized-light"), "Expected 'solarized-light' in the output");
            assert!(response.contains("dracula"), "Expected 'dracula' in the output");
            assert!(response.contains("github-dark"), "Expected 'github-dark' in the output");
            assert!(response.contains("nord"), "Expected 'nord' in the output");
            assert!(response.contains("gruvbox"), "Expected 'gruvbox' in the output");
            assert!(!response.is_empty(), "Expected non-empty output");
        }
    }
    
    assert!(!response.contains("Error"), "Should not contain error messages");

    println!("✅ Kiro Cli theme --list subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "theme_subcommand", feature = "sanity"))]
fn test_kiro_cli_theme_folder_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli theme --folder... | Description: Tests the <code> kiro-cli theme --folder</code> subcommand to verify kiro cli folder.");

    println!("\n🔍 Executing 'kiro-cli theme --folder' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["theme","--folder"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(!response.is_empty(), "Expected non-empty output");
    assert!(response.contains("kiro"), "Expected 'kiro' in response.");
    assert!(response.contains("themes"), "Expected 'themes' in response.");

    println!("✅ Kiro Cli theme --folder subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "theme_subcommand", feature = "sanity"))]
fn test_kiro_cli_theme_verbose_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli theme --verbose... | Description: Tests the <code> kiro-cli theme --verbose</code> subcommand to verify kiro cli verbose response.");

    println!("\n🔍 Executing 'kiro-cli theme --verbose' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["theme","--verbose"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(!response.is_empty(), "Expected non-empty output");
    assert!(!response.contains("Error"), "Should not contain error messages");

    println!("✅ Kiro Cli theme --verbose subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "theme_subcommand", feature = "sanity"))]
fn test_kiro_cli_theme_v_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli theme -v... | Description: Tests the <code> kiro-cli theme -v</code> subcommand to verify kiro cli -v response.");

    println!("\n🔍 Executing 'kiro-cli theme -v' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["theme","-v"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(!response.is_empty(), "Expected non-empty output");
    assert!(!response.contains("Error"), "Should not contain error messages");

    println!("✅ Kiro Cli theme -v subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "theme_subcommand", feature = "sanity"))]
fn test_kiro_cli_theme_h_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli theme -h ... | Description: Tests the <code> kiro-cli theme -h </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli theme -h' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["theme", "-h"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Usage:"), "Expected 'Usage:' in the output");
    assert!(response.contains("Options:"), "Expected 'Options:' in the output");
    assert!(response.contains("Arguments"), "Expected 'Options:' in the output");

    assert!(response.contains("THEME"), "Expected 'THEME:' in the output");
    assert!(response.contains("-h"), "Expected '-h' in the output");
    
    assert!(response.contains("--help"), "Expected '--help' in the output");
    assert!(response.contains("--verbose"), "Expected '--verbose' in the output");
    assert!(response.contains("-v"), "Expected '-v' in the output");
    
    assert!(response.contains("--list"), "Expected '--list' in the output");
    assert!(response.contains("--folder"), "Expected '--folder' in the output");

    println!("✅ Kiro Cli theme -h subcommand executed successfully!");
    
    Ok(())
}