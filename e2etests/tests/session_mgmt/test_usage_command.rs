#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

/// Tests the /usage command to display current context window usage
/// Verifies token usage information, progress bar, breakdown sections, and Pro Tips
#[test]
#[cfg(all(feature = "usage", feature = "sanity"))]
fn test_usage_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /usage command... | Description: Tests the <code> /usage</code> command to display current context window usage. Verifies token usage information, progress bar, breakdown sections, and Pro Tips");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/usage",Some(2000))?;
    
    println!("📝 Tools response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Check if credit-based usage is supported
    if response.contains("Credit based usage is not supported for your subscription") {
        println!("✅ Credit-based usage not supported - test passed with expected message");
        assert!(response.contains("Credit based usage is not supported"), "Missing expected unsupported message");
    } else if response.contains("Current context window"){
        // Verify context window information for supported subscriptions
        assert!(response.contains("Current context window"), "Missing context window header");
        assert!(response.contains("tokens"), "Missing tokens used information");
        println!("✅ Found context window and token usage information");
        
        // Verify progress bar
        assert!(response.contains("%"), "Missing percentage display");
        println!("✅ Found progress bar with percentage");
        
        // Verify token breakdown sections
        assert!(response.contains(" Context files:"), "Missing Context files section");
        assert!(response.contains(" Tools:"), "Missing Tools section");
        assert!(response.contains(" Kiro responses:"), "Missing Kiro responses section");
        assert!(response.contains(" Your prompts:"), "Missing Your prompts section");
        println!("✅ Found all token breakdown sections");
        
        // Verify token counts and percentages format
        assert!(response.contains("tokens ("), "Missing token count format");
        assert!(response.contains("%)"), "Missing percentage format in breakdown");
        println!("✅ Verified token count and percentage format");
        
        // Verify Pro Tips section
        assert!(response.contains(" Pro Tips:"), "Missing Pro Tips section");
        println!("✅ Found Pro Tips section");
        
        // Verify specific tip commands
        assert!(response.contains("/compact"), "Missing /compact command tip");
        assert!(response.contains("/clear"), "Missing /clear command tip");
        assert!(response.contains("/context show"), "Missing /context show command tip");
        println!("✅ Found all command tips: /compact, /clear, /context show");
    } else {
        assert!(response.contains("Upgrade to Kiro for better usage insights through"), "Missing upgrade message");
    }
    println!("✅ All usage content verified!");
    println!("✅ Test completed successfully");
    
    drop(chat);
    Ok(())
}

// Tests the /usage --help command to display help information for the usage command
// Verifies Usage section, Options section, and help flags (-h, --help)
#[test]
#[cfg(all(feature = "usage", feature = "sanity"))]
fn test_usage_help_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /usage --help command... | Description: Tests the <code> /usage --help</code> command to display help information for the usage command. Verifies Usage section, Options section, and help flags (-h, --help)");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
     
    let response = chat.execute_command_with_timeout("/usage --help",Some(2000))?;
    
    println!("📝 Usage help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    // Verify Usage section
    assert!(response.contains("Usage:"), "Missing Usage section");

    assert!(response.contains("/usage"), "Missing /usage command in usage section");
    println!("✅ Found Usage section with /usage command");
    
    // Verify Options section
    assert!(response.contains("Options:"), "Missing Options section");
    println!("✅ Found Options section");
    
    // Verify help flags
    assert!(response.contains("-h") &&  response.contains("--help") && response.contains("Print help"), "Missing -h, --help flags");
    println!("✅ Found help flags: -h, --help with description");
    
    println!("✅ All usage help content verified!");
    
    println!("✅ Test completed successfully");
    
    drop(chat);

    Ok(())
}

/// Tests the /usage -h command (short form of --help)
/// Verifies Usage section, Options section, and help flags (-h, --help)
#[test]
#[cfg(all(feature = "usage", feature = "sanity"))]
fn test_usage_h_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing /usage -h command... | Description: Tests the <code> /usage -h</code> command (short form of --help). Verifies Usage section, Options section, and help flags (-h, --help)");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    let response = chat.execute_command_with_timeout("/usage -h",Some(2000))?;
    
    println!("📝 Usage help response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    
    // Verify Usage section
    assert!(response.contains("Usage:"), "Missing Usage section");
    assert!(response.contains("/usage"), "Missing /usage command in usage section");
    println!("✅ Found Usage section with /usage command");
    
    // Verify Options section
    assert!(response.contains("Options:"), "Missing Options section");
    println!("✅ Found Options section");
    
    // Verify help flags
    assert!(response.contains("-h") &&  response.contains("--help") && response.contains("Print help"), "Missing -h, --help flags");
    println!("✅ Found help flags: -h, --help with description");
    
    println!("✅ All usage help content verified!");
    
    println!("✅ Test completed successfully");
    
    drop(chat);

    Ok(())
}