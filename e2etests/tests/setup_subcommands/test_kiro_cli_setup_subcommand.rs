#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "setup_subcommands", feature = "sanity"))]
fn test_kiro_cli_setup_help_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli setup --help ... | Description: Tests the <code> kiro-cli setup --help  </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli setup --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["setup","--help"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Options:"), "Expected 'Options:' in the output");
    assert!(response.contains("dotfiles"), "Expected 'dotfiles' in the output");
    assert!(response.contains("input-method"), "Expected 'input-method' in the output");
    assert!(response.contains("no-confirm"), "Expected 'no-confirm' in the output");
    assert!(response.contains("force"), "Expected 'force' in the output");
    assert!(response.contains("global"), "Expected 'global' in the output");
    assert!(response.contains("verbose"), "Expected 'verbose' in the output");
    assert!(response.contains("help"), "Expected 'help' in the output");

    println!("✅ Kiro Cli setup --help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "setup_subcommands", feature = "sanity"))]
fn test_kiro_cli_setup_dotfiles_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli setup --dotfiles ... | Description: Tests the <code> kiro-cli setup --dotfiles </code> subcommand to verify dotfiles setup.");

    // Run inside chat session which has PTY for interactive prompts
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("\n🔍 Executing 'kiro-cli setup --dotfiles' subcommand in chat session...");
    let response = chat.execute_command_with_timeout("!kiro-cli setup --dotfiles", Some(500))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    let select_response = chat.send_key_input("\r")?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    println!("📝 SELECT RESPONSE:");
    println!("{}", select_response);
    println!("📝 END SELECT RESPONSE");
    
    assert!(response.contains("shell config"), "Expected 'shell config' in response.");

    println!("✅ Kiro Cli setup --dotfiles subcommand executed successfully!");
    
    drop(chat);
    Ok(())
}

#[test]
#[cfg(all(feature = "setup_subcommands", feature = "sanity"))]
fn test_kiro_cli_setup_input_method_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli setup --input-method ... | Description: Tests the <code> kiro-cli setup --input-method </code> subcommand to verify input method setup.");

    // Skip test on Linux only
    if cfg!(target_os = "linux") {
        println!("⚠️ Skipping test - running on Linux");
        return Ok(());
    }

    // Run inside chat session which has PTY for interactive prompts
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("\n🔍 Executing 'kiro-cli setup --input-method' subcommand in chat session...");
    let response = chat.execute_command_with_timeout("!kiro-cli setup --input-method", Some(1000))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    
    let select_response = chat.send_key_input("\r")?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    println!("📝 SELECT RESPONSE:");
    println!("{}", select_response);
    println!("📝 END SELECT RESPONSE");
    
    assert!(response.contains("input"), "Expected 'input' in response.");
    assert!(response.contains("enable support"), "Expected 'enable support' in response.");

    println!("✅ Kiro Cli setup --input-method subcommand executed successfully!");
    
    drop(chat);
    Ok(())
}

#[test]
#[cfg(all(feature = "setup_subcommands", feature = "sanity"))]
fn test_kiro_cli_setup_force_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli setup --force ... | Description: Tests the <code> kiro-cli setup --force </code> subcommand to verify kiro-cli force setup.");

    // Skip test on Linux only
    if cfg!(target_os = "linux") {
        println!("⚠️ Skipping test - running on Linux");
        return Ok(());
    }

    // Run inside chat session which has PTY for interactive prompts
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("\n🔍 Executing 'kiro-cli setup --dotfiles' subcommand in chat session...");
    let response = chat.execute_command_with_timeout("!kiro-cli setup --force", Some(500))?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    if response.contains("shell config") {
        let select_response = chat.send_key_input("\r")?;
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        assert!(response.contains("Do you want"), "Expected 'Do you want' in response.");
        assert!(response.contains("shell config"), "Expected 'shell config' in response.");
        println!("📝 SELECT RESPONSE:");
        println!("{}", select_response);
        println!("📝 END SELECT RESPONSE");

        if select_response.contains("terminals"){
            let terminal_response = chat.send_key_input("\r")?;
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("📝 SELECT RESPONSE:");
            println!("{}", terminal_response);
            println!("📝 END SELECT RESPONSE");
            assert!(terminal_response.contains("Do you want"), "Expected 'Do you want' in response.");
            assert!(terminal_response.contains("terminals"), "Expected 'terminals' in response.");

        }

    }
    println!("✅ Kiro Cli setup --input-method subcommand executed successfully!");
    
    drop(chat);
    Ok(())
}

#[test]
#[cfg(all(feature = "setup_subcommands", feature = "sanity"))]
fn test_kiro_cli_setup_help_shorthand_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli setup -h ... | Description: Tests the <code> kiro-cli setup -h  </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli setup --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["setup","-h"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Options:"), "Expected 'Options:' in the output");
    assert!(response.contains("dotfiles"), "Expected 'dotfiles' in the output");
    assert!(response.contains("input-method"), "Expected 'input-method' in the output");
    assert!(response.contains("no-confirm"), "Expected 'no-confirm' in the output");
    assert!(response.contains("force"), "Expected 'force' in the output");
    assert!(response.contains("global"), "Expected 'global' in the output");
    assert!(response.contains("verbose"), "Expected 'verbose' in the output");
    assert!(response.contains("help"), "Expected 'help' in the output");

    println!("✅ Kiro Cli setup -h subcommand executed successfully!");
    
    Ok(())
}