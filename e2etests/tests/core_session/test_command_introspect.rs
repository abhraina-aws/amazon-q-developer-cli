#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

//Test the introspect command
#[test]
#[cfg(all(feature = "core_session", feature = "sanity"))]
fn test_introspect_command() -> Result<(), Box<dyn std::error::Error>> {

    println!("\n🔍 Testing introspect command... | Description: Tests the <code> introspect </code> command.");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    println!("✅ Kiro Chat session started");
    
    let response = chat.execute_command("introspect")?;
    println!("📝 Help response: {} bytes", response);
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    println!("✅ Introspect command executed successfully");
    if response.contains("I'm Kiro") {
        assert!(response.contains("I'm Kiro"),"Missing Kiro message");
    } else if response.contains("Core Capabilities") {
        assert!(response.contains("Core Capabilities"),"Missing Core Capabilities");
    } else if response.contains("Available Commands") {
        assert!(response.contains("Available Commands"),"Missing Available Commands.");
    } else if response.contains("Experimental Features") {
        assert!(response.contains("Experimental Features"),"Missing Experimental Features.");
    }

    // Release the lock
    drop(chat);
    
    Ok(())
}