#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

// Tests the /agent command without subcommands to display help information
//Verifies agent management description, usage, available subcommands, and options
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn agent_without_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent command... | Description: Tests the <code> /agent</code> command without subcommands to display help information. Verifies agent management description, usage, available subcommands, and options");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/agent",Some(1000))?;

    println!("📝 Agent response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Manage agents"), "Expected output 'Manage agents' is missing in response");
    assert!(response.contains("Usage"), "Expected output 'Usage' is missing in response");
    assert!(response.contains("/agent"), "Expected output '/agent' is missing in response");
    assert!(response.contains("<COMMAND>"), "Expected output '<COMMAND>' is missing in response");

    assert!(response.contains("Commands"), "Expected output 'Commands' is missing in response");
    assert!(response.contains("list"), "Expected output 'list' is missing in response");
    assert!(response.contains("create"), "Expected output 'create' is missing in response");
    assert!(response.contains("schema"), "Expected output 'schema' is missing in response");
    assert!(response.contains("set-default"), "Expected output 'set-default' is missing in response");
    assert!(response.contains("help"), "Expected output 'help' is missing in response");

    assert!(response.contains("List all available agents"), "Expected output 'List all available agents' is missing in response");
    assert!(response.contains("Create a new agent"), "Expected output 'Create a new agent' is missing in response");
    assert!(response.contains("Show agent config schema"), "Expected output 'Show agent config schema' is missing in response");
    assert!(response.contains("Define a default agent"), "Expected output 'Define a default agent' is missing in response");

    assert!(response.contains("Options"), "Expected output 'Options' is missing in response");
    assert!(response.contains("-h"), "Expected output '-h' is missing in response");
    assert!(response.contains("--help"), "Expected output '--help' is missing in response");

    println!("✅ /agent command executed successfully");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

/// Tests the /agent create command to create a new agent with specified name
/// Verifies agent creation process, file system operations, and cleanup
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_create_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent create --name <agent_name> command... | Description: Tests the <code> /agent create</code> command to create a new agent with specified name. Verifies agent creation process, file system operations, and cleanup");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let agent_name = format!("test_demo_agent_{}", timestamp);

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let create_response = chat.execute_command_with_timeout(&format!("/agent create --name {}", agent_name),Some(1000))?;

    println!("📝 Agent create response: {} bytes", create_response.len());
    println!("📝 CREATE RESPONSE:");
    println!("{}", create_response);
    println!("📝 END CREATE RESPONSE");

    let save_response = chat.execute_command(":wq")?;

    println!("📝 Save response: {} bytes", save_response.len());
    println!("📝 SAVE RESPONSE:");
    println!("{}", save_response);
    println!("📝 END SAVE RESPONSE");

    assert!(save_response.contains("Agent") && save_response.contains(&agent_name) && save_response.contains("has been created successfully"), "Expected output 'Agent has been created successfully' is missing in response");

    let whoami_response = chat.execute_command_with_timeout("!whoami",Some(1000))?;

    let lines: Vec<&str> = whoami_response.lines().collect();
    let username = lines.iter()
        .find(|line| !line.starts_with("!") && !line.starts_with(">") && !line.trim().is_empty())
        .expect("Expected output 'username' is missing in whoami response")
        .trim();

    let agent_path = format!("/Users/{}/.kiro/agents/{}.json", username, agent_name);

    if std::path::Path::new(&agent_path).exists() {
        std::fs::remove_file(&agent_path)?;
    } else {
        println!("⚠️ Agent file not found at: {}", agent_path);
    }

    assert!(!std::path::Path::new(&agent_path).exists(), "Expected output 'agent file deletion' is missing");

    println!("✅ Agent create command executed successfully");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

