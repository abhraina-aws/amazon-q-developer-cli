#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;
use std::fs;
use std::path::PathBuf;

#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_prompts_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /prompts command... | Description: Tests the <code> /prompts</code> command to display available prompts with usage instructions and argument requirements");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/prompts",Some(2000))?;

    println!("📝 Prompts command response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify usage instruction
    assert!(response.contains("Usage"),"Missing Usage instruction");
    assert!(response.contains("@"),"Missing @");
    assert!(response.contains("<prompt name>"),"Missing <prompt name>");
    assert!(response.contains("[...args]"),"Missing [...args]");
    
    println!("✅ Found usage instruction");

    // Verify table headers
    assert!(response.contains("Prompt"), "Missing Prompt header");
    assert!(response.contains("Arguments"), "Missing Arguments");
    assert!(response.contains("*"), "Missing *");
    assert!(response.contains("required"), "Missing required");

    // Verify command executed successfully
    assert!(!response.is_empty(), "Empty response from prompts command");

    println!("✅ All prompts command functionality verified!");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_prompts_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /prompts --help command... | Description: Tests the <code> /prompts --help</code> command to display comprehensive help information about prompts functionality and MCP server integration");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/prompts --help",Some(1000))?;

    println!("📝 Prompts help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify description
    assert!(response.contains("Prompts are reusable templates that help you quickly access common workflows and tasks"), "Missing prompts description");
    assert!(response.contains("These templates are provided by the mcp servers you have installed and configured"), "Missing MCP servers description");

    assert!(response.contains("@"),"Missing @ syntax");
    assert!(response.contains("<prompt name> [arg]"), "Missing <prompt name> [arg] example");
    assert!(response.contains("[arg]"), "Missing [arg] example");
    assert!(response.contains("Retrieve prompt specified"), "Missing Retrieve prompt specified description");
    assert!(response.contains("/prompts"), "Missing /prompts");
    assert!(response.contains("get"), "Missing get");
    assert!(response.contains("<prompt name>"), "Missing <prompt name>");
    assert!(response.contains("[arg]"), "Missing [arg]");
 

    // Verify main description
    assert!(response.contains("View and retrieve prompts"), "Missing main description");
    assert!(response.contains("Usage"), "Missing Usage");
    assert!(response.contains("/prompts"), "Missing /prompts");
    assert!(response.contains("[COMMAND]"), "Missing [COMMAND]");
    assert!(response.contains("Commands"), "Missing Commands section");
    assert!(response.contains("list"), "Missing list command");
    assert!(response.contains("get"), "Missing get command");
    assert!(response.contains("help"), "Missing help command");
    assert!(response.contains("List available prompts from a tool or show all available prompt"), "Missing list description");

    // Verify Options section
    assert!(response.contains("Options"), "Missing Options section");
    assert!(response.contains("-h") && response.contains("--help"), "Missing help flags");

    println!("✅ All prompts help content verified!");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_prompts_list_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /prompts list command... | Description: Tests the <code> /prompts list</code> command to display all available prompts with their arguments and usage information");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/prompts list",Some(2000))?;

    println!("📝 Prompts list response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify usage instruction
    assert!(response.contains("Usage:"), "Missing Usage instruction");
    assert!(response.contains("@"), "Missing @");
    assert!(response.contains("<prompt name>"), "Missing <prompt name>");
    assert!(response.contains("[...args]"), "Missing [...args]");

    // Verify table headers
    assert!(response.contains("Prompt"), "Missing Prompt header");
    assert!(response.contains("Arguments"), "Missing Arguments");
    assert!(response.contains("*"), "Missing *");
    assert!(response.contains("required"), "Missing required");

    // Verify command executed successfully
    assert!(!response.is_empty(), "Empty response from prompts list command");

    println!("✅ All prompts list command functionality verified!");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}


