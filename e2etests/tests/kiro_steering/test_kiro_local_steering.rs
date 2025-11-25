#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::path::PathBuf;

#[test]
#[cfg(all(feature = "kiro_steering", feature = "sanity"))]
fn test_kiro_local_steering() -> Result<(), Box<dyn std::error::Error>> {
    let steering_dir = PathBuf::from(".kiro/steering");
    let steering_file = steering_dir.join("local_prompt.md");
    
    // Create .kiro/steering directory if it doesn't exist
    fs::create_dir_all(&steering_dir)?;
    
    // Create MD file with test prompt content
    let local_prompt_content = "# Test Prompt\n\nThis is a test steering prompt.";
    fs::write(&steering_file, local_prompt_content)?;

    // Execute /context show with new session to pick up steering files
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap();

    println!("\n✅ Kiro cli Chat session started");
    let response = chat.execute_command_with_timeout("/context show", Some(1000))?;

    println!("📝 MCP help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Check if steering file is listed
    assert!(response.contains("steering/local_prompt.md"), "local_prompt file not found steering folder");
    
    drop(chat);
    
    // Clean up - delete steering file
    fs::remove_file(&steering_file)?;
    
    Ok(())
}
