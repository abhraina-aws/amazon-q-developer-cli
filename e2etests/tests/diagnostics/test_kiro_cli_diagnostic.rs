#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "diagnostics", feature = "sanity"))]
fn test_kiro_cli_diagnostics_help_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli diagnostics --help ... | Description: Tests the <code> kiro-cli diagnostics --help  </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli diagnostics --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["diagnostics","--help"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Options:"), "Expected 'Options:' in the output");
    assert!(response.contains("-f"), "Expected '-f' in the output");
    assert!(response.contains("--format"), "Expected '--format' in the output");
    assert!(response.contains("<FORMAT>"), "Expected '<FORMAT>' in the output");
    assert!(response.contains("plain"), "Expected 'plain' in the output");
    assert!(response.contains("json"), "Expected 'json' in the output");
    assert!(response.contains("json-pretty"), "Expected 'json-pretty' in the output");
    assert!(response.contains("--force"), "Expected '--force' in the output");

    assert!(response.contains("-v"), "Expected '-v' in the output");
    assert!(response.contains("--verbose"), "Expected '--verbose' in the output");

    assert!(response.contains("-h"), "Expected '-h' in the output");
    assert!(response.contains("--help"), "Expected '--help' in the output");

    println!("✅ Kiro Cli diagnostics --help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "diagnostics", feature = "sanity"))]
fn test_kiro_cli_diagnostics_plain_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli diagnostics --format plain ... | Description: Tests the <code> kiro-cli diagnostics --format plain  </code> subcommand to verify plain format.");

    println!("\n🔍 Executing 'kiro-cli diagnostics --format plain' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["diagnostics","--format", "plain"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("system-info"), "Expected 'system-info' in the output");
    assert!(response.contains("environment"), "Expected 'environment' in the output");
    assert!(response.contains("env-vars"), "Expected 'env-vars' in the output");

    println!("✅ Kiro Cli diagnostics --format plain subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "diagnostics", feature = "sanity"))]
fn test_kiro_cli_diagnostics_json_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli diagnostics --format json ... | Description: Tests the <code> kiro-cli diagnostics --format json  </code> subcommand to verify json format.");

    println!("\n🔍 Executing 'kiro-cli diagnostics --format json' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["diagnostics","--format", "json"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("{"), "Expected `{{` in the output");
    assert!(response.contains("}"), "Expected `}}`in the output");
    assert!(response.contains("env-vars"), "Expected 'env-vars' in the output");
    assert!(response.contains("system-info"), "Expected 'system-info' in the output");

    println!("✅ Kiro Cli diagnostics --format json subcommand executed successfully!");
    
    Ok(())
}