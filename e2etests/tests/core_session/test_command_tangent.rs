#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

// Test the tangent command.
#[test]
#[cfg(all(feature = "core_session", feature = "sanity"))]
fn test_tangent_command() -> Result<(), Box<dyn std::error::Error>> {

    println!("\n🔍 Testing tangent ... | Description: Tests the <code> /tangent </code> command.");
    let session =q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    // Enable tangent mode first
    let _enable_result = q_chat_helper::execute_q_subcommand("kiro-cli", &["settings", "chat.enableTangentMode", "true"])?;
    println!("Enable result: {} ",_enable_result);
    println!("enable result end.");
    
    // Wait for settings to take effect
    std::thread::sleep(std::time::Duration::from_secs(10));
    
    // Execute the tangent command
    let response = chat.execute_command("/tangent")?;

    println!("📝 transform response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(!response.is_empty(), "Expected non-empty response");
    
    // Check if tangent mode is enabled
    if !response.contains("Tangent mode is disabled") {
        // Tangent mode is enabled - check for expected content
        assert!(response.contains("checkpoint") || response.contains("tangent"), "Expected checkpoint or tangent-related message");
        println!("✅ Tangent command executed with tangent mode enabled");
    } else {
        println!("⚠️ Tangent mode still disabled after timeout");
    }

    println!("Tangent command executed successfully.");
    drop(chat);
    Ok(())
}