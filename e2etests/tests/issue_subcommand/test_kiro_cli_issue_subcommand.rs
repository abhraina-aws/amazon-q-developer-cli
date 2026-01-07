#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "issue_subcommand", feature = "sanity"))]
fn test_kiro_cli_issue_help_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli issue --help ... | Description: Tests the <code> kiro-cli issue --help </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli issue --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["issue", "--help"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Usage:"), "Expected 'Usage:' in the output");
    assert!(response.contains("Options:"), "Expected 'Options:' in the output");
    assert!(response.contains("-h"), "Expected '-h' in the output");
    assert!(response.contains("--help"), "Expected '--help' in the output");

    println!("✅ Kiro Cli issue --help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "issue_subcommand", feature = "sanity"))]
fn test_kiro_cli_issue_h_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli issue -h ... | Description: Tests the <code> kiro-cli issue -h </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli issue -h' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["issue", "-h"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Usage:"), "Expected 'Usage:' in the output");
    assert!(response.contains("Options:"), "Expected 'Options:' in the output");
    assert!(response.contains("-h"), "Expected '-h' in the output");
    assert!(response.contains("--help"), "Expected '--help' in the output");

    println!("✅ Kiro Cli issue -h subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "issue_subcommand", feature = "sanity"))]
fn test_kiro_cli_issue_force_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli issue --force ... | Description: Tests the <code> kiro-cli issue --force </code> subcommand to verify interactive issue creation.");

    println!("\n🔍 Executing 'kiro-cli issue --force' subcommand...");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Execute the command with longer timeout
    let response = chat.execute_command_with_timeout("!kiro-cli issue --force", Some(1000))?;
    
    println!("📝 INITIAL OUTPUT:");
    println!("{}", response);
    
    // Check if we got the interactive prompt
    if response.contains("Issue Title") {
        println!("🔍 Detected interactive prompt, sending test title...");
        
        // Send the issue title
        let title_response = chat.send_key_input("Test issue from automated test")?;
        
        println!("📝 TITLE INPUT RESPONSE:");
        println!("{}", title_response);
        
        // Send Enter to confirm the input and wait longer for GitHub processing
        let enter_response = chat.send_key_input_with_timeout("\r", Some(1000))?;
        
        println!("📝 ENTER RESPONSE:");
        println!("{}", enter_response);
        
        // Wait additional time for GitHub redirect and read any remaining output
        std::thread::sleep(std::time::Duration::from_secs(3));
        let final_response = chat.send_key_input_with_timeout("", Some(1000))?;
        
        println!("📝 FINAL OUTPUT:");
        println!("{}", final_response);
        
        // Combine all outputs
        let combined_output = format!("{}{}{}{}", response, title_response, enter_response, final_response);
        
        // Basic success criteria
        assert!(!combined_output.contains("Error"), "Should not contain error messages");
        assert!(combined_output.contains("Test issue from automated test"), "Should contain our input text");
        
        // Check for GitHub redirect message
        if combined_output.contains("Heading over to GitHub") {
            println!("✅ Issue creation process completed with GitHub redirect!");
        } else if combined_output.contains("GitHub") || 
                  combined_output.contains("Issue created") ||
                  combined_output.contains("✔") {
            println!("✅ Issue creation process completed successfully!");
        } else {
            println!("ℹ️  Issue creation process completed (interactive input successful)");
            println!("🔍 Debug: Looking for 'Heading over to GitHub' in output...");
        }
    } else {
        // If no interactive prompt, check for other expected behaviors
        assert!(!response.contains("Error"), "Should not contain error messages");
        println!("ℹ️  Command executed without interactive prompt");
    }

    println!("✅ Kiro Cli issue --force subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "issue_subcommand", feature = "sanity"))]
fn test_kiro_cli_issue_f_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli issue -f ... | Description: Tests the <code> kiro-cli issue -f </code> subcommand to verify interactive issue creation. using -f ");

    println!("\n🔍 Executing 'kiro-cli issue -f' subcommand...");

    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    
    // Execute the command with longer timeout
    let response = chat.execute_command_with_timeout("!kiro-cli issue -f", Some(1000))?;
    
    println!("📝 INITIAL OUTPUT:");
    println!("{}", response);
    
    // Check if we got the interactive prompt
    if response.contains("Issue Title") {
        println!("🔍 Detected interactive prompt, sending test title...");
        
        // Send the issue title
        let title_response = chat.send_key_input("Test issue from automated test")?;
        
        println!("📝 TITLE INPUT RESPONSE:");
        println!("{}", title_response);
        
        // Send Enter to confirm the input and wait longer for GitHub processing
        let enter_response = chat.send_key_input_with_timeout("\r", Some(1000))?;
        
        println!("📝 ENTER RESPONSE:");
        println!("{}", enter_response);
        
        // Wait additional time for GitHub redirect and read any remaining output
        std::thread::sleep(std::time::Duration::from_secs(3));
        let final_response = chat.send_key_input_with_timeout("", Some(1000))?;
        
        println!("📝 FINAL OUTPUT:");
        println!("{}", final_response);
        
        // Combine all outputs
        let combined_output = format!("{}{}{}{}", response, title_response, enter_response, final_response);
        
        // Basic success criteria
        assert!(!combined_output.contains("Error"), "Should not contain error messages");
        assert!(combined_output.contains("Test issue from automated test"), "Should contain our input text");
        
        // Check for GitHub redirect message
        if combined_output.contains("Heading over to GitHub") {
            println!("✅ Issue creation process completed with GitHub redirect!");
        } else if combined_output.contains("GitHub") || 
                  combined_output.contains("Issue created") ||
                  combined_output.contains("✔") {
            println!("✅ Issue creation process completed successfully!");
        } else {
            println!("ℹ️  Issue creation process completed (interactive input successful)");
            println!("🔍 Debug: Looking for 'Heading over to GitHub' in output...");
        }
    } else {
        // If no interactive prompt, check for other expected behaviors
        assert!(!response.contains("Error"), "Should not contain error messages");
        println!("ℹ️  Command executed without interactive prompt");
    }

    println!("✅ Kiro Cli issue -f subcommand executed successfully!");
    
    Ok(())
}