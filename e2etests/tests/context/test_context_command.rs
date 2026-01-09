#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_context_show_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context show command... | Description: Tests the <code> /context show</code> command to display current context information including agent configuration and context files");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    let response = chat.execute_command_with_timeout("/context show",Some(500))?;
    
    println!("📝 Context show response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify context show output contains expected sections
    assert!(response.contains("Agent"), "Missing Agent section");

    // Verify agent configuration details
    assert!(response.contains("kiro_default"), "Missing kiro_default");
    
    println!("✅ All context show content verified!");

    // Release the lock before cleanup
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_context_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context help command... | Description: Tests the <code> /context help</code> command to display comprehensive help information for context management including usage, commands, and options");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    let response = chat.execute_command_with_timeout("/context help",Some(500))?;
    
    println!("📝 Context help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify Usage section
    assert!(response.contains("Usage"), "Missing Usage section");
    assert!(response.contains("/context"),"Missing /context command");
    assert!(response.contains("[COMMAND]"), "Missing [COMMAND] placeholder");
    
    // Verify Commands section
    assert!(response.contains("Commands"), "Missing Commands section");
    assert!(response.contains("show"), "Missing show command");
    assert!(response.contains("add"), "Missing add command");
    assert!(response.contains("remove"), "Missing remove command");
    assert!(response.contains("clear"), "Missing clear command");
    assert!(response.contains("help"), "Missing help command");
        
    println!("✅ All context help content verified!");

    // Release the lock before cleanup
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_context_without_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context without sub command... | Description: Tests the <code> /context</code> command without subcommands to verify it displays help information with usage and available commands");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    let response = chat.execute_command_with_timeout("/context",Some(500))?;
    
    println!("📝 Context response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // /context without subcommands shows context usage information, not help
    assert!(response.contains("Current context window"), "Missing context window information");
    assert!(response.contains("% used"), "Missing % usage percentage");
    assert!(response.contains("Context files"), "Missing Context files section");
    assert!(response.contains("Tools"), "Missing Tools section");
    assert!(response.contains("Kiro responses"), "Missing Kiro responses section");
    assert!(response.contains("Your prompts"), "Missing Your prompts section");
    assert!(response.contains("Pro Tips"), "Missing Pro Tips section");

    println!("✅ All context help content verified!");

    // Release the lock before cleanup
    drop(chat);
    
    
    Ok(())
}

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_context_invalid_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context invalid command... | Description: Tests the <code> /context test</code> command with invalid subcommand to verify proper error handling and help display");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    let response = chat.execute_command_with_timeout("/context test",Some(500))?;
    
    println!("📝 Context invalid response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify error message for invalid subcommand
    assert!(response.contains("error"), "Missing error message");    

    println!("✅ All context invalid command content verified!");

    // Release the lock before cleanup
    drop(chat);
    

    Ok(())
}

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_add_non_existing_file_context() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context add non-existing file command... | Description: Tests the <code> /context add</code> command with non-existing file to verify proper error handling and force option suggestion");

    let non_existing_file_path = "/tmp/non_existing_file.py";

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Try to add non-existing file to context
    let add_response = chat.execute_command_with_timeout(&format!("/context add {}", non_existing_file_path),Some(1000))?;
    
    println!("📝 Context add response: {} bytes", add_response.len());
    println!("📝 ADD RESPONSE:");
    println!("{}", add_response);
    println!("📝 END ADD RESPONSE");
    
    // Verify error message for non-existing file
    assert!(add_response.contains("Error"), "Missing error message");

    println!("✅ All context add non-existing file content verified!");

    // Release the lock before cleanup
    drop(chat);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_context_remove_command_of_non_existent_file() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context remove non existing file command... | Description: Tests the <code> /context remove</code> command with non-existing file to verify proper error handling");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    let response = chat.execute_command_with_timeout("/context remove non_existent_file.txt",Some(1000))?;
    
    println!("📝 Context remove response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify error message for non-existent file
    assert!(response.contains("Error"), "Missing error message");

    println!("✅ All context remove non-existing file content verified!");

    // Release the lock before cleanup
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_add_remove_file_context() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context add <filename> command and /context remove <filename> command... | Description: Tests the <code> /context add</code> command to add a file to context and <code> /context remove</code> command to remove a file from context");

    let test_file_path = "/tmp/test_context_unique_file.py";
    // Create a test file
    std::fs::write(test_file_path, "# Test file for context\nprint('Hello from test file')")?;

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Clear context first to avoid interference from previous tests
    let _ = chat.execute_command_with_timeout("/context clear", Some(1000));
    
    // Add file to context
    let add_response = chat.execute_command_with_timeout(&format!("/context add {}", test_file_path),Some(1000))?;
    
    println!("📝 Context add response: {} bytes", add_response.len());
    println!("📝 ADD RESPONSE:");
    println!("{}", add_response);
    println!("📝 END ADD RESPONSE");
    
    // Verify file was added successfully - be flexible with the exact message format
    assert!(add_response.contains("Added"), "Missing Added message");
    assert!(add_response.contains("context"), "Missing context message");
    
    // Execute /context show to confirm file is present
    let show_response = chat.execute_command_with_timeout("/context show",Some(500))?;
    
    println!("📝 Context show response: {} bytes", show_response.len());
    println!("📝 SHOW RESPONSE:");
    println!("{}", show_response);
    println!("📝 END SHOW RESPONSE");
    
    // Verify file is present in context
    assert!(show_response.contains(test_file_path), "File not found in context show output");    
    // Remove file from context
    let remove_response = chat.execute_command_with_timeout(&format!("/context remove {}", test_file_path),Some(1000))?;
    
    println!("📝 Context remove response: {} bytes", remove_response.len());
    println!("📝 REMOVE RESPONSE:");
    println!("{}", remove_response);
    println!("📝 END REMOVE RESPONSE");
    
    // Verify file was removed successfully - be flexible with the exact message format
    assert!(remove_response.contains("Removed"), "Missing Removed message");
    assert!(add_response.contains("context"), "Missing context message");
    
    // Execute /context show to confirm file is gone
    let final_show_response = chat.execute_command_with_timeout("/context show",Some(500))?;
    
    println!("📝 Final context show response: {} bytes", final_show_response.len());
    println!("📝 FINAL SHOW RESPONSE:");
    println!("{}", final_show_response);
    println!("📝 END FINAL SHOW RESPONSE");
    
    // Verify file is no longer in context
    assert!(!final_show_response.contains(test_file_path), "File still found in context after removal");

    println!("✅ All context add/remove file content verified!");

    // Release the lock before cleanup
    drop(chat);

    // Clean up test file
    let _ = std::fs::remove_file(test_file_path);
    Ok(())
}

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_add_glob_pattern_file_context()-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context add *.py glob pattern command... | Description: Tests the <code> /context add</code> command with glob patterns to add multiple files matching a pattern and verify pattern-based context management");

    let test_file1_path = "/tmp/test_context_file1.py";
    let test_file2_path = "/tmp/test_context_file2.py";
    let test_file3_path = "/tmp/test_context_file.js"; // Non-matching file
    let glob_pattern = "/tmp/*.py";
    
    // Create test files
    std::fs::write(test_file1_path, "# Test Python file 1 for context\nprint('Hello from Python file 1')")?;
    std::fs::write(test_file2_path, "# Test Python file 2 for context\nprint('Hello from Python file 2')")?;
    std::fs::write(test_file3_path, "// Test JavaScript file\nconsole.log('Hello from JS file');")?;

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Add glob pattern to context
    let add_response = chat.execute_command_with_timeout(&format!("/context add {}", glob_pattern),Some(3000))?;
    
    println!("📝 Context add response: {} bytes", add_response.len());
    println!("📝 ADD RESPONSE:");
    println!("{}", add_response);
    println!("📝 END ADD RESPONSE");
    
    // Verify glob pattern was added successfully - be flexible with the exact message format
    assert!(add_response.contains("Added"), "Missing Added message");
    assert!(add_response.contains("context"), "Missing context message");
    
    // Execute /context show to confirm pattern matches files
    let show_response = chat.execute_command_with_timeout("/context show",Some(500))?;
    
    println!("📝 Context show response: {} bytes", show_response.len());
    println!("📝 SHOW RESPONSE:");
    println!("{}", show_response);
    println!("📝 END SHOW RESPONSE");
    
    // Verify that the Python files are present in context (glob pattern matched them)
    assert!(show_response.contains(test_file1_path) && show_response.contains(test_file2_path), "Python files not found in context show output");
    assert!(!show_response.contains(test_file3_path), "JavaScript file should not be matched by .py pattern");

    // Remove glob pattern from context
    let remove_response = chat.execute_command_with_timeout(&format!("/context remove {}", glob_pattern),Some(1000))?;
    
    println!("📝 Context remove response: {} bytes", remove_response.len());
    println!("📝 REMOVE RESPONSE:");
    println!("{}", remove_response);
    println!("📝 END REMOVE RESPONSE");
    
    // Verify glob pattern was removed successfully - be flexible with the exact message format
    assert!(remove_response.contains("Removed"), "Missing Removed message");
    assert!(add_response.contains("context"), "Missing context message");
    
    // Execute /context show to confirm glob pattern is gone
    let final_show_response = chat.execute_command_with_timeout("/context show",Some(1000))?;
    
    println!("📝 Final context show response: {} bytes", final_show_response.len());
    println!("📝 FINAL SHOW RESPONSE:");
    println!("{}", final_show_response);
    println!("📝 END FINAL SHOW RESPONSE");
    
    // Verify glob pattern is no longer in context
    assert!(!final_show_response.contains(glob_pattern), "Glob pattern still found in context after removal");

    println!("✅ All context glob pattern content verified!");

    // Release the lock before cleanup
    drop(chat);

    // Clean up test file
    let _ = std::fs::remove_file(test_file1_path);
    let _ = std::fs::remove_file(test_file2_path);
    let _ = std::fs::remove_file(test_file3_path);
    
    Ok(())
}

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_add_remove_multiple_file_context()-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context add <filename1> <filename2> <filename3> command and /context remove <filename1> <filename2> <filename3>... | Description: Tests the <code> /context add</code> command with multiple files to verify batch context operations and <code> /context remove</code> command with multiple files to verify");   
    let test_file1_path = "/tmp/test_context_file1.py";
    let test_file2_path = "/tmp/test_context_file2.py";
    let test_file3_path = "/tmp/test_context_file.js";
    
    // Create test files
    std::fs::write(test_file1_path, "# Test Python file 1 for context\nprint('Hello from Python file 1')")?;
    std::fs::write(test_file2_path, "# Test Python file 2 for context\nprint('Hello from Python file 2')")?;
    std::fs::write(test_file3_path, "// Test JavaScript file\nconsole.log('Hello from JS file');")?;

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    // Add multiple files to context in one command
    let add_response = chat.execute_command_with_timeout(&format!("/context add {} {} {}", test_file1_path, test_file2_path, test_file3_path),Some(1000))?;
    
    println!("📝 Context add response: {} bytes", add_response.len());
    println!("📝 ADD RESPONSE:");
    println!("{}", add_response);
    println!("📝 END ADD RESPONSE");
    
    // Verify files were added successfully - be flexible with the exact message format
    assert!(add_response.contains("Added"), "Missing Added message");
    
    // Execute /context show to confirm files are present
    let show_response = chat.execute_command_with_timeout("/context show",Some(500))?;
    
    println!("📝 Context show response: {} bytes", show_response.len());
    println!("📝 SHOW RESPONSE:");
    println!("{}", show_response);
    println!("📝 END SHOW RESPONSE");
    
    // Verify all files are present in context
    assert!(show_response.contains(test_file1_path), "Python file not found in context show output");
    assert!(show_response.contains(test_file2_path), "JavaScript file not found in context show output");
    assert!(show_response.contains(test_file3_path), "Text file not found in context show output");

    // Remove multiple files from context
    let remove_response = chat.execute_command_with_timeout(&format!("/context remove {} {} {}", test_file1_path, test_file2_path, test_file3_path),Some(1000))?;
    
    println!("📝 Context remove response: {} bytes", remove_response.len());
    println!("📝 REMOVE RESPONSE:");
    println!("{}", remove_response);
    println!("📝 END REMOVE RESPONSE");
    
    // Verify files were removed successfully - be flexible with the exact message format
    assert!(remove_response.contains("Removed"), "Missing Removed message");
    
    // Execute /context show to confirm files are gone
    let final_show_response = chat.execute_command_with_timeout("/context show",Some(500))?;
    
    println!("📝 Final context show response: {} bytes", final_show_response.len());
    println!("📝 FINAL SHOW RESPONSE:");
    println!("{}", final_show_response);
    println!("📝 END FINAL SHOW RESPONSE");
    
    // Verify files are no longer in context
    assert!(!final_show_response.contains(test_file1_path), "Python file still found in context after removal");
    assert!(!final_show_response.contains(test_file2_path), "JavaScript file still found in context after removal");
    assert!(!final_show_response.contains(test_file3_path), "Text file still found in context after removal");

    println!("✅ All context add/remove multiple file content verified!");

    // Release the lock before cleanup
    drop(chat);

    // Clean up test file
    let _ = std::fs::remove_file(test_file1_path);
    let _ = std::fs::remove_file(test_file2_path);
    let _ = std::fs::remove_file(test_file3_path);    
    
    Ok(())
}

