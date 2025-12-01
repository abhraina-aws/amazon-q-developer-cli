#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_translate_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli translate subcommand... | Description: Tests the <code> kiro-cli translate </code> subcommand for Natural Language to Shell translation");
    
    let session = q_chat_helper::get_chat_session();
    let mut chat = session.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    println!("\n🔍 Executing 'kiro-cli translate' subcommand with input 'hello'...");
    
    // Use stdin function for translate subcommand
    println!("\n🔍 Testing kiro-cli translate subcommand to create and delete a project directory...");
    let response = q_chat_helper::execute_q_subcommand_with_stdin("kiro-cli", &["translate"], Some("print the create a file with name testkirocli in current working directory."))?;
    
    println!("📝 Translate response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");
    

    let insert_response = chat.send_key_input("\r")?;
    println!("Select response: {}", insert_response);

    // Verify translation output contains shell subcommand
    assert!(response.contains("touch"), "Expected 'touch' in response.");
    // assert!(response.contains("demoproject"), "Missing demoproject name");

   // now I want to delete the demoproject directory
    println!("\n🔍 Testing kiro-cli translate subcommand to delete created testkiro file...");
    let response = q_chat_helper::execute_q_subcommand_with_stdin("kiro-cli", &["translate"], Some("print the delete file testkirocli in curent working directory."))?;

    println!("📝 Translate response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Verify translation output contains shell subcommand
    assert!(response.contains("-delete"), "Expected '-delete' in reponse.");
    // assert!(response.contains("demoproject"), "Missing demoproject name");
    
    println!("✅ Translate subcommand executed successfully!");
    
    Ok(())
}
