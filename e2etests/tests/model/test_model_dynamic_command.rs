#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "model", feature = "sanity"))]
fn test_model_dynamic_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /model command with dynamic selection... | Description: Tests the <code> /model</code> command interactive selection interface to choose different models and verify selection confirmation");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    // Execute /model command to get list
    let model_response = chat.execute_command_with_timeout("/model",Some(2000))?;
    
    println!("📝 Model response: {} bytes", model_response.len());
    println!("📝 MODEL RESPONSE:");
    println!("{}", model_response);
    println!("📝 END MODEL RESPONSE");
    
    // Helper function to strip ANSI color codes
    let strip_ansi = |s: &str| -> String {
        let mut result = String::new();
        let mut in_escape = false;
        for c in s.chars() {
            if c == '\x1b' {
                in_escape = true;
            } else if in_escape && c == 'm' {
                in_escape = false;
            } else if !in_escape {
                result.push(c);
            }
        }
        result
    };
    
    // Parse available models from response
    let mut models = Vec::new();
    
    for line in model_response.lines() {
        let trimmed_line = line.trim();
        let cleaned_line = strip_ansi(trimmed_line);
        
        // Parse model lines directly - look for lines with model names
        if cleaned_line.contains("claude-") || cleaned_line.contains("qwen") || cleaned_line.contains("Auto") {
            let model_name = cleaned_line
                .split('|')
                .next()
                .unwrap_or(&cleaned_line)
                .replace("❯", "")
                .replace("(current)", "")
                .trim()
                .to_string();
            
            println!("\n🔍 Extracted model: '{}'", model_name);
            if !model_name.is_empty() {
                models.push(model_name);
            }
        }
    }
    
    println!("📝 Found models: {:?}", models);
    assert!(!models.is_empty(), "No models found in response");
    
    // Send down arrow to select different model
    let selection_response = chat.send_key_input("\x1b[B")?;
    
    println!("📝 Selection response: {} bytes", selection_response.len());
    println!("📝 SELECTION RESPONSE:");
    println!("{}", selection_response);
    println!("📝 END SELECTION RESPONSE");
    
    // Find which model is now selected (has ❯ marker)
    let selected_model = selection_response.lines()
        .find(|line| {
            let cleaned = strip_ansi(line);
            cleaned.contains("❯")
        })
        .map(|line| {
            let cleaned = strip_ansi(line.trim());
            cleaned
                .split('|')
                .next()
                .unwrap_or(&cleaned)
                .replace("❯", "")
                .replace("(current)", "")
                .trim()
                .to_string()
        })
        .unwrap_or_else(|| models.get(1).unwrap_or(&models[0]).clone());
    
    println!("📝 Selected model: {}", selected_model);
    
    // Send Enter to confirm
    let confirm_response = chat.send_key_input("\r")?;
    
    println!("📝 Confirm response: {} bytes", confirm_response.len());
    println!("📝 CONFIRM RESPONSE:");
    println!("{}", confirm_response);
    println!("📝 END CONFIRM RESPONSE");
    
    // Verify selection with dynamic model name
    assert!(confirm_response.contains(&format!("Using {}", selected_model)), 
           "Missing confirmation for selected model: {}", selected_model);
    println!("✅ Confirmed selection of: {}", selected_model);
    
    drop(chat);

    Ok(())
}


#[test]
#[cfg(all(feature = "model", feature = "sanity"))]
fn test_model_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /model --help command... | Description: Tests the <code> /model --help</code> command to display help information for model selection functionality");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/model --help",Some(500))?;
    
    println!("📝 Model help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify Usage section
    assert!(response.contains("Usage:"), "Missing Usage section");
    assert!(response.contains("/model"), "Missing /model command in usage section");
    println!("✅ Found Usage section with /model command");
    
    // Verify Options section
    assert!(response.contains("Options:"), "Missing Options section");
    println!("✅ Found Options section");
    
    // Verify help flags
    assert!(response.contains("-h") &&  response.contains("--help"), "Missing -h, --help flags");
    println!("✅ Found help flags: -h, --help with Print help description");
    
    println!("✅ All model help content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "model", feature = "sanity"))]
fn test_model_h_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /model -h command... | Description: Tests the <code> /model -h</code> command (short form) to display help information for model selection functionality");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/model -h",Some(500))?;
    
    println!("📝 Model help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify Usage section
    assert!(response.contains("Usage:"), "Missing Usage section");
    assert!(response.contains("/model"), "Missing /model command in usage section");
    println!("✅ Found Usage section with /model command");
    
    // Verify Options section
    assert!(response.contains("Options:"), "Missing Options section");
    println!("✅ Found Options section");
    
    // Verify help flags
    assert!(response.contains("-h") &&  response.contains("--help"), "Missing -h, --help flags");
    println!("✅ Found help flags: -h, --help with Print help description");
    
    println!("✅ All model help content verified!");
    
    drop(chat);

    Ok(())
}

#[test]
#[cfg(all(feature = "model", feature = "sanity"))]
fn test_model_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /model command... | Description: Tests the <code> /model </code> command to check it shows the auto model select");

    let session = q_chat_helper::get_new_chat_session()?;
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/model",Some(500))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify Usage section
    assert!(response.contains("Press"), "Expected 'Press' in response.");
    assert!(response.contains("Auto"), "Expected 'Auto' in response.");

    println!("✅ Model content verified for Auto");
    
    drop(chat);

    Ok(())
}