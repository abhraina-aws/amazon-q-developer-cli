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
fn test_kiro_global_steering() -> Result<(), Box<dyn std::error::Error>> {
    let original_dir = env::current_dir()?;
    let home_dir = PathBuf::from(env::var("HOME").or_else(|_| env::var("USERPROFILE"))?); 
    let steering_dir = home_dir.join(".kiro/steering");
    let steering_file = steering_dir.join("global_prompt.md");
    
    // Create .kiro/steering directory if it doesn't exist
    fs::create_dir_all(&steering_dir)?;
    
    // Create MD file with global prompt content
    let global_prompt_content = "# Global Prompt\n\nThis is a global steering prompt.";
    fs::write(&steering_file, global_prompt_content)?;
    
    // Change to home directory and execute /context show
    env::set_current_dir(&home_dir)?;
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap();
    
    println!("\n✅ Kiro cli Chat session started");
    let response = chat.execute_command_with_timeout("/context show", Some(1000))?;
    
    println!("📝 Global steering response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Check if global steering file is listed
    assert!(response.contains("steering/global_prompt.md"), "global_prompt file not found in steering folder");
    
    drop(chat);
    
    // Switch back to original directory
    env::set_current_dir(&original_dir)?;
    
    // Clean up - delete steering file
    fs::remove_file(&steering_file)?;
    
    Ok(())
}