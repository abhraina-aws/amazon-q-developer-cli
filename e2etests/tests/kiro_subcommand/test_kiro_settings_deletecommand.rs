#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_subcommand", feature = "sanity"))]
fn test_kiro_setting_delete_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\n🔍 Testing kiro settings --delete <KEY> <VALUE>... | Description: Tests the <code>kiro settings --delete <KEY> </code> subcommand to validate DELETE content."
    );
// Get all the settings
    let response = q_chat_helper::execute_q_subcommand("q", &["settings", "list"])?;

    println!("📝 List response: {} bytes", response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    // Find first setting (parse key = value format)
    for line in response.lines() {
        if line.contains(" = ") {
            let parts: Vec<&str> = line.split(" = ").collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                
                println!("📝 Found setting: {} = {}", key, value);
                
                // Delete the setting
                let delete_response = q_chat_helper::execute_q_subcommand("q", &["settings", "--delete", key])?;
                println!("📝 Delete response: {}", delete_response);
                
                // Restore the setting
                let restore_response = q_chat_helper::execute_q_subcommand("q", &["settings", key, value])?;
                println!("📝 Restore response: {}", restore_response);
                
                assert!(delete_response.contains("Removing"), "Missing delete confirmation");
                break; // Only test first setting
            }
        }
    }

    Ok(())

}