#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_prompts_get_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /prompts get command... | Description: Tests the <code> /prompts get prompt_name</code> command to display all available prompts with their arguments and usage information");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    // First, check if any prompts exist
    let response = chat.execute_command_with_timeout("/prompts list",Some(2000))?;
    println!("📝 Prompts list response: {}", response);
    
    // Look for prompt names in the table output (skip header lines)
    let first_prompt_opt = response
        .lines()
        .skip_while(|line| !line.contains("▔") && !line.contains("─")) // Skip until we find the table separator
        .skip(1) // Skip the separator line itself
        .find(|line| {
            let trimmed = line.trim();
            // Skip empty lines, lines starting with >, lines with Usage, and lines that only contain special chars
            !trimmed.is_empty() 
                && !trimmed.starts_with(">") 
                && !line.contains("Usage:")
                && !trimmed.chars().all(|c| c == '▔' || c == '─' || c.is_whitespace())
                && trimmed.chars().any(|c| c.is_alphanumeric())
        })
        .and_then(|line| {
            // Extract the first word (prompt name) from the table row
            let first_word = line.trim().split_whitespace().next()?;
            // Validate it's a reasonable prompt name (alphanumeric with hyphens/underscores)
            if first_word.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                Some(first_word)
            } else {
                None
            }
        });

    let prompts_dir = PathBuf::from(".kiro/prompts");
    let test_prompt_path = prompts_dir.join("test-e2e-prompt.md");
    let mut created_prompt = false;
    let prompt_name: String;

    // If no prompts found, create one
    if first_prompt_opt.is_none() {
        println!("📝 No prompts found, creating temporary test prompt");
        fs::create_dir_all(&prompts_dir)?;
        let prompt_content = r#"---
name: test-e2e-prompt
---
What is AWS? Explain in 10 words.
"#;
        fs::write(&test_prompt_path, prompt_content)?;
        created_prompt = true;
        prompt_name = "test-e2e-prompt".to_string();
        println!("📝 Created temporary test prompt: {}", prompt_name);
        
        // Re-run list command to verify prompt was created
        let new_response = chat.execute_command_with_timeout("/prompts list",Some(2000))?;
        println!("📝 Updated prompts list response: {}", new_response);
    } else {
        prompt_name = first_prompt_opt.unwrap().to_string();
        println!("📝 Found existing prompt: {}", prompt_name);
    }

    // Test the get command
    let get_response = chat.execute_command_with_timeout(&format!("/prompts get {}", prompt_name),Some(2000))?;
    println!("📝 Get response: {}", get_response);

    assert!(!get_response.is_empty(), "Prompt get command should return content");
    println!("✅ Prompt get command executed successfully");
    
    drop(chat);

    // Cleanup: Remove the test prompt if we created it
    if created_prompt && test_prompt_path.exists() {
        fs::remove_file(&test_prompt_path)?;
        println!("📝 Cleaned up temporary test prompt");
    }

    Ok(())
}

#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_create_prompt_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /prompts create --name promptname command... | Description: Tests the <code> /prompts create --name promptname </code>  command create a new local prompt");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/prompts create --name testprompt",Some(2000))?;
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // it will open vi editor so we need to add some prmppt then close it using :wp
    // Enter insert mode
    chat.send_key_input("i")?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Press enter to go to new line
    chat.send_key_input("\r")?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Add prompt content
    chat.send_key_input("This is a test prompt for e2e testing.")?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Exit insert mode 
    chat.send_key_input("\x1B")?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Save and exit vi editor
    let response = chat.send_key_input(":wq\r")?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("📝 Prompts list response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Created local prompt"), "Missing Created local prompt");
    assert!(response.contains("testprompt"), "Missing testprompt");
    assert!(response.contains("testprompt.md"), "Missing testprompt.md");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_prompts_details_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /prompts details <prompt> command... | Description: Tests the <code> /prompts details <prompt> </code> command to display detailed information about a specific prompt");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/prompts details testprompt",Some(2000))?;

    println!("📝 Prompts list response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Prompt Details"), "Missing Prompt Details");
    assert!(response.contains("Name"), "Missing Name");
    assert!(response.contains("Source"), "Missing Source");
    assert!(response.contains("Usage"), "Missing Usage");
    assert!(response.contains("Content Preview"), "Missing Content Preview");
    assert!(response.contains("testprompt"), "Missing testprompt");
    assert!(response.contains("This is a test prompt for e2e testing."), "Missing prompt content");

    println!("✅ All prompts details command functionality verified!");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "ai_prompts", feature = "sanity"))]
fn test_prompts_remove_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /prompts remove <prompt> command... | Description: Tests the <code> /prompts remove <prompt> </code> command remove an existing local prompt");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/prompts remove testprompt",Some(2000))?;

    println!("📝 Prompts list response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Warning"), "Missing Warning");
    assert!(response.contains("This will permanently remove the local"), "Missing This will permanently remove the local message");
    assert!(response.contains("testprompt"), "Missing testprompt");

    let response = chat.send_key_input("y\r")?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Removed local prompt"), "Missing Removed local prompt message");
    assert!(response.contains("successfully"), "Missing successfully message");
    assert!(response.contains("testprompt"), "Missing testprompt");

    println!("✅ All prompts remove command functionality verified!");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}