#[test]
#[cfg(all(feature = "context", feature = "sanity"))]
fn test_clear_context_command()-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /context clear command... | Description: Tests the <code> /context clear</code> command to remove all files from context and verify the context is completely cleared");

    let test_file_path = "/tmp/test_context_file.py";
    
    // Create test files
    std::fs::write(test_file_path, "# Test Python file 1 for context\nprint('Hello from Python file 1')")?;

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Add multiple files to context
    let add_response = chat.execute_command_with_timeout(&format!("/context add {}", test_file_path),Some(1000))?;
    
    println!("📝 Context add response: {} bytes", add_response.len());
    println!("📝 ADD RESPONSE:");
    println!("{}", add_response);
    println!("📝 END ADD RESPONSE");
    
    // Verify files were added successfully - be flexible with the exact message format
    assert!(add_response.contains("Added"), "Missing Added message");
    assert!(add_response.contains("context"), "Missing context message");
    
    // Execute /context show to confirm files are present
    let show_response = chat.execute_command_with_timeout("/context show",Some(500))?;
    
    println!("📝 Context show response: {} bytes", show_response.len());
    println!("📝 SHOW RESPONSE:");
    println!("{}", show_response);
    println!("📝 END SHOW RESPONSE");
    
    // Verify files are present in context
    assert!(show_response.contains(test_file_path), "Python file not found in context show output");
    
    // Execute /context clear to remove all files
    let clear_response = chat.execute_command_with_timeout("/context clear",Some(500))?;
    
    println!("📝 Context clear response: {} bytes", clear_response.len());
    println!("📝 CLEAR RESPONSE:");
    println!("{}", clear_response);
    println!("📝 END CLEAR RESPONSE");
    
    // Verify context was cleared successfully
    assert!(clear_response.contains("Cleared context"), "Missing Cleared context message");
    
    // Execute /context show to confirm no files remain
    let final_show_response = chat.execute_command_with_timeout("/context show",Some(500))?;
    
    println!("📝 Final context show response: {} bytes", final_show_response.len());
    println!("📝 FINAL SHOW RESPONSE:");
    println!("{}", final_show_response);
    println!("📝 END FINAL SHOW RESPONSE");
    
    // Verify no files remain in context
    assert!(!final_show_response.contains(test_file_path), "Python file still found in context after clear");
    assert!(final_show_response.contains("Agent (kiro_default)"), "Missing Agent (kiro_default) section");
    assert!(final_show_response.contains("No files in the current directory matched the rules above"), "Missing empty context indicator");

    println!("✅ Clean context command content verified!");

    drop(chat);

    // Clean up test file
    let _ = std::fs::remove_file(test_file_path);
    println!("✅ Cleaned up test file");
    
    Ok(())
}
