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
    q_chat_helper::execute_q_subcommand("kiro-cli", &["settings", "chat.enableTangentMode", "true"])?;
    let response = chat.execute_command("/tangent")?;

    println!("📝 transform response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(!response.is_empty(), "Expected non-empty response");
    assert!(response.contains("checkpoint"),"Missing conversation checkpoint message.");
    assert!(response.contains("/tangent"),"Missing /tangent command.");

    println!("Tangent command executed successfully.");
    drop(chat);
    Ok(())
}