/// Tests the /agent edit command to edit a existing agent with specified name
/// Verifies agent edit process, file system operations, and cleanup
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_edit_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent edit --name <agent_name> command... | Description: Tests the <code> /agent edit</code> command to edit a existing agent. Verifies agent edit process, file system operations, and cleanup");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let agent_name = format!("test_demo_agent_{}", timestamp);

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    chat.execute_command_with_timeout(&format!("/agent create --name {}", agent_name),Some(1000))?;

    let save_response = chat.execute_command(":wq")?;

    assert!(save_response.contains("Agent") && save_response.contains(&agent_name) && save_response.contains("has been created successfully"), "Expected output 'Agent has been created successfully' is missing in response");

    // Edit the agent description
    let edit_response = chat.execute_command_with_timeout(&format!("/agent edit --name {}", agent_name),Some(2000))?;

    println!("📝 Agent edit response: {} bytes", edit_response.len());
    println!("📝 EDIT RESPONSE:");
    println!("{}", edit_response);
    println!("📝 END EDIT RESPONSE");


    // Use line-based editing
    chat.execute_command("/description")?; // Search for description line
    chat.execute_command("S")?; // Delete line and enter insert mode
    chat.execute_command("  \"description\": \"Updated agent description for testing\",")?;
    chat.execute_command("\u{1b}")?; // ESC

    let save_edit = chat.execute_command(":wq")?;

    println!("📝 Edit save response: {} bytes", save_edit.len());
    println!("📝 EDIT SAVE RESPONSE:");
    println!("{}", save_edit);
    println!("📝 END EDIT SAVE RESPONSE");

    assert!(save_edit.contains("Agent") && save_edit.contains(&agent_name) && save_edit.contains("has been edited successfully"), "Expected output 'has been edited successfully' is missing in response");

    let whoami_response = chat.execute_command_with_timeout("!whoami",Some(500))?;

    let lines: Vec<&str> = whoami_response.lines().collect();
    let username = lines.iter()
        .find(|line| !line.starts_with("!") && !line.starts_with(">") && !line.trim().is_empty())
        .expect("Expected output 'username' is missing in whoami response")
        .trim();
    println!("✅ Current username: {}", username);

    let agent_path = format!("/Users/{}/.kiro/agents/{}.json", username, agent_name);
    println!("✅ Agent path: {}", agent_path);

    if std::path::Path::new(&agent_path).exists() {
        std::fs::remove_file(&agent_path)?;
    } else {
        println!("⚠️ Agent file not found at: {}", agent_path);
    }

    assert!(!std::path::Path::new(&agent_path).exists(), "Agent file should be deleted");
    println!("✅ Agent deletion verified");

    //Release the lock before cleanup
    drop(chat);

    Ok(())
}
/// Tests the /agent create command without required arguments to verify error handling
/// Verifies proper error messages, usage information, and help suggestions
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_create_missing_args() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent create without required arguments... | Description: Tests the <code> /agent create</code> command without required arguments to verify error handling. Verifies proper error messages, usage information, and help suggestions");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/agent create",Some(2000))?;

    println!("📝 Agent create missing args response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("error"), "Expected output 'error' is missing in response");
    assert!(response.contains("the following required arguments"), "Expected output 'the following required arguments' is missing in response");
    assert!(response.contains("were not provided:"), "Expected output 'were not provided:' is missing in response");
    assert!(response.contains("--name"), "Expected output '--name' is missing in response");
    assert!(response.contains("<NAME>"), "Expected output '<NAME>' is missing in response");

    assert!(response.contains("Usage"), "Expected output 'Usage' is missing in response");
    assert!(response.contains("/agent create"), "Expected output '/agent create' is missing in response");
    assert!(response.contains("--name <NAME>"), "Expected output '--name <NAME>' is missing in response");

    assert!(response.contains("For more information"), "Expected output 'For more information' is missing in response");
    assert!(response.contains("try"), "Expected output 'try' is missing in response");

    assert!(response.contains("Options"), "Expected output 'Options' is missing in response");
    assert!(response.contains("<NAME>"), "Expected output '<NAME>' is missing in response");
    assert!(response.contains("Name of the agent to be created"), "Expected output 'Name of the agent to be created' is missing in response");
    assert!(response.contains("<DIRECTORY>"), "Expected output '<DIRECTORY>' is missing in response");
    assert!(response.contains("<FROM>"), "Expected output '<FROM>' is missing in response");

    println!("✅ /agent create executed successfully with expected error for missing arguments");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

/// Tests the /agent help command to display comprehensive agent help information
/// Verifies agent descriptions, usage notes, launch instructions, and configuration paths
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent help... | Description: Tests the <code> /agent help</code> command to display comprehensive agent help information. Verifies agent descriptions, usage notes, launch instructions, and configuration paths");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/agent help",Some(1000))?;

    println!("📝 Agent help command response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("~/.kiro/agents/"), "Expected output '~/.kiro/agents/' is missing in response");
    assert!(response.contains("cwd/.kiro/agents"), "Expected output 'cwd/.kiro/agents' is missing in response");
    assert!(response.contains("Usage"), "Expected output 'Usage' is missing in response");
    assert!(response.contains("/agent"), "Expected output '/agent' is missing in response");
    assert!(response.contains("<COMMAND>"), "Expected output '<COMMAND>' is missing in response");
    assert!(response.contains("Commands:"), "Expected output 'Commands:' is missing in response");
    assert!(response.contains("list"), "Expected output 'list' is missing in response");
    assert!(response.contains("create"), "Expected output 'create' is missing in response");
    assert!(response.contains("schema"), "Expected output 'schema' is missing in response");
    assert!(response.contains("set-default"), "Expected output 'set-default' is missing in response");
    assert!(response.contains("help"), "Expected output 'help' is missing in response");

    assert!(response.contains("Options"), "Expected output 'Options' is missing in response");
    assert!(response.contains("-h"), "Expected output '-h' is missing in response");
    assert!(response.contains("--help"), "Expected output '--help' is missing in response");

    println!("✅ /agent help executed successfully");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

/// Tests the /agent command with invalid subcommand to verify error handling
/// Verifies that invalid commands display help information with available commands and options
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_invalid_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent invalidcommand... | Description: Tests the <code> /agent</code> command with invalid subcommand to verify error handling. Verifies that invalid commands display help information with available commands and options");

    let session =q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/agent invalidcommand",Some(1000))?;

    println!("📝 Agent invalid command response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Commands"), "Expected output 'Commands' is missing in response");
    assert!(response.contains("list"), "Expected output 'list' is missing in response");
    assert!(response.contains("create"), "Expected output 'create' is missing in response");
    assert!(response.contains("schema"), "Expected output 'schema' is missing in response");
    assert!(response.contains("set-default"), "Expected output 'set-default' is missing in response");
    assert!(response.contains("help"), "Expected output 'help' is missing in response");
    assert!(response.contains("Options"), "Expected output 'Options' is missing in response");

    println!("✅ /agent invalidcommand executed successfully with expected error");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

