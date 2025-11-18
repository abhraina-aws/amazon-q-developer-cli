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

    assert!(response.contains("Manage agents"), "Missing 'Manage agents' description");
    assert!(response.contains("Usage:"), "Missing usage information");
    assert!(response.contains("/agent"), "Missing agent command");
    assert!(response.contains("<COMMAND>"), "Missing command placeholder");
    println!("✅ Found agent command description and usage");

    assert!(response.contains("Commands:"), "Missing Commands section");
    assert!(response.contains("list"), "Missing list subcommand");
    assert!(response.contains("create"), "Missing create subcommand");
    assert!(response.contains("schema"), "Missing schema subcommand");
    assert!(response.contains("set-default"), "Missing set-default subcommand");
    assert!(response.contains("help"), "Missing help subcommand");
    println!("✅ Verified all agent subcommands: list, create, schema, set-default, help");

    assert!(response.contains("List all available agents"), "Missing list command description");
    assert!(response.contains("Create a new agent"), "Missing create command description");
    assert!(response.contains("Show agent config schema"), "Missing schema command description");
    assert!(response.contains("Define a default agent"), "Missing set-default command description");
    println!("✅ Verified command descriptions");

    assert!(response.contains("Options:"), "Missing Options section");
    assert!(response.contains("-h"), "Missing short help option");
    assert!(response.contains("--help"), "Missing long help option");
    println!("✅ Found options section with help flag");

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

    assert!(save_response.contains("Agent") && save_response.contains(&agent_name) && save_response.contains("has been created successfully"), "Missing agent creation success message");
    println!("✅ Found agent creation success message");

    let whoami_response = chat.execute_command_with_timeout("!whoami",Some(1000))?;

    println!("📝 Whoami response: {} bytes", whoami_response.len());
    println!("📝 WHOAMI RESPONSE:");
    println!("{}", whoami_response);
    println!("📝 END WHOAMI RESPONSE");

    let lines: Vec<&str> = whoami_response.lines().collect();
    let username = lines.iter()
        .find(|line| !line.starts_with("!") && !line.starts_with(">") && !line.trim().is_empty())
        .expect("Failed to get username from whoami command")
        .trim();
    println!("✅ Current username: {}", username);

    let agent_path = format!("/Users/{}/.kiro/agents/{}.json", username, agent_name);
    println!("✅ Agent path: {}", agent_path);

    if std::path::Path::new(&agent_path).exists() {
        std::fs::remove_file(&agent_path)?;
        println!("✅ Agent file deleted: {}", agent_path);
    } else {
        println!("⚠️ Agent file not found at: {}", agent_path);
    }

    assert!(!std::path::Path::new(&agent_path).exists(), "Agent file should be deleted");
    println!("✅ Agent deletion verified");

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


    assert!(save_response.contains("Agent") && save_response.contains(&agent_name) && save_response.contains("has been created successfully"), "Missing agent creation success message");
    println!("✅ Found agent creation success message");

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

    assert!(save_edit.contains("Agent") && save_edit.contains(&agent_name) && save_edit.contains("has been edited successfully"), "Missing agent update success message");
    println!("✅ Found agent update success message");

    let whoami_response = chat.execute_command_with_timeout("!whoami",Some(500))?;

    println!("📝 Whoami response: {} bytes", whoami_response.len());
    println!("📝 WHOAMI RESPONSE:");
    println!("{}", whoami_response);
    println!("📝 END WHOAMI RESPONSE");

    let lines: Vec<&str> = whoami_response.lines().collect();
    let username = lines.iter()
        .find(|line| !line.starts_with("!") && !line.starts_with(">") && !line.trim().is_empty())
        .expect("Failed to get username from whoami command")
        .trim();
    println!("✅ Current username: {}", username);

    let agent_path = format!("/Users/{}/.kiro/agents/{}.json", username, agent_name);
    println!("✅ Agent path: {}", agent_path);

    if std::path::Path::new(&agent_path).exists() {
        std::fs::remove_file(&agent_path)?;
        println!("✅ Agent file deleted: {}", agent_path);
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

    assert!(response.contains("error:"), "Missing error message part 1a");
    assert!(response.contains("the following required arguments"), "Missing error message part 1b");
    assert!(response.contains("were not provided:"), "Missing error message part 2");
    assert!(response.contains("--name"), "Missing required name argument part 1");
    assert!(response.contains("<NAME>"), "Missing required name argument part 2");
    println!("✅ Found error message for missing required arguments");

    assert!(response.contains("Usage:"), "Missing usage information part 1");
    assert!(response.contains("/agent create"), "Missing usage information part 2a");
    assert!(response.contains("--name <NAME>"), "Missing usage information part 2b");
    println!("✅ Found usage information");

    assert!(response.contains("For more information"), "Missing help suggestion part 1");
    assert!(response.contains("try"), "Missing help suggestion part 2a");
    println!("✅ Found help suggestion");

    assert!(response.contains("Options:"), "Missing options section");
    assert!(response.contains("<NAME>"), "Missing name option part 2");
    assert!(response.contains("Name of the agent to be created"), "Missing name description");
    assert!(response.contains("<DIRECTORY>"), "Missing directory option part 2");
    assert!(response.contains("<FROM>"), "Missing from option part 2");
    println!("✅ Found all expected options");

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

    assert!(response.contains("~/.kiro/agents/"), "Missing gloabal path(~/.kiro/agents/) path");
    assert!(response.contains("cwd/.kiro/agents"), "Missing workspace (cwd/.kiro/agents) path");
    assert!(response.contains("Usage:"), "Missing usage label");
    assert!(response.contains("/agent"), "Missing /agent command");
    assert!(response.contains("<COMMAND>"), "Missing command parameter");
    assert!(response.contains("Commands:"), "Missing commands section");
    assert!(response.contains("list"), "Missing list command");
    assert!(response.contains("create"), "Missing create command");
    assert!(response.contains("schema"), "Missing schema command");
    assert!(response.contains("set-default"), "Missing set-default command");
    assert!(response.contains("help"), "Missing help command");
    println!("✅ Found all expected commands in help output");

    assert!(response.contains("Options:"), "Missing options section");
    assert!(response.contains("-h"), "Missing short help flag");
    assert!(response.contains("--help"), "Missing long help flag");
    println!("✅ Found all expected options in help output");

    println!("✅ All expected help content found");
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

    assert!(response.contains("Commands:"), "Missing commands section");
    assert!(response.contains("list"), "Missing list command");
    assert!(response.contains("create"), "Missing create command");
    assert!(response.contains("schema"), "Missing schema command");
    assert!(response.contains("set-default"), "Missing set-default command");
    assert!(response.contains("help"), "Missing help command");
    println!("✅ Found all expected commands in help output");

    assert!(response.contains("Options:"), "Missing options section");
    println!("✅ Found options section");

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

    assert!(response.contains("kiro_default"), "Missing kiro_default agent");
    println!("✅ Found kiro_default agent in list");

    assert!(response.contains("* kiro_default"), "Missing bullet point format for kiro_default");
    println!("✅ Verified bullet point format for agent list");

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

    let mut failures = Vec::new();

    if !response.contains("✓") { failures.push("Missing success checkmark"); }
    if !response.contains("Default agent set to") { failures.push("Missing success message"); }
    if !response.contains("kiro_default") { failures.push("Missing agent name"); }
    if !response.contains("This will take effect") { failures.push("Missing effect message"); }
    if !response.contains("next time kiro-cli chat is launched") { failures.push("Missing launch message"); }

    if !failures.is_empty() {
        panic!("Test failures: {}", failures.join(", "));
    }

    println!("✅ All expected success messages found");
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

    assert!(schema_response.contains("$schema"), "Missing $schema key");
    assert!(schema_response.contains("title"), "Missing title key");
    assert!(schema_response.contains("description"), "Missing description key");
    assert!(schema_response.contains("type"), "Missing type key");
    assert!(schema_response.contains("properties"), "Missing properties key");
    assert!(schema_response.contains("name"), "Missing name key");

    println!("✅ Found all expected JSON schema keys and properties");
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

    let mut failures = Vec::new();

    if !response.contains("error") { failures.push("Missing error message"); }
    if !response.contains("the following required arguments were not provided:") { failures.push("Missing error message2"); }
    if !response.contains("--name <NAME>") { failures.push("Missing required name argument"); }
    if !response.contains("Usage:") { failures.push("Missing usage text"); }
    if !response.contains("/agent") { failures.push("Missing agent command"); }
    if !response.contains("set-default") { failures.push("Missing set-default subcommand"); }
    if !response.contains("--name") { failures.push("Missing name flag"); }
    if !response.contains("For more information") { failures.push("Missing help text"); }
    if !response.contains("--help") { failures.push("Missing help flag"); }
    if !response.contains("Options:") { failures.push("Missing options section"); }
    if !response.contains("-n") { failures.push("Missing short name flag"); }
    if !response.contains("<NAME>") { failures.push("Missing name parameter"); }
    if !response.contains("-h") { failures.push("Missing short help flag"); }
    if !response.contains("Print help") { failures.push("Missing help description"); }

    if !failures.is_empty() {
        panic!("Test failures: {}", failures.join(", "));
    }

    println!("✅ All expected error messages and options found");
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
        "Expected agent creation confirmation"
    );
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
    println!("📝 Agent swap response: {} bytes", _response2.len());
    println!("📝 Agent swap response Full output : {}", _response2);

    assert!(
        _response2.contains("✓") || _response2.contains("Choose one of the following agents"),
        "Expected agent swap confirmation"
    );
    drop(chat);
    Ok(())
}