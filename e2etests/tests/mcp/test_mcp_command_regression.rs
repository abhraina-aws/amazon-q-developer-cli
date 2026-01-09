#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "mcp", feature = "regression"))]
fn test_mcp_remove_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro mcp remove --help command... | Description: Tests the <code> kiro mcp remove --help</code> command to display help information for removing MCP servers");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Execute q mcp remove --help command
    let help_response = chat.execute_command_with_timeout("execute below bash command q mcp remove --help",Some(1000))?;
    
    println!("📝 MCP remove help response: {} bytes", help_response.len());
    println!("📝 HELP RESPONSE:");
    println!("{}", help_response);
    println!("📝 END HELP RESPONSE");
    
    // Verify tool execution prompt appears
    assert!(help_response.contains("Using tool"), "Missing tool execution indicator");
    assert!(help_response.contains("Allow this action?"), "Missing permission prompt");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command_with_timeout("y",Some(500))?;

    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    // Verify complete help content in final response
    assert!(allow_response.contains("Usage") && allow_response.contains("qchat mcp remove"), "Missing usage information");
    assert!(allow_response.contains("Options"), "Missing option information");
    assert!(allow_response.contains("--name <NAME>"), "Missing --name option");
    assert!(allow_response.contains("--scope <SCOPE>"), "Missing --scope option");
    assert!(allow_response.contains("--agent <AGENT>"), "Missing --agent option");
    assert!(allow_response.contains("-h, --help"), "Missing help option");
    println!("✅ Found all expected MCP remove help content and completion");
    
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "mcp", feature = "regression"))]
fn test_mcp_add_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro mcp add --help command... | Description: Tests the <code> kiro mcp add --help</code> command to display help information for adding new MCP servers");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Execute mcp add --help command
    println!("\n🔍 Executing command: 'kiro mcp add --help'");
    let response = chat.execute_command_with_timeout("execute below bash command q mcp add --help",Some(1000))?;
    
    println!("📝 Restart response: {} bytes", response.len());
    println!("📝 RESTART RESPONSE:");
    println!("{}", response);
    println!("📝 END RESTART RESPONSE");
    
    // Verify tool execution details
    assert!(response.contains("q mcp add --help"), "Missing command execution description");
    assert!(response.contains("Purpose"), "Missing purpose description");
    println!("✅ Found tool execution details");

    // Verify tool execution prompt appears
    assert!(response.contains("Using tool"), "Missing tool execution indicator");
    assert!(response.contains("Allow this action?"), "Missing permission prompt");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    
    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    // Verify mcp add help output
    assert!(allow_response.contains("Usage") && allow_response.contains("qchat mcp add"), "Missing usage information");
     assert!(allow_response.contains("Options"), "Missing Options");
    assert!(allow_response.contains("--name <NAME>"), "Missing --name option");
    assert!(allow_response.contains("--command <COMMAND>"), "Missing --command option");
    assert!(allow_response.contains("--scope <SCOPE>"), "Missing --scope option");
    assert!(allow_response.contains("--agent <AGENT>"), "Missing --agent option");
    assert!(allow_response.contains("--force"), "Missing --force option");
    assert!(allow_response.contains("--help"), "Missing --help option");
    assert!(allow_response.contains("Completed in"), "Missing completion indicator");
    assert!(allow_response.contains("Required"), "Missing Requried indicator");
    assert!(allow_response.contains("Optional"), "Missing Optional indicator");
    println!("✅ MCP add help command executed successfully");
    
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "mcp", feature = "regression"))]
fn test_mcp_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro mcp --help command... | Description: Tests the <code> kiro mcp --help</code> command to display comprehensive MCP management help including all subcommands");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Execute q mcp --help command
    let help_response = chat.execute_command_with_timeout("execute below bash command q mcp --help",Some(1000))?;
    
    println!("📝 MCP help response: {} bytes", help_response.len());
    println!("📝 HELP RESPONSE:");
    println!("{}", help_response);
    println!("📝 END HELP RESPONSE");
    
    // Verify tool execution prompt appears
    assert!(help_response.contains("Using tool"), "Missing tool execution indicator");
    assert!(help_response.contains("Allow this action?"), "Missing permission prompt");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command_with_timeout("y",Some(500))?;

    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    // Verify complete help content
    assert!(allow_response.contains("Model Context Protocol (MCP)"), "Missing MCP description");
    assert!(allow_response.contains("Usage") && allow_response.contains("qchat mcp"), "Missing usage information");
    assert!(allow_response.contains("Commands"), "Missing Commands section");
    
    // Verify command descriptions
    assert!(allow_response.contains("add"), "Missing add command description");
    assert!(allow_response.contains("remove"), "Missing remove command description");
    assert!(allow_response.contains("list"), "Missing list command description");
    assert!(allow_response.contains("import"), "Missing import command description");
    assert!(allow_response.contains("status"), "Missing status command description");
    assert!(allow_response.contains("help"), "Missing help command");
    println!("✅ Found all MCP commands with descriptions");
    
    assert!(allow_response.contains("Options"), "Missing Options section");
    assert!(allow_response.contains("-v, --verbose"), "Missing verbose option");
    assert!(allow_response.contains("-h, --help"), "Missing help option");
    println!("✅ Found all expected MCP help content and completion");
    
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "mcp", feature = "regression"))]
fn test_mcp_import_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro mcp import --help command... | Description: Tests the <code> kiro mcp import --help</code> command to display help information for importing MCP server configurations");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Execute mcp import --help command
    println!("\n🔍 Executing command: 'kiro mcp import --help'");
    let response = chat.execute_command_with_timeout("execute below bash command q mcp import --help",Some(1000))?;
    
    println!("📝 Restart response: {} bytes", response.len());
    println!("📝 RESTART RESPONSE:");
    println!("{}", response);
    println!("📝 END RESTART RESPONSE");

    // Verify tool execution details
    assert!(response.contains("q mcp import --help"), "Missing command execution description");
    assert!(response.contains("Purpose"), "Missing purpose description");
    println!("✅ Found tool execution details");
    
    // Verify tool execution prompt appears
    assert!(response.contains("Using tool"), "Missing tool execution indicator");
    assert!(response.contains("Allow this action?"), "Missing permission prompt");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    
    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    // Verify usage line
    assert!(allow_response.contains("Usage"), "Missing complete usage line");
    println!("✅ Found usage information");
    
    // Verify Arguments section
    assert!(allow_response.contains("Arguments"), "Missing Arguments section");
    println!("✅ Found Arguments section with SCOPE");
    
    // Verify Options section
    assert!(allow_response.contains("Options"), "Missing Options section");
    assert!(allow_response.contains("--file <FILE>"), "Missing --file option");
    assert!(allow_response.contains("--force"), "Missing --force option");
    assert!(allow_response.contains("-v, --verbose..."), "Missing --verbose option");
    assert!(allow_response.contains("-h, --help"), "Missing --help option");
    println!("✅ Found all options with descriptions");
    
    println!("✅ All q mcp import --help content verified successfully");
    
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "mcp", feature = "regression"))]
fn test_mcp_list_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro mcp list command... | Description: Tests the <code> kiro mcp list</code> command to display all configured MCP servers and their status");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    let response = chat.execute_command_with_timeout("execute below bash command q mcp list",Some(1000))?;
    
    println!("📝 MCP list response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify tool execution prompt
    assert!(response.contains("Using tool"), "Missing tool execution indicator");
    assert!(response.contains("q mcp list"), "Missing command in tool execution");
    assert!(response.contains("Allow this action?"), "Missing permission prompt");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    
    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    
    // Verify MCP server listing
    assert!(allow_response.contains("q_cli_default"), "Missing q_cli_default server");
    println!("✅ Found MCP server listing with  servers and completion");
    
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "mcp", feature = "regression"))]
fn test_mcp_list_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro mcp list --help command... | Description: Tests the <code> kiro mcp list --help</code> command to display help information for listing MCP servers");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    let response = chat.execute_command_with_timeout("execute below bash command q mcp list --help",Some(1000))?;
    
    println!("📝 MCP list help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify tool execution prompt
    assert!(response.contains("Using tool"), "Missing tool execution indicator");
    assert!(response.contains("q mcp list --help"), "Missing command in tool execution");
    assert!(response.contains("Allow this action?"), "Missing permission prompt");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    
    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    // Verify help content
    assert!(allow_response.contains("Usage"), "Missing usage format");
    
    // Verify arguments section
    assert!(allow_response.contains("Arguments"), "Missing Arguments section");
    assert!(allow_response.contains("[SCOPE]"), "Missing scope argument");
    
    // Verify options section
    assert!(allow_response.contains("Options"), "Missing Options section");
    assert!(allow_response.contains("-v") && allow_response.contains("--verbose"), "Missing verbose option");
    assert!(allow_response.contains("-h") && allow_response.contains("--help"), "Missing help option");
    
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "mcp", feature = "regression"))]
fn test_mcp_status_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro mcp status --help command... | Description: Tests the <code> kiro mcp status --help</code> command to display help information for checking MCP server status");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Execute mcp status --help command
    println!("\n🔍 Executing command: 'kiro mcp status --help'");
    let response = chat.execute_command_with_timeout("execute below bash command q mcp status --help",Some(1000))?;

    println!("📝 Restart response: {} bytes", response.len());
    println!("📝 RESTART RESPONSE:");
    println!("{}", response);
    println!("📝 END RESTART RESPONSE");

    // Verify tool execution details
    assert!(response.contains("Purpose"), "Missing purpose description");
    println!("✅ Found tool execution details");
    
    // Verify tool execution prompt appears
    assert!(response.contains("Using tool"), "Missing tool execution indicator");
    assert!(response.contains("Allow this action?"), "Missing permission prompt");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    
    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    // Verify usage line
    assert!(allow_response.contains("Usage") && allow_response.contains("qchat mcp status [OPTIONS] --name <NAME>"), "Missing complete usage line");
    println!("✅ Found usage information");
    
    // Verify Options section
    assert!(allow_response.contains("Options"), "Missing Options section");
    assert!(allow_response.contains("--name <NAME>"), "Missing --name option");
    assert!(allow_response.contains("-v, --verbose") , "Missing --verbose option");
    assert!(allow_response.contains("-h, --help"), "Missing --help option");
    println!("✅ Found all options with descriptions");
    
    println!("✅ All q mcp status --help content verified successfully");
    
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "mcp", feature = "regression"))]
fn test_add_and_remove_mcp_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro mcp add command... | Description: Tests the <code> kiro mcp add</code> and <code> q mcp remove</code> subcommands to add and remove MCP servers");

    // First install uv dependency before starting Q Chat
    println!("\n🔍 Installing uv dependency...");

    std::process::Command::new("pip3")
        .args(["install", "uv", "--break-system-packages"])
        .output()
        .expect("Failed to install uv");
    
    println!("✅ uv dependency installed");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    // First check if MCP already exists using q mcp list
    println!("\n🔍 Checking if aws-documentation MCP already exists...");
    let list_response = chat.execute_command_with_timeout("execute below bash command q mcp list",Some(1000))?;
    
    println!("📝 List response: {} bytes", list_response.len());
    println!("📝 LIST RESPONSE:");
    println!("{}", list_response);
    println!("📝 END LIST RESPONSE");
    
    // Allow the list command
    let list_allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    println!("📝 List allow response: {} bytes", list_allow_response.len());
    println!("📝 LIST ALLOW RESPONSE:");
    println!("{}", list_allow_response);
    println!("📝 END LIST ALLOW RESPONSE");
    
    // Check if aws-documentation exists in the list
    if list_allow_response.contains("aws-documentation") {
        println!("\n🔍 aws-documentation MCP already exists, removing it first...");
        
        let remove_response = chat.execute_command_with_timeout("execute below bash command q mcp remove --name aws-documentation",Some(1000))?;
        println!("📝 Remove response: {} bytes", remove_response.len());
        println!("📝 REMOVE RESPONSE:");
        println!("{}", remove_response);
        println!("📝 END REMOVE RESPONSE");
        
        // Allow the remove command
        let remove_allow_response = chat.execute_command_with_timeout("y",Some(500))?;
        println!("📝 Remove allow response: {} bytes", remove_allow_response.len());
        println!("📝 REMOVE ALLOW RESPONSE:");
        println!("{}", remove_allow_response);
        println!("📝 END REMOVE ALLOW RESPONSE");
        
        // Verify successful removal
        assert!(remove_allow_response.contains("Removed") && remove_allow_response.contains("'aws-documentation'"), "Missing removal success message");
        println!("✅ Successfully removed existing aws-documentation MCP");
    } else {
        println!("✅ aws-documentation MCP does not exist, proceeding with add");
    }

    // Now add the MCP server
    println!("\n🔍 Executing command: 'q mcp add --name aws-documentation --command uvx --args awslabs.aws-documentation-mcp-server@latest'");
    let response = chat.execute_command_with_timeout("execute below bash command q mcp add --name aws-documentation --command uvx --args awslabs.aws-documentation-mcp-server@latest",Some(2000))?;
    
    println!("📝 Response: {} bytes", response.len());
    println!("📝 RESPONSE:");
    println!("{}", response);
    println!("📝 END RESPONSE");

    // Verify tool execution details
    assert!(response.contains("q mcp add --name aws-documentation --command uvx --args awslabs.aws-documentation-mcp-server@latest"), "Missing full command");
    assert!(response.contains("Using tool"), "Missing tool execution indicator");
    assert!(response.contains("Allow this action?"), "Missing permission prompt");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    // Verify successful addition
    assert!(allow_response.contains("Added") && allow_response.contains("'aws-documentation'"), "Missing success message");
    assert!(allow_response.contains("/Users/") && allow_response.contains("/.aws/amazonq/mcp.json"), "Missing config file path");
    println!("✅ Found successful addition message");
    
    // Now test removing the MCP server
    println!("\n🔍 Executing remove command: 'q mcp remove --name aws-documentation'");
    let remove_response = chat.execute_command_with_timeout("execute below bash command q mcp remove --name aws-documentation",Some(2000))?;
    println!("📝 Remove response: {} bytes", remove_response.len());
    println!("📝 REMOVE RESPONSE:");
    println!("{}", remove_response);
    println!("📝 END REMOVE RESPONSE");
    
    // Verify remove tool execution details
    assert!(response.contains("Using tool"), "Missing using tool indicator");
    assert!(remove_response.contains("q mcp remove --name aws-documentation"), "Missing full remove command");
    assert!(remove_response.contains("Allow this action?"), "Missing remove permission prompt");
    println!("✅ Found remove tool execution permission prompt");
    
    // Allow the remove tool execution
    let remove_allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    println!("📝 Remove allow response: {} bytes", remove_allow_response.len());
    println!("📝 REMOVE ALLOW RESPONSE:");
    println!("{}", remove_allow_response);
    println!("📝 END REMOVE ALLOW RESPONSE");
    
    // Verify successful removal
    assert!(remove_allow_response.contains("Removed") && remove_allow_response.contains("'aws-documentation'"), "Missing removal success message");
    assert!(remove_allow_response.contains("/Users/") && remove_allow_response.contains("/.aws/amazonq/mcp.json"), "Missing config file path in removal");
    println!("✅ Found successful removal message");
    
    drop(chat);
    Ok(())
}