/// Tests the /agent list command to display all available agents
/// Verifies agent listing format and presence of default agent
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_list_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent list command... | Description: Tests the <code> /agent list</code> command to display all available agents. Verifies agent listing format and presence of default agent");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/agent list",Some(1000))?;

    println!("📝 Agent list response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("kiro_default"), "Expected output 'kiro_default' is missing in response");

    assert!(response.contains("* kiro_default"), "Expected output '* kiro_default' is missing in response");

    println!("✅ /agent list command executed successfully");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

/// Tests the /agent set-default command with valid arguments to set default agent
/// Verifies success messages and confirmation of default agent configuration
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_set_default_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent set-default with valid arguments... | Description: Tests the <code> /agent set-default</code> command with valid arguments to set default agent. Verifies success messages and confirmation of default agent configuration");

    let session =q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let _ = chat.execute_command("clear");
    let _ = chat.execute_command("\x0C");

    let response = chat.execute_command_with_timeout("/agent set-default -n kiro_default",Some(1000))?;

    println!("📝 Agent set-default command response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("✓"), "Expected output '✓' is missing in response");
    assert!(response.contains("Default agent set to"), "Expected output 'Default agent set to' is missing in response");
    assert!(response.contains("kiro_default"), "Expected output 'kiro_default' is missing in response");
    assert!(response.contains("This will take effect"), "Expected output 'This will take effect' is missing in response");
    assert!(response.contains("next time kiro-cli chat is launched"), "Expected output 'next time kiro-cli chat is launched' is missing in response");

    println!("✅ /agent set-default executed successfully with valid arguments");

    // Release the lock before cleanup
    drop(chat);


    Ok(())
}
// Tests the /agent schema command to display agent configuration schema
// Verifies JSON schema structure with required keys and properties
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_schema_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent schema... | Description: Tests the <code> /agent schema </code> command to display agent configuration schema. Verifies JSON schema structure with required keys and properties");

    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let schema_response = chat.execute_command_with_timeout("/agent schema",Some(1000))?;

    println!("📝 Agent schema response: {} bytes", schema_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", schema_response);
    println!("📝 END OUTPUT");

    assert!(schema_response.contains("$schema"), "Expected output '$schema' is missing in response");
    assert!(schema_response.contains("title"), "Expected output 'title' is missing in response");
    assert!(schema_response.contains("description"), "Expected output 'description' is missing in response");
    assert!(schema_response.contains("type"), "Expected output 'type' is missing in response");
    assert!(schema_response.contains("properties"), "Expected output 'properties' is missing in response");
    assert!(schema_response.contains("name"), "Expected output 'name' is missing in response");

    println!("✅ /agent schema executed successfully with valid JSON schema");

    drop(chat);
    Ok(())
}

/// Tests the /agent set-default command without required arguments to verify error handling
/// Verifies error messages, usage information, and available options display
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_set_default_missing_args() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent set-default without required arguments... | Description: Tests the <code> /agent set-default</code> command without required arguments to verify error handling. Verifies error messages, usage information, and available options display");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let response = chat.execute_command_with_timeout("/agent set-default",Some(2000))?;

    println!("📝 Agent set-default missing args response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("error"), "Expected output 'error' is missing in response");
    assert!(response.contains("the following required arguments were not provided:"), "Expected output 'the following required arguments were not provided:' is missing in response");
    assert!(response.contains("--name <NAME>"), "Expected output '--name <NAME>' is missing in response");
    assert!(response.contains("Usage"), "Expected output 'Usage' is missing in response");
    assert!(response.contains("/agent"), "Expected output '/agent' is missing in response");
    assert!(response.contains("set-default"), "Expected output 'set-default' is missing in response");
    assert!(response.contains("--name"), "Expected output '--name' is missing in response");
    assert!(response.contains("For more information"), "Expected output 'For more information' is missing in response");
    assert!(response.contains("--help"), "Expected output '--help' is missing in response");
    assert!(response.contains("Options"), "Expected output 'Options' is missing in response");
    assert!(response.contains("-n"), "Expected output '-n' is missing in response");
    assert!(response.contains("<NAME>"), "Expected output '<NAME>' is missing in response");
    assert!(response.contains("-h"), "Expected output '-h' is missing in response");
    assert!(response.contains("Print help"), "Expected output 'Print help' is missing in response");

    println!("✅ /agent set-default executed successfully with expected error for missing arguments");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

/// Tests the /agent generate command to generate agent responses
/// Verifies agent generation process and response validation
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_generate_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent generate command... | Description: Tests the <code> /agent generate</code>command with vi editor interaction");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    // Clear any previous session output to prevent contamination
    let _ = chat.execute_command("clear");
    // Start the command and wait for name prompt
    let response = chat.execute_command_with_timeout("/agent generate", Some(20000))?;

    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Wait longer for the prompt to fully appear
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Enter agent name
    chat.send_key_input("test-agent\r")?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Enter description
    chat.send_key_input("Test agent description\r")?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Select scope (Enter for default)
    chat.send_key_input("\r")?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Wait for MCP menu, then confirm (Enter)
    let _final_response = chat.send_key_input("\r")?;

    println!("📝 FULL OUTPUT:");
    println!("{}", _final_response);
    println!("📝 END OUTPUT");
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Handle vi editor opening - enter insert mode and add content
    chat.send_key_input("i")?; // Enter insert mode
   // chat.send_key_input("Test system instructions for the agent")?;
    chat.send_key_input("\u{1b}")?; // ESC to exit insert mode

    std::thread::sleep(std::time::Duration::from_secs(3));

    // Get final response
    let final_response = chat.execute_command(":wq")?;
    println!("📝 Final response: {}", final_response);

    assert!(
        final_response.contains("has been created and saved successfully") ||
            final_response.contains("Generating agent config") ||
            final_response.contains("Agent 'test-agent'"),
        "Expected output 'agent creation confirmation' is missing in response"
    );
    println!("✅ /agent generate executed successfully with expected response");
    drop(chat);
    Ok(())

}

// Tests the /agent swap command to swap the agents
// Verifies agent swap process and response validation
#[test]
#[cfg(all(feature = "agent", feature = "sanity"))]
fn test_agent_swap_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /agent swap command... | Description: Tests the <code> /agent swap</code>command.");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
 // Clear any previous session output to prevent contamination
    let _ = chat.execute_command("clear");
    // Start the command and wait for name prompt
    let _response1 = chat.execute_command_with_timeout("/agent swap",Some(2000))?;
    println!("📝 Agent swap response: {} bytes", _response1.len());
    println!("📝 Full output: {}", _response1);
    println!("📝 End output");
    let _response2 = chat.execute_command_with_timeout("1",Some(1000))?;
    println!("📝 Agent swap response Full output : {}", _response2);

    assert!(
        _response2.contains("✓") || _response2.contains("Choose one of the following agents"),
        "Expected output 'agent swap confirmation' is missing in response"
    );
    println!("✅ /agent swap executed successfully with expected response");
    drop(chat);
    Ok(())
}