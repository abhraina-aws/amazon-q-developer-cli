#[allow(unused_imports)]
use q_cli_e2e_tests::q_chat_helper;

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_help_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init --help ... | Description: Tests the <code> kiro-cli init --help  </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli init --help' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","--help"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Options:"), "Expected 'Options:' in the output");
    assert!(response.contains("Arguments"), "Expected 'Arguments' in the output");
    assert!(response.contains("SHELL"), "Expected 'SHELL' in the output");

    assert!(response.contains("bash"), "Expected 'bash' in the output");
    assert!(response.contains("zsh"), "Expected 'zsh' in the output");
    assert!(response.contains("fish"), "Expected 'fish' in the output");

    assert!(response.contains("nu"), "Expected 'nu' in the output");
    assert!(response.contains("WHEN"), "Expected 'WHEN' in the output");
    assert!(response.contains("RCFILE"), "Expected 'RCFILE' in the output");

    assert!(response.contains("rcfile"), "Expected 'rcfile' in the output");

    assert!(response.contains("-v"), "Expected '-v' in the output");
    assert!(response.contains("--verbose"), "Expected '--verbose' in the output");

    assert!(response.contains("-h"), "Expected '-h' in the output");
    assert!(response.contains("--help"), "Expected '--help' in the output");

    println!("✅ Kiro Cli init --help subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_bash_pre_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init bash pre ... | Description: Tests the <code> kiro-cli init bash pre  </code> subcommand to verify bash pre init subcommand.");

    println!("\n🔍 Executing 'kiro-cli init bash pre' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","bash","pre"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("SHOULD_QTERM_LAUNCH"), "Expected 'SHOULD_QTERM_LAUNCH' in the output");
    assert!(response.contains("function __fig_source_bash_preexec"), "Expected 'function __fig_source_bash_preexec' in the output");
    assert!(response.contains("bash-preexec.sh"), "Expected 'bash-preexec.sh' in the output");

    assert!(response.contains("General Usage:"), "Expected 'General Usage:' in the output");
    assert!(response.contains("#!/usr/bin/env bash"), "Expected '#!/usr/bin/env bash' in the output");
    assert!(response.contains("https://github.com/rcaloras/bash-preexec"), "Expected 'https://github.com/rcaloras/bash-preexec' in the output");

    assert!(response.contains("#"), "Expected '#' in the output");

    println!("✅ Kiro Cli init bash pre subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_bash_post_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init bash post ... | Description: Tests the <code> kiro-cli init bash post  </code> subcommand to verify kiro-cli init bash post subcommand.");

    println!("\n🔍 Executing 'kiro-cli init bash post' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","bash","post"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Q_SHELL"), "Expected 'Q_SHELL' in the output");
    assert!(response.contains("bash-preexec.sh"), "Expected 'bash-preexec.sh' in the output");

    assert!(response.contains("General Usage:"), "Expected 'General Usage:' in the output");
    assert!(response.contains("https://github.com/rcaloras/bash-preexec"), "Expected 'https://github.com/rcaloras/bash-preexec' in the output");

    assert!(response.contains("#"), "Expected '#' in the output");

    println!("✅ Kiro Cli init bash post subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_zsh_pre_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init zsh pre ... | Description: Tests the <code> kiro-cli init zsh pre  </code> subcommand to verify kiro-cli init zsh pre subcommand.");

    println!("\n🔍 Executing 'kiro-cli init zsh pre' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","zsh","pre"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Q_SHELL"), "Expected 'Q_SHELL' in the output");
    assert!(response.contains("SHOULD_QTERM_LAUNCH"), "Expected 'SHOULD_QTERM_LAUNCH' in the output");

    assert!(response.contains("mkdir"), "Expected 'mkdir' in the output");
    assert!(response.contains("add"), "Expected 'add' in the output");

    assert!(response.contains("#"), "Expected '#' in the output");

    println!("✅ Kiro Cli init zsh pre subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_zsh_post_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init zsh post ... | Description: Tests the <code> kiro-cli init zsh post  </code> subcommand to verify kiro-cli init zsh pre subcommand.");

    println!("\n🔍 Executing 'kiro-cli init zsh post' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","zsh","post"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Q_SHELL"), "Expected 'Q_SHELL' in the output");
    assert!(response.contains("Global Configuration Variables"), "Expected 'Global Configuration Variables' in the output");

    assert!(response.contains("Utility Functions"), "Expected 'Utility Functions' in the output");
    assert!(response.contains("Widget Helpers"), "Expected 'Widget Helpers' in the output");

    assert!(response.contains("Highlighting"), "Expected 'Highlighting' in the output");
    assert!(response.contains("Autosuggest Widget Implementations"), "Expected 'Autosuggest Widget Implementations' in the output");
    assert!(response.contains("InlineShell Suggestion Strategy"), "Expected 'InlineShell Suggestion Strategy' in the output");
    
    println!("✅ Kiro Cli init zsh post subcommand executed successfully!");
    
    Ok(())
}