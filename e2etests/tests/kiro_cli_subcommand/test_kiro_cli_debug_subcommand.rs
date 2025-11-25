#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_debug_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli debug subcommand... | Description: Tests the <code> kiro-cli debug </code> subcommand that provides debugging utilities for the app including app debugging, build switching, logs viewing, and various diagnostic tools.");
    
    println!("\n🔍 Executing 'kiro-cli debug' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["debug"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert debug help output contains expected commands
    assert!(response.contains("Debug the app"), "Response should contain debug description");
    assert!(response.contains("Commands"), "Response should list available commands");
    assert!(response.contains("app"), "Response should contain 'app' command");
    assert!(response.contains("build"), "Response should contain 'build' command");
    assert!(response.contains("logs"), "Response should contain 'logs' command");

    println!("✅ kiro-cli debug subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_debug_app_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli debug app subcommand... | Description: Tests the <code> kiro-cli debug app </code> subcommand that provides debugging utilities for the app including app debugging, build switching, logs viewing, and various diagnostic tools.");
    
    println!("\n🔍 Executing 'kiro cli debug app' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["debug", "app"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    if response.contains("app is only supported on macOS") {
        assert!(response.contains("app is only supported on macOS"), "Expected 'app is only supported on macOS' in reponse.");
    } else {
         // Assert that kiro-cli debug app launches the Amazon kiro-cli interface
    assert!(response.contains("Kiro CLI"), "Response should contain 'Kiro CLI'");
    assert!(response.contains("Running the Kiro CLI.app"), "Missing Running Kiro CLI confrmation");
    }

   
    
    println!("✅ kiro-cli debug app subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_debug_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli debug --help subcommand... | Description: Tests the <code> kiro-cli debug --help</code> subcommand to validate help output format and content.");
    
    println!("\n🔍 Executing 'kiro-cli debug --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["debug", "help"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert debug help output contains expected commands
    assert!(response.contains("Usage:") && response.contains("kiro-cli") && response.contains("[OPTIONS]") && response.contains("<COMMAND>"), 
            "Help should contain usage line");
    assert!(response.contains("Commands:"), "Response should list available commands");
    assert!(response.contains("app"), "Response should contain 'app' command");
    assert!(response.contains("build"), "Response should contain 'build' command");
    assert!(response.contains("logs"), "Response should contain 'logs' command");
    assert!(response.contains("Options:"), 
            "Help should contain Options section");
    assert!(response.contains("-v, --verbose"), 
            "Help should contain verbose option");
    assert!(response.contains("-h, --help"), 
            "Should contain help option");

    println!("✅ kiro-cli debug --help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_debug_build_help() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli build --help subcommand... | Description: Tests the <code> kiro-cli build --help </code> subcommand to validate help output format and available build options.");
    
    println!("\n🔍 Executing 'kiro-cli build --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["debug", "build", "--help"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert expected output
    assert!(response.contains("<APP>"), "Response should contain APP argument");
    assert!(response.contains("[BUILD]"), "Response should contain BUILD argument");
    assert!(response.contains("-v, --verbose...  Increase logging verbosity"), "Response should contain verbose option");
    assert!(response.contains("-h, --help        Print help"), "Response should contain help option");

    println!("✅ kiro-cli debug build --help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_debug_build_autocomplete() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli debug build autocomplete subcommand... | Description: Tests the <code> kiro-cli debug build autocomplete </code> subcommand to get current autocomplete build version.");
    
    println!("\n🔍 Executing 'kiro-cli debug build autocomplete' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["debug", "build", "autocomplete"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert expected output (should be either "production" or "beta")
    assert!(response.contains("production") || response.contains("beta"), "Response should contain either 'production' or 'beta'");

    println!("✅ kiro-cli debug build autocomplete subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_debug_build_dashboard() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli debug build dashboard subcommand... | Description: Tests the <code> kiro-cli debug build dashboard </code> subcommand to get current dashboard build version.");
    
    println!("\n🔍 Executing 'kiro-cli debug build dashboard' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["debug", "build", "dashboard"])?;

    println!("📝 Debug response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Assert expected output (should be either "production" or "beta")
    assert!(response.contains("production") || response.contains("beta"), "Response should contain either 'production' or 'beta'");

    println!("✅ kiro-cli debug build dashboard subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_debug_build_autocomplete_switch() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli debug build autocomplete switch functionality... | Description: Tests the <code> kiro-cli debug build autocomplete &lt;build&gt; </code> subcommand to switch between different autocomplete builds and revert back.");
    
    let builds = ["production", "beta"];
    
    // Get current build
    println!("\n🔍 Getting current build...");
    let current_response = q_chat_helper::execute_q_subcommand("kiro-cli", &["debug", "build", "autocomplete"])?;
    let current_build = current_response.split_whitespace().last().unwrap_or("production");

    println!("📝 Build response: {} bytes", current_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", current_response);
    println!("📝 END OUTPUT");    
    
    // Find any different build from the array
    let other_build = builds.iter().find(|&&b| b != current_build)
        .unwrap_or(&"beta"); // fallback to beta if current not found in array

    
    // Switch to other build
    println!("\n🔍 Switching to {} build...", other_build);
    let switch_response = q_chat_helper::execute_q_subcommand("kiro-cli", &["debug", "build", "autocomplete", other_build])?;

    println!("📝 Switch response: {} bytes", switch_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", switch_response);
    println!("📝 END OUTPUT");

    assert!(switch_response.contains("Kiro CLI"), "Expected output 'Kiro CLI' is missing in response");
    assert!(switch_response.contains(other_build), "Expected output '{}' is missing in response", other_build);
    assert!(switch_response.contains("autocomplete"), "Expected output 'autocomplete' is missing in response");

    // Switch back to original build
    println!("\n🔍 Switching back to {} build...", current_build);
    let revert_response = q_chat_helper::execute_q_subcommand("kiro-cli", &["debug", "build", "autocomplete", current_build])?;

    println!("📝 Switching back response: {} bytes", revert_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", revert_response);
    println!("📝 END OUTPUT");

    assert!(revert_response.contains("Kiro CLI"), "Expected output 'Kiro CLI' is missing in response");
    assert!(revert_response.contains(current_build), "Expected output '{}' is missing in response", current_build);
    assert!(revert_response.contains("autocomplete"), "Expected output 'autocomplete' is missing in response");

    println!("✅ Build switching test completed successfully!");
    
    Ok(())
}
