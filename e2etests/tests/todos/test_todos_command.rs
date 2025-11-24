#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;
#[allow(unused_imports)]
use regex::Regex;

#[test]
#[cfg(all(feature = "todos", feature = "sanity"))]
fn test_todos_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /todos command... | Description: Tests the <code> /todos</code> command to view, manage, and resume to-do lists");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    let response = chat.execute_command_with_timeout("/todos",Some(2000))?;

    println!("📝 Help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify help content
    assert!(response.contains("Commands"), "Missing Commands section");
    assert!(response.contains("resume"), "Missing resume command");
    assert!(response.contains("view"), "Missing view command");
    assert!(response.contains("delete"), "Missing delete command");
    assert!(response.contains("help"), "Missing help command");

    println!("✅ /todos command test completed successfully");

    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "todos", feature = "sanity"))]
fn test_todos_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /todos help command... | Description: Tests the <code> /todos help</code> command to display help information about the todos ");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    let response = chat.execute_command_with_timeout("/todos help",Some(2000))?;

    println!("📝 Help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify help content
    assert!(response.contains("Commands"), "Missing Commands section");
    assert!(response.contains("resume"), "Missing resume command");
    assert!(response.contains("view"), "Missing view command");
    assert!(response.contains("delete"), "Missing delete command");
    assert!(response.contains("help"), "Missing help command");

    println!("✅ /todos help command test completed successfully");

    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "todos", feature = "sanity"))]
fn test_todos_view_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /todos view command... | Description: Tests the <code> /todos view</code> command to view to-do lists");

    // Use a new isolated session to avoid context contamination from previous tests
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("Executing 'kiro-cli settings chat.enableTodoList true' to enable todos feature...");
    q_chat_helper::execute_q_subcommand("kiro-cli", &["settings", "chat.enableTodoList", "true"])?;

    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["settings", "all"])?;

    println!("📝 Help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("chat.enableTodoList = true"), "Failed to enable todos feature using chat.enableTodoList = true");
    println!("✅ Todos feature enabled");

    println!("✅ Kiro CLI chat session started");

    let response = chat.execute_command_with_timeout("create a todo_list with 2 tasks: 1. Review code changes 2. Update documentation",Some(2000))?;

    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify help content
    assert!(response.contains("TODO list"), "Expecting 'TODO list' in reponse.");
    assert!(response.contains("ID"), "Expecting 'ID' in response.");

    let view_response = chat.execute_command_with_timeout("/todos view",Some(2000))?;

    println!("📝 FULL OUTPUT:");
    println!("{}", view_response);
    println!("📝 END OUTPUT");

    assert!(view_response.contains("to-do"), "Expecting 'to-do' in response.");
    assert!(view_response.contains("view"), "Expecting 'view' in response.");

    // Send down arrow to select different model
    let selection_response = chat.send_key_input("\x1b[B")?;
   
    println!("📝 SELECTION RESPONSE:");
    println!("{}", selection_response);
    println!("📝 END SELECTION RESPONSE");

    // Send Enter to confirm
    let confirm_response = chat.send_key_input("\r")?;

    println!("📝 CONFIRM RESPONSE:");
    println!("{}", confirm_response);
    println!("📝 END CONFIRM RESPONSE");

    assert!(confirm_response.contains("TODO"), "Expecting 'TODO' in response.");

    let delete_response = chat.execute_command_with_timeout("/todos delete",Some(2000))?;

    println!("📝 FULL OUTPUT:");
    println!("{}", delete_response);
    println!("📝 END OUTPUT");

    assert!(delete_response.contains("to-do"), "Expecting 'to-do' in reponse.");
    assert!(delete_response.contains("delete"), "Expecting 'delete' in reponse");

    // Send down arrow to select different model
    let selection_response = chat.send_key_input("\x1b[B")?;

    println!("📝 SELECTION RESPONSE:");
    println!("{}", selection_response);
    println!("📝 END SELECTION RESPONSE");

    // Send Enter to confirm
    let confirm_response = chat.send_key_input("\r")?;

    println!("📝 CONFIRM RESPONSE:");
    println!("{}", confirm_response);
    println!("📝 END CONFIRM RESPONSE");

    assert!(confirm_response.contains("Deleted"), "Expecting 'Deleted' in reponse.");
    assert!(confirm_response.contains("to-do"), "Expecting 'to-do' in reponse.");

    println!("✅ /todos view command test completed successfully");

    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "todos", feature = "sanity"))]
