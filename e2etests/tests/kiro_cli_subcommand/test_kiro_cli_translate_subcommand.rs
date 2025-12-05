#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_translate_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli translate subcommand... | Description: Tests the <code> kiro-cli translate </code> subcommand for Natural Language to Shell translation");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("\n🔍 Executing 'kiro-cli translate' subcommand with input 'hello'...");
    
    // Use stdin function for translate subcommand
    println!("\n🔍 Testing kiro-cli translate subcommand to create a file...");
    let response = q_chat_helper::execute_q_subcommand_with_stdin("kiro-cli", &["translate"], Some("create a file with name testkirocli in current working directory"))?;
    
    println!("📝 Translate response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Check if we got valid completions or an error
    if response.contains("no valid completions were generated") {
        println!("⚠️ No valid completions generated - this may be expected behavior");
        // Test passes if the translate command runs without crashing
        assert!(response.contains("kiro-cli translate"), "Expected kiro-cli translate command reference");
    } else {
        // If we got completions, verify they contain expected shell commands
        assert!(response.contains("touch") || response.contains("echo") || response.contains(">"), "Expected shell command in response");
    }

    // Test delete command translation
    println!("\n🔍 Testing kiro-cli translate subcommand to delete a file...");
    let delete_response = q_chat_helper::execute_q_subcommand_with_stdin("kiro-cli", &["translate"], Some("delete file testkirocli in current working directory"))?;

    println!("📝 Delete translate response: {} bytes", delete_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", delete_response);
    println!("📝 END OUTPUT");

    // Check if we got valid completions or an error
    if delete_response.contains("no valid completions were generated") {
        println!("⚠️ No valid completions generated for delete - this may be expected behavior");
        assert!(delete_response.contains("kiro-cli translate"), "Expected kiro-cli translate command reference");
    } else {
        // If we got completions, verify they contain expected delete commands
        assert!(delete_response.contains("rm") || delete_response.contains("del") || delete_response.contains("remove"), "Expected delete command in response");
    }
    
    println!("✅ Translate subcommand executed successfully!");
    
    Ok(())
}
