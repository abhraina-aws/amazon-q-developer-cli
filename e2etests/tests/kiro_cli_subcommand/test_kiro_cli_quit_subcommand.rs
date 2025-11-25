#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "kiro_cli_subcommand", feature = "sanity"))]
fn test_kiro_cli_quit_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli settings kiro quit subcommand | Description: Tests the <code>kiro-cli quit </code> subcommand to validate whether it quit the kiro-cli app.");
    // Launch kiro-cli app.
    println!("Launching Kiro-cli...");
    let launch_response = q_chat_helper::execute_q_subcommand("kiro-cli", &["launch"])?;

    println!("📝 Debug response: {} bytes", launch_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", launch_response);
    println!("📝 END OUTPUT");

    if launch_response.contains("minimal mode") {
        assert!(launch_response.contains("minimal mode"),"Expected 'minimal mode' in response.");
    } else {
         assert!(launch_response.contains("Opening Kiro CLI dashboard"),"Missing amazon Kiro CLI opening message");
    }

    // Quit kiro-cli app.
    let quit_response = q_chat_helper::execute_q_subcommand("kiro-cli", &["quit"])?;

    println!("📝 Debug response: {} bytes", quit_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", quit_response);
    println!("📝 END OUTPUT");

    assert!(quit_response.contains("Quitting Kiro CLI app"), "Missing Kiro CLI quit message");
    Ok(())

}