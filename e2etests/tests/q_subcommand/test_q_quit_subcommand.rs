#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "q_subcommand", feature = "sanity"))]
fn test_q_quit_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\n🔍 Testing kiro settings kiro quit subcommand | Description: Tests the <code>kiro quit </code> subcommand to validate whether it quit the kiro app."
    );
    // Launch Amazon Q app.
    println!("Launching Kiro-cli...");
    let launch_response = q_chat_helper::execute_q_subcommand("q", &["launch"])?;
    println!("📝 Debug response: {} bytes", launch_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", launch_response);
    println!("📝 END OUTPUT");

    assert!(launch_response.contains("Opening Kiro CLI dashboard"),"Missing amazon Kiro CLI opening message");

    // Quit Amazon q app.
    println!("Quitting Kiro CLI...");
    let quit_response = q_chat_helper::execute_q_subcommand("q", &["quit"])?;
    println!("📝 Debug response: {} bytes", quit_response.len());
    println!("📝 FULL OUTPUT:");
    println!("{}", quit_response);
    println!("📝 END OUTPUT");

    assert!(quit_response.contains("Quitting Kiro CLI app"), "Missing Kiro CLI quit message");
    Ok(())

}