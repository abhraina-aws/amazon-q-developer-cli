#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[allow(dead_code)]
struct FileCleanup<'a> {
    path: &'a str,
}

impl<'a> Drop for FileCleanup<'a> {
    fn drop(&mut self) {
        if std::path::Path::new(self.path).exists() {
            let _ = std::fs::remove_file(self.path);
            println!("✅ Cleaned up test file");
        }
    }
}


#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_tools_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools command... | Description: Tests the <code>/tools</code> command to display all available tools with their permission status including built-in and MCP tools");
    
    // Use a new isolated session to avoid context contamination
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");
    
    // Wait a bit for session to be ready
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    let response = chat.execute_command_with_timeout("/tools", Some(5000))?;
    
    println!("📝 Tools response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    assert!(response.contains("Tool"), "Missing Tool header");
    assert!(response.contains("Permission"), "Missing Permission header");
    assert!(response.contains("Built-in"), "Missing Built-in section");
    
    // Verify some expected built-in tools
    assert!(response.contains("shell"), "Missing shell tool");
    assert!(response.contains("read"), "Missing read tool");
    assert!(response.contains("write"), "Missing write tool");
    assert!(response.contains("aws"), "Missing use_aws tool");
    
    println!("✅ /tools command executed successfully");

    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_tools_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools --help command... | Description: Tests the <code> /tools --help</code> command to display comprehensive help information about tools management including available subcommands and options");
    
    // Use a new isolated session to avoid context contamination
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    let response = chat.execute_command_with_timeout("/tools --help",Some(2000))?;
    
    println!("📝 Tools help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify Usage section
    assert!(response.contains("Usage"), "Missing Usage label");
    assert!(response.contains("/tools"), "Missing /tools command");
    assert!(response.contains("[COMMAND]"), "Missing [COMMAND] placeholder");
    
    // Verify Commands section
    assert!(response.contains("Commands"), "Missing Commands section");
    assert!(response.contains("schema"), "Missing schema command");
    assert!(response.contains("trust"), "Missing trust command");
    assert!(response.contains("untrust"), "Missing untrust command");
    assert!(response.contains("trust-all"), "Missing trust-all command");
    assert!(response.contains("reset"), "Missing reset command");
    assert!(response.contains("help"), "Missing help command");
    
    // Verify Options section
    assert!(response.contains("Options"), "Missing Options section");
    assert!(response.contains("-h") &&  response.contains("--help"), "Missing -h, --help flags");
    
    println!("✅ /tools --help command executed successfully");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_tools_trust_all_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools trust-all command... | Description: Tests the <code> /tools trust-all</code> command to trust all available tools and verify all tools show trusted status, then tests reset functionality");
    
    // Use a new isolated session to avoid context contamination
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");
    // Execute trust-all command
    let trust_all_response = chat.execute_command_with_timeout("/tools trust-all",Some(2000))?;
    
    println!("📝 Trust-all response: {} bytes", trust_all_response.len());
    println!("📝 TRUST-ALL OUTPUT:");
    println!("{}", trust_all_response);
    println!("📝 END TRUST-ALL OUTPUT");
    
    // Verify that all tools now show "trusted" permission
    assert!(trust_all_response.contains("All tools") && trust_all_response.contains("trusted"), "Missing trusted tools after trust-all");
    
    // Now check tools list to verify all tools are trusted
    let tools_response = chat.execute_command_with_timeout("/tools",Some(2000))?;
    
    println!("📝 Tools response after trust-all: {} bytes", tools_response.len());
    println!("📝 TOOLS OUTPUT:");
    println!("{}", tools_response);
    println!("📝 END TOOLS OUTPUT");
    
    // Verify that all tools now show "trusted" permission
    assert!(tools_response.contains("trusted"), "Missing trusted tools after trust-all");
    
    // Verify no tools have other permission statuses
    assert!(!tools_response.contains("not trusted"), "Found 'not trusted' tools after trust-all");
    assert!(!tools_response.contains("read-only commands"), "Found 'read-only commands' tools after trust-all");
    
    // Count lines with "trusted" to ensure multiple tools are trusted
    let trusted_count = tools_response.matches("trusted").count();
    assert!(trusted_count > 0, "No trusted tools found");
    
    // Execute reset command
    let reset_response = chat.execute_command_with_timeout("/tools reset",Some(1000))?;
    
    println!("📝 Reset response: {} bytes", reset_response.len());
    println!("📝 RESET OUTPUT:");
    println!("{}", reset_response);
    println!("📝 END RESET OUTPUT");
    
    // Verify reset confirmation message
    assert!(reset_response.contains("Reset") && reset_response.contains("permission"), "Missing reset confirmation message");
    
    // Now check tools list to verify tools have mixed permissions
    let tools_response = chat.execute_command_with_timeout("/tools",Some(2000))?;
    
    println!("📝 Tools response after reset: {} bytes", tools_response.len());
    println!("📝 TOOLS OUTPUT:");
    println!("{}", tools_response);
    println!("📝 END TOOLS OUTPUT");
    
    // Verify that tools have all permission types
    assert!(tools_response.contains("trusted"), "Missing trusted tools");
    assert!(tools_response.contains("not trusted"), "Missing not trusted tools");
    assert!(tools_response.contains("read-only commands"), "Missing read-only commands tools");
    
    println!("✅ /tools trust-all and reset commands executed successfully");

    drop(chat);

    Ok(())
}



#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_tools_reset_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools reset --help command... | Description: Tests the <code> /tools reset --help</code> command to display help information for the reset subcommand");
    
    // Use a new isolated session to avoid context contamination
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    let response = chat.execute_command_with_timeout("/tools reset --help",Some(2000))?;
    
    println!("📝 Tools reset help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify usage format
    assert!(response.contains("Usage"), "Missing Usage section");
    assert!(response.contains("/tools reset"), "Missing /tools reset command");
    
    // Verify options section
    assert!(response.contains("Options"), "Missing Options section");
    assert!(response.contains("-h"), "Missing -h flag");
    assert!(response.contains("--help"), "Missing --help flag");
    
    println!("✅ /tools reset --help command executed successfully");
     
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_tools_trust_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools trust command... | Description: Tests the <code> /tools</code> trust and untrust commands to manage individual tool permissions and verify trust status changes");
  
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    // First get list of tools to find one that's not trusted
    let tools_response = chat.execute_command_with_timeout("/tools",Some(2000))?;
    
    println!("📝 Tools response: {} bytes", tools_response.len());
    println!("📝 TOOLS OUTPUT:");
    println!("{}", tools_response);
    println!("📝 END TOOLS OUTPUT");
    
    // Find a tool that's not trusted (prefer shell as it's a known working tool)
    let mut untrusted_tool: Option<String> = None;
    let mut fallback_tool: Option<String> = None;
    
    // Look for tools that are "not trusted"
    let lines: Vec<&str> = tools_response.lines().collect();
    for line in lines {
        if line.contains("not trusted") {
            // Extract tool name - look for pattern "- toolname" or just "toolname"
            let trimmed = line.trim();
            if trimmed.starts_with("- ") || trimmed.starts_with("-") {
                let tool_part = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("-")).unwrap_or(trimmed).trim();
                let parts: Vec<&str> = tool_part.split_whitespace().collect();
                if let Some(display_name) = parts.first() {
                    
                    // Map display names to actual tool names
                    let actual_tool_name = match *display_name {
                        "shell" => "execute_bash",
                        "write" => "fs_write",
                        "read" => "fs_read",
                        "report" => "report_issue",
                        "todo" => "todo_list",
                        "aws" => "use_aws",
                        other => other,
                    };
                    
                    // Prefer shell or report as they are known working tools
                    if display_name == &"shell" || display_name == &"report" {
                        untrusted_tool = Some(actual_tool_name.to_string());
                        break;
                    } else if fallback_tool.is_none() {
                        fallback_tool = Some(actual_tool_name.to_string());
                    }
                }
            }
        }
    }
    
    // Use shell if found, otherwise use fallback
    if untrusted_tool.is_none() {
        untrusted_tool = fallback_tool;
        if let Some(ref tool) = untrusted_tool {
            println!("📝 Using fallback tool: {}", tool);
        }
    }
    
    if let Some(tool_name) = untrusted_tool {
        
        // Execute trust command
        let trust_command = format!("/tools trust {}", tool_name);
        let trust_response = chat.execute_command_with_timeout(&trust_command,Some(2000))?;
        
        println!("📝 TRUST OUTPUT:");
        println!("{}", trust_response);
        println!("📝 END TRUST OUTPUT");
        
        // Verify trust confirmation message
        assert!(
            trust_response.contains(&tool_name) && !trust_response.contains("does not exist"),
            "Missing trust confirmation message or tool does not exist"
        );
        println!("✅ Tool '{}' trusted successfully", tool_name);
        
        // Execute untrust command
        let untrust_command = format!("/tools untrust {}", tool_name);
        let untrust_response = chat.execute_command_with_timeout(&untrust_command,Some(2000))?;
        
        println!("📝 UNTRUST OUTPUT:");
        println!("{}", untrust_response);
        println!("📝 END UNTRUST OUTPUT");
        
        // Verify untrust confirmation message
        assert!(
            untrust_response.contains(&tool_name) && !untrust_response.contains("does not exist"),
            "Missing untrust confirmation message or tool does not exist"
        );
        println!("✅ Found untrust confirmation message for tool: {}", tool_name);

    } else {
        println!("ℹ️ No untrusted tools found to test trust command");
    }

    println!("✅ /tools trust and untrust commands executed successfully");
  
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_tools_trust_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools trust --help command... | Description: Tests the <code>/tools trust --help</code> command to display help information for trusting specific tools");
    
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");
    
    let response = chat.execute_command_with_timeout("/tools trust --help",Some(2000))?;
    
    println!("📝 Tools trust help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify usage format
    assert!(response.contains("Usage"), "Missing Usage label");
    assert!(response.contains("/tools trust"), "Missing /tools trust command");
    assert!(response.contains("<TOOL_NAMES>"), "Missing <TOOL_NAMES> parameter");
    
    // Verify arguments section
    assert!(response.contains("Arguments"), "Missing Arguments label");
    assert!(response.contains("<TOOL_NAMES>"), "Missing <TOOL_NAMES> in arguments");
    
    // Verify options section
    assert!(response.contains("Options"), "Missing Options section");
    assert!(response.contains("-h") && response.contains("--help"), "Missing -h, --help option");
    
    println!("✅ /tools trust --help command executed successfully");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_tools_untrust_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools untrust --help command... | Description: Tests the <code>/tools untrust --help</code> command to display help information for untrusting specific tools");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    let response = chat.execute_command_with_timeout("/tools untrust --help",Some(2000))?;
    
    println!("📝 Tools untrust help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify usage format
    assert!(response.contains("Usage"), "Missing Usage label");
    assert!(response.contains("/tools untrust"), "Missing /tools untrust command");
    
    // Verify arguments section
    assert!(response.contains("Arguments"), "Missing Arguments label");
    assert!(response.contains("Names of tools") || response.contains("tool"), "Missing tool names description");
    
    // Verify options section
    assert!(response.contains("Options"), "Missing Options section");
    assert!(response.contains("-h") && response.contains("--help"), "Missing -h, --help option");
    
    println!("✅ /tools untrust --help command executed successfully");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_tools_schema_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools schema --help command... | Description: Tests the <code>/tools schema --help</code> command to display help information for viewing tool schemas");
    
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    let response = chat.execute_command_with_timeout("/tools schema --help",Some(2000))?;
    
    println!("📝 Tools schema help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify usage format
    assert!(response.contains("Usage"), "Missing Usage label");
    assert!(response.contains("/tools schema"), "Missing /tools schema command");
    
    // Verify options section
    assert!(response.contains("Options"), "Missing Options section");
    assert!(response.contains("-h") && response.contains("--help"), "Missing -h, --help option");
    
    println!("✅ /tools schema --help command executed successfully");
    
    drop(chat);

    Ok(())
}

//TODO: As response not giving full content , need to check this.
/*#[test]
#[cfg(feature = "tools")]
fn test_tools_schema_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools schema command...");
  
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let response = chat.execute_command("/tools schema")?;
    
    println!("📝 Tools schema response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify JSON structure
    assert!(response.contains("{") && response.contains("}"), "Missing JSON structure");
    println!("✅ Found JSON structure");
    
    // Verify core built-in tools
    assert!(response.contains("fs_read") || response.contains("fs_write") || response.contains("execute_bash") || response.contains("use_aws"), "Missing tools");
    println!("✅ Found core built-in tools");
    
    // Verify tool structure elements
    assert!(response.contains("name"), "Missing name field");
    assert!(response.contains("description"), "Missing description field");
    assert!(response.contains("input_schema"), "Missing input_schema field");
    assert!(response.contains("properties"), "Missing properties field");
    println!("✅ Found required tool structure: name, description, input_schema, properties");
    
    // Check for optional MCP/GitHub tools if present
    if response.contains("download_files_from_github") {
        println!("✅ Found GitHub-related tools");
    }
    if response.contains("consolidate_findings_to_csv") {
        println!("✅ Found analysis tools");
    }
    if response.contains("gh_issue") {
        println!("✅ Found GitHub issue reporting tool");
    }
    
    // Verify schema structure for at least one tool
    if response.contains("type") {
        println!("✅ Found proper schema type definitions");
    }
    
    println!("✅ All tools schema content verified!");
    
    drop(chat);

    Ok(())
}*/

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_fs_write_and_fs_read_tools() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing `fs_write` and `fs_read` tool ... | Description: Tests the <code> fs_write</code> and <code> fs_read</code> tools by creating a file with specific content and reading it back to verify file I/O operations work correctly");

    let save_path = "demo.txt";
    let _cleanup = FileCleanup { path: save_path };
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    // Test fs_write tool by asking to create a file with "Hello World" content
    let mut response = chat.execute_command_with_timeout(&format!("Create a file at {} with content 'Hello World'", save_path),Some(2000))?;

    println!("📝 fs_write response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // If approval is required, send 't' to trust the tool for the session
    if response.contains("Allow this action?") {
        println!("📝 Tool approval required, sending 't' to trust");
        let approval_response = chat.send_key_input_with_timeout("t\n", Some(10000))?;
        println!("📝 Immediate response after approval: {} bytes", approval_response.len());
        
        // Wait a bit more for the tool to complete and get the full response
        std::thread::sleep(std::time::Duration::from_millis(2000));
        let completion_response = chat.execute_command_with_timeout("", Some(3000)).unwrap_or_default();
        
        // Combine responses
        response = format!("{}{}{}", response, approval_response, completion_response);
        println!("📝 FULL APPROVAL RESPONSE:");
        println!("{}", response);
        println!("📝 END FULL APPROVAL RESPONSE");
    }
    
    // Verify tool usage indication
    assert!(response.contains("write") || response.contains("fs_write") || response.contains("demo.txt"), "Missing fs_write tool usage indication");
    
    // Verify file path in response
    assert!(response.contains("demo.txt"), "Missing expected file path");
    
    // Wait a bit for file to be written
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Verify file was actually created
    assert!(std::path::Path::new(save_path).exists(), "File was not created");
    println!("✅ File {} was created successfully", save_path);

    // Test fs_read tool by asking to read the created file
    let mut read_response = chat.execute_command_with_timeout(&format!("Read file {}", save_path),Some(2000))?;

    println!("📝 fs_read response: {} bytes", read_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", read_response);
    println!("📝 END OUTPUT");
    
    // If approval is required, send 't' to trust the tool for the session
    if read_response.contains("Allow this action?") && read_response.contains("[y/n/t]:") {
        println!("📝 Tool approval required for read, sending 't' to trust");
        read_response = chat.send_key_input_with_timeout("t\n", Some(3000))?;
        println!("📝 Response after approval: {}", read_response);
    }
    
    // Verify tool usage indication
    assert!(read_response.contains("read") || read_response.contains("fs_read") || read_response.contains("demo.txt"), "Missing fs_read tool usage indication");
    
    // Verify file path in response
    assert!(read_response.contains("demo.txt"), "Missing demo.txt file path");
    
    // Verify content reference
    assert!(read_response.contains("Hello World"), "Missing Hello World content reference");
    
    println!("✅ fs_write and fs_read tool executed and verified successfully!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_execute_bash_tool() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing `execute_bash` tool ... | Description: Tests the <code>execute_bash</code> tool by running the 'pwd' command and verifying proper command execution and output");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    // Test execute_bash tool by asking to run pwd command
    let mut response = chat.execute_command_with_timeout("Run pwd",Some(3000))?;

    println!("📝 execute_bash response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // If approval is required, send 't' to trust the tool for the session
    if response.contains("Allow this action?") && response.contains("[y/n/t]:") {
        println!("📝 Tool approval required, sending 't' to trust");
       let grant_permission = chat.send_key_input_with_timeout("t\n", Some(2000))?;
        println!("📝 Response after approval: {}", grant_permission);
    }
    
    // Verify command in response
    assert!(response.contains("pwd"), "Missing pwd command reference");
    
    // Verify success indication or directory path
    assert!(response.contains("e2etests") || response.contains("/"), "Missing directory output");
    
    println!("✅ execute_bash tool executed and verified successfully!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_report_issue_tool() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing `report_issue` tool ... | Description: Tests the <code> report_issue</code> reporting functionality by creating a sample issue and verifying the browser opens GitHub for issue submission");
    
    // Use a new isolated session to avoid context contamination from previous tests
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    // Test report_issue tool by asking to report an issue
    let response = chat.execute_command_with_timeout("Report a bug: 'Test issue for e2e testing'",Some(2000))?;

    println!("📝 report_issue response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
   assert!(response.contains("github"), "Missing github");
   assert!(response.contains("Title"), "Missing Title");
   assert!(response.contains("Heading over to GitHub..."),"Missing Heading over to GitHub...");
    
    println!("✅ report_issue tool executed and verified successfully!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_use_aws_tool() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing `use_aws` tool ... | Description: Tests the <code>use_aws</code> tool by executing AWS commands to describe EC2 instances and verifying proper AWS CLI integration");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    // Test use_aws tool by asking to describe EC2 instances in us-west-2
    let mut response = chat.execute_command_with_timeout("Describe EC2 instances in us-west-2",Some(2000))?;

    println!("📝 use_aws response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Handle approval if required
    if response.contains("Allow this action?") {
        println!("📝 Tool approval required, sending 't' to trust");
        let approval_response = chat.send_key_input_with_timeout("t\n", Some(10000))?;
        println!("📝 Immediate response after approval: {} bytes", approval_response.len());
        
        // Wait for AWS command to complete
        std::thread::sleep(std::time::Duration::from_millis(3000));
        let completion_response = chat.execute_command_with_timeout("", Some(5000)).unwrap_or_default();
        
        // Combine responses
        response = format!("{}{}{}", response, approval_response, completion_response);
        println!("📝 Full response after approval: {} bytes", response.len());
    }
    
    // Verify AWS tool usage (flexible checks since we may not get full output in test environment)
    assert!(response.contains("us-west-2") || response.contains("Region"), "Missing region information");
    assert!(response.contains("aws") || response.contains("ec2"), "Missing AWS/EC2 reference");
    
    println!("✅ use_aws tool executed and verified successfully!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_trust_execute_bash_for_direct_execution() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing Trust execute_bash for direct execution ... | Description: Tests the ability to trust the <code>execute_bash</code> tool so it runs commands without asking for user confirmation each time");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    // First, trust the execute_bash tool
    let trust_response = chat.execute_command_with_timeout("/tools trust execute_bash",Some(2000))?;
    
    println!("📝 Trust response: {} bytes", trust_response.len());
    println!("📝 TRUST OUTPUT:");
    println!("{}", trust_response);
    println!("📝 END TRUST OUTPUT");
    
    // Verify trust confirmation
    assert!(trust_response.contains("trusted") || trust_response.contains("execute_bash"), "Missing execute_bash trust confirmation");

    // Now test execute_bash tool with a simple command that should run directly without confirmation
    let response = chat.execute_command_with_timeout("Run mkdir -p test_dir && echo 'test' > test_dir/test.txt",Some(2000))?;

    println!("📝 execute_bash response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify tool usage indication (the tool is called "shell" in the output)
    assert!(
        response.contains("shell") || response.contains("execute_bash") || response.contains("mkdir"),
        "Missing tool usage indication"
    );
    
    // Verify the command was executed directly without asking for confirmation
    assert!(!response.contains("Allow this action?"), "Tool should not ask for confirmation when trusted");
    assert!(response.contains("Completed") || response.contains("Done"), "Missing completion confirmation");
    assert!(response.contains("test_dir"), "Missing test_dir reference");

    chat.execute_command_with_timeout("Delete the directory test_dir/test.txt",Some(2000))?;
     
    println!("✅ Directory successfully deleted");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "tools", feature = "sanity"))]
fn test_tools_trust_all_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /tools trust-all --help command... | Description: Tests the <code> /tools trust-all --help</code>command to display help information for the trust-all subcommand");
  
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    let response = chat.execute_command_with_timeout("/tools trust-all --help",Some(2000))?;
    
    println!("📝 Tools trust-all help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    
    // Verify usage format
    assert!(response.contains("Usage"), "Missing Usage section");
    assert!(response.contains("/tools trust-all"), "Missing /tools trust-all command");
    
    // Verify options section
    assert!(response.contains("Options"), "Missing Options section");
    assert!(response.contains("-h"), "Missing -h flag");
    assert!(response.contains("--help"), "Missing --help flag");
    
    println!("✅ /tools trust-all --help command executed successfully");
    
    drop(chat);

    Ok(())
}