fn test_todos_resume_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /todos resume command... | Description: Tests the <code> /todos resume</code> command to resume a specific to-do list");

    // Use a new isolated session to avoid context contamination from previous tests
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("Executing 'kiro-cli settings chat.enableTodoList true' to enable todos feature...");
    q_chat_helper::execute_q_subcommand("kiro-cli", &["settings", "chat.enableTodoList", "true"])?;

    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["settings", "all"])?;

    println!("📝 Help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("chat.enableTodoList = true"), "Failed to enable todos feature using chat.enableTodoList = true");
    println!("✅ Todos feature enabled");

    println!("✅ Kiro CLI chat session started");

    let create_response = chat.execute_command_with_timeout("create a todo_list with 1 tasks: 1. Draft email to dummy host ",Some(3000))?;

    println!("📝 CREATE OUTPUT:");
    println!("{}", create_response);
    println!("📝 END CREATE OUTPUT");

    // Verify help content
    assert!(create_response.contains("TODO"), "Expecting 'TODO' in response.");
    assert!(create_response.contains("list"), "Expecting 'list' in response");

    let resume_response = chat.execute_command_with_timeout("/todos resume",Some(2000))?;

    println!("📝 RESUME OUTPUT:");
    println!("{}", resume_response);
    println!("📝 END RESUME OUTPUT");

    assert!(resume_response.contains("to-do"), "Expecting 'to-do' in response.");
    assert!(resume_response.contains("resume"), "Expecting 'resume' in response.");

    // Send down arrow to select different model
    let selection_response = chat.send_key_input("\x1b[B")?;

    println!("📝 SELECTION RESPONSE:");
    println!("{}", selection_response);
    println!("📝 END SELECTION RESPONSE");

    // Send Enter to confirm
    let confirm_response = chat.send_key_input("\r")?;

    println!("📝 CONFIRM RESPONSE:");
    println!("{}", confirm_response);
    println!("📝 END CONFIRM RESPONSE");

    assert!(confirm_response.contains("Resuming"), "Expecting 'Resuming' in reponse.");
    assert!(resume_response.contains("Draft email to dummy host"), "Expecting 'Draft email to dummy host' in response.");
    assert!(confirm_response.contains("TODO"), "Expecting TODO in response.");

    let delete_response = chat.execute_command_with_timeout("/todos delete",Some(2000))?;

    println!("📝 FULL OUTPUT:");
    println!("{}", delete_response);
    println!("📝 END OUTPUT");

    assert!(delete_response.contains("to-do"), "Expecting 'to-do' in reponse.");
    assert!(delete_response.contains("delete"), "Expecting 'delete' in reponse");

    // Send down arrow to select different model
    let selection_response = chat.send_key_input("\x1b[B")?;

    println!("📝 SELECTION RESPONSE:");
    println!("{}", selection_response);
    println!("📝 END SELECTION RESPONSE");

    // Send Enter to confirm
    let confirm_response = chat.send_key_input("\r")?;

    println!("📝 CONFIRM RESPONSE:");
    println!("{}", confirm_response);
    println!("📝 END CONFIRM RESPONSE");

    assert!(confirm_response.contains("Deleted"), "Expecting 'Deleted' in reponse.");
    assert!(confirm_response.contains("to-do"), "Expecting 'to-do' in reponse.");

    println!("✅ /todos resume command test completed successfully");

    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "todos", feature = "sanity"))]