#[test]
#[cfg(all(feature = "mcp", feature = "regression"))]
fn test_mcp_status_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro mcp status --name <server-name> command... | Description: Tests the <code> kiro mcp status</code> command with server name to display detailed status information for a specific MCP server");

    // First install uv dependency before starting Q Chat
    println!("\n🔍 Installing uv dependency...");

    std::process::Command::new("pip3")
        .args(["install", "uv", "--break-system-packages"])
        .output()
        .expect("Failed to install uv");
    
    println!("✅ uv dependency installed");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    // Execute mcp add command
    println!("\n🔍 Executing command: 'q mcp add --name aws-documentation --command uvx --args awslabs.aws-documentation-mcp-server@latest'");
    let response = chat.execute_command_with_timeout("execute below bash command q mcp add --name aws-documentation --command uvx --args awslabs.aws-documentation-mcp-server@latest",Some(2000))?;
    
    println!("📝 Response: {} bytes", response.len());
    println!("📝 RESPONSE:");
    println!("{}", response);
    println!("📝 END RESPONSE");

    // Verify tool execution details
    assert!(response.contains("q mcp add --name aws-documentation --command uvx --args awslabs.aws-documentation-mcp-server@latest"), "Missing full command");
    assert!(response.contains("Using tool"), "Missing tool execution indicator");
    println!("✅ Found tool execution permission prompt");
    
    // Allow the tool execution
    let allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    println!("📝 Allow response: {} bytes", allow_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", allow_response);
    println!("📝 END ALLOW RESPONSE");
    
    // Verify successful addition
    assert!(allow_response.contains("Added") && allow_response.contains("'aws-documentation'"), "Missing success message");
    println!("✅ Found successful addition message");

    // Allow the tool execution
    let response = chat.execute_command_with_timeout("execute below bash command q mcp status --name aws-documentation",Some(2000))?;
    println!("📝 Allow response: {} bytes", response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", response);
    println!("📝 END ALLOW RESPONSE");

     // Verify tool execution details
    assert!(response.contains("q mcp status --name aws-documentation"), "Missing full command");
    assert!(response.contains("Using tool"), "Missing tool execution indicator");
    println!("✅ Found tool execution permission prompt");

     // Allow the tool execution
    let show_response = chat.execute_command_with_timeout("y",Some(500))?;
    println!("📝 Allow response: {} bytes", show_response.len());
    println!("📝 ALLOW RESPONSE:");
    println!("{}", show_response);
    println!("📝 END ALLOW RESPONSE");


    // Verify successful status retrieval
    assert!(show_response.contains("Scope"), "Missing Scope");
    assert!(show_response.contains("Agent"), "Missing Agent");
    assert!(show_response.contains("Command"), "Missing Command");
     assert!(show_response.contains("Disabled"), "Missing Disabled");
      assert!(show_response.contains("Env Vars"), "Missing Env Vars");
    
    // Now test removing the MCP server
    println!("\n🔍 Executing remove command: 'q mcp remove --name aws-documentation'");
    let remove_response = chat.execute_command_with_timeout("execute below bash command q mcp remove --name aws-documentation",Some(2000))?;
    println!("📝 Remove response: {} bytes", remove_response.len());
    println!("📝 REMOVE RESPONSE:");
    println!("{}", remove_response);
    println!("📝 END REMOVE RESPONSE");
    
    // Verify remove tool execution details
    assert!(response.contains("Using tool"), "Missing using tool indicator");
    assert!(remove_response.contains("q mcp remove --name aws-documentation"), "Missing full remove command");
    assert!(remove_response.contains("Allow this action?"), "Missing remove permission prompt");
    println!("✅ Found remove tool execution permission prompt");
    
    // Allow the remove tool execution
    let remove_allow_response = chat.execute_command_with_timeout("y",Some(500))?;
    println!("📝 Remove allow response: {} bytes", remove_allow_response.len());
    println!("📝 REMOVE ALLOW RESPONSE:");
    println!("{}", remove_allow_response);
    println!("📝 END REMOVE ALLOW RESPONSE");
    
    // Verify successful removal
    assert!(remove_allow_response.contains("Removed") && remove_allow_response.contains("'aws-documentation'"), "Missing removal success message");
    assert!(remove_allow_response.contains("/Users/") && remove_allow_response.contains("/.aws/amazonq/mcp.json"), "Missing config file path in removal");
    println!("✅ Found successful removal message");
    
    drop(chat);
    Ok(())
}