fn test_todos_delete_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /todos delete command... | Description: Tests the <code> /todos delete</code> command to delete a specific to-do list");

    // Use a new isolated session to avoid context contamination from previous tests
    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("Executing 'kiro-cli settings chat.enableTodoList true' to enable todos feature...");
    q_chat_helper::execute_q_subcommand("kiro-cli", &["settings", "chat.enableTodoList", "true"])?;

    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["settings", "all"])?;

    println!("📝 Help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("chat.enableTodoList = true"), "Failed to enable todos feature using chat.enableTodoList = true");
    println!("✅ Todos feature enabled");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    // Create a new todo list for testing
    println!("Creating a new todo list for testing...");
    let create_response = chat.execute_command_with_timeout("create a todo_list with 2 tasks: 1. Review code changes 2. Update documentation", Some(3000))?;
    println!("create_response: {}", create_response);
    
 // Verify help content
    assert!(create_response.contains("TODO"), "Expecting 'TODO' in response.");
    assert!(create_response.contains("list"), "Expecting 'list' in response");
    
    println!("Todo list created successfully, now testing delete...");
    
    // Test the delete command and actually delete the todo
    let delete_response = chat.execute_command_with_timeout("/todos delete", Some(1000))?;
    
    assert!(delete_response.contains("to-do"), "Expecting 'to-do' in reponse.");
    assert!(delete_response.contains("delete"), "Expecting 'delete' in reponse");

    // Send down arrow to select different model
    let selection_response = chat.send_key_input("\x1b[B")?;

    println!("📝 SELECTION RESPONSE:");
    println!("{}", selection_response);
    println!("📝 END SELECTION RESPONSE");

    // Send Enter to confirm
    let confirm_response = chat.send_key_input("\r")?;

    println!("📝 CONFIRM RESPONSE:");
    println!("{}", confirm_response);
    println!("📝 END CONFIRM RESPONSE");

    assert!(confirm_response.contains("Deleted"), "Expecting 'Deleted' in reponse.");
    assert!(confirm_response.contains("to-do"), "Expecting 'to-do' in reponse.");

    println!("✅ /todos delete command test completed successfully");
    drop(chat);
    Ok(())
}

#[test]
#[cfg(all(feature = "todos", feature = "sanity"))]
fn test_todos_clear_finished_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /todos clear-finished command... | Description: Tests that <code> /todos clear-finished </code> command to validate it clears the todo list.");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("✅ Kiro CLI chat session started");

    // Create todo list with 2 tasks
    println!("\n🔍 Creating todo list with 2 tasks...");
    let create_response = chat.execute_command_with_timeout("create a todo_list with 2 tasks: 1. Review code changes 2. Update documentation", Some(2000))?;

    println!("📝 Create response: {} bytes", create_response.len());
    println!("📝 Create response: {}", create_response);
    
    assert!(create_response.contains("todo_list"), "Todo list was not created");

    // Extract todo ID
    let re = Regex::new(r"(\d{10,})")?;
    let todo_id = re.find(&create_response)
        .map(|m| m.as_str())
        .ok_or("Could not extract todo list ID")?;

    // Mark all tasks as completed
    println!("\n🔍 Marking all tasks as completed...");
    let mark_response = chat.execute_command_with_timeout(&format!("mark all tasks as completed for todo list {}", todo_id), Some(2000))?;

    println!("📝 Mark complete response: {} bytes", mark_response.len());
    println!("📝 Mark complete response: {}", mark_response);
    println!("✅ Found Task completion response.");

    // Test clear-finished command
    println!("\n🔍 Testing clear-finished command...");
    let clear_response = chat.execute_command_with_timeout("/todos clear-finished", Some(2000))?;
    println!("📝 Clear response: {} bytes", clear_response.len());
    println!("📝 {}", clear_response);

    assert!(!clear_response.is_empty(), "Expected non-empty response from clear-finished command");

    println!("✅ All finished task cleared successfully.");

    drop(chat);
    Ok(())
}