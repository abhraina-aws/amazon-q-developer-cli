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

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_fish_pre_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init fish pre ... | Description: Tests the <code> kiro-cli init fish pre  </code> subcommand to verify kiro-cli init fish pre subcommand.");

    println!("\n🔍 Executing 'kiro-cli init fish pre' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","fish","pre"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("command mkdir"), "Expected 'command mkdir' in the output");
    assert!(response.contains("Q_NEW_SESSION"), "Expected 'Q_NEW_SESSION' in the output");

    assert!(response.contains("Load parent"), "Expected 'Load parent' in the output");
    
    println!("✅ Kiro Cli init fist pre subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_fish_post_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init fish post ... | Description: Tests the <code> kiro-cli init fish post  </code> subcommand to verify kiro-cli init fish post subcommand.");

    println!("\n🔍 Executing 'kiro-cli init fish post' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","fish","post"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("set --query"), "Expected 'set --query' in the output");
    assert!(response.contains("TTY"), "Expected 'TTY' in the output");

    assert!(response.contains("fig_wrap_prompt"), "Expected 'fig_wrap_prompt' in the output");
    
    println!("✅ Kiro Cli init fist post subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_nu_pre_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init nu pre ... | Description: Tests the <code> kiro-cli init nu pre  </code> subcommand to verify kiro-cli init nu pre subcommand.");

    println!("\n🔍 Executing 'kiro-cli init nu pre' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","nu","pre"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Q_SET_PARENT_CHECK"), "Expected 'Q_SET_PARENT_CHECK' in the output");
    assert!(response.contains("should_launch"), "Expected 'should_launch' in the output");

    assert!(response.contains("with-env"), "Expected 'with-env' in the output");
    assert!(response.contains("Q_SET_PARENT_CHECK"), "Expected 'Q_SET_PARENT_CHECK' in the output");
    println!("✅ Kiro Cli init nu pre subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_nu_post_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init nu pre ... | Description: Tests the <code> kiro-cli init nu post  </code> subcommand to verify kiro-cli init nu post subcommand.");

    println!("\n🔍 Executing 'kiro-cli init nu post' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","nu","post"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("fig_reset_hooks"), "Expected 'fig_reset_hooks' in the output");
    assert!(response.contains("let hooks ="), "Expected 'let hooks =' in the output");

    assert!(response.contains("fig_pre_execution_hook"), "Expected 'fig_pre_execution_hook' in the output");
    assert!(response.contains("fig_set_prompt"), "Expected 'fig_set_prompt' in the output");
    assert!(response.contains("StartPrompt"), "Expected 'StartPrompt' in the output");

    println!("✅ Kiro Cli init nu post subcommand executed successfully!");
    
    Ok(())
}


#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_help_shorthand_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init -h ... | Description: Tests the <code> kiro-cli init -h  </code> subcommand to verify help options.");

    println!("\n🔍 Executing 'kiro-cli init -h' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","-h"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Usage"), "Expected 'Usage' in the output");
    assert!(response.contains("[OPTIONS]"), "Expected '[OPTIONS]' in the output");
    
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

    println!("✅ Kiro Cli init -h subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_bash_verbose_bash_pre_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init --verbose bash pre ... | Description: Tests the <code> kiro-cli init --verbose bash pre  </code> subcommand to verify verbose bash pre response.");

    println!("\n🔍 Executing 'kiro-cli init -verbose bash pre' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","--verbose","bash","pre"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("SHOULD_QTERM_LAUNCH"), "Expected 'SHOULD_QTERM_LAUNCH' in the output");
    assert!(response.contains("mkdir -p"), "Expected 'mkdir -p' in the output");
    assert!(response.contains("Q_NEW_SESSION"), "Expected 'Q_NEW_SESSION' in the output");
    
    assert!(response.contains("Q_SET_PARENT_CHECK"), "Expected 'Q_SET_PARENT_CHECK' in the output");
    assert!(response.contains("kiro-cli-term"), "Expected 'kiro-cli-term' in the output");
    assert!(response.contains("Q_EXECUTION_STRING"), "Expected 'Q_EXECUTION_STRING' in the output");
    
    assert!(response.contains("exec -a"), "Expected 'exec -a' in the output");
    assert!(!response.contains("SHOULD_QTERM_LAUNCH") || response.contains("bash pre"), "bash post should not contain SHOULD_QTERM_LAUNCH");

    println!("✅ Kiro Cli init --verbose bash pre subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_bash_verbose_bash_post_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init --verbose bash post ... | Description: Tests the <code> kiro-cli init --verbose bash post  </code> subcommand to verify verbose bash post response.");

    println!("\n🔍 Executing 'kiro-cli init -verbose bash post' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","--verbose","bash","post"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("__fig_preexec"), "Expected '__fig_preexec' in the output");
    assert!(response.contains("__fig_pre_prompt"), "Expected '__fig_pre_prompt' in the output");
    assert!(response.contains("PROMPT_COMMAND"), "Expected 'PROMPT_COMMAND' in the output");
    
    assert!(response.contains("precmd_functions"), "Expected 'precmd_functions' in the output");
    assert!(response.contains("preexec_functions"), "Expected 'preexec_functions' in the output");
    assert!(response.contains("fig_osc"), "Expected 'fig_osc' in the output");
    
    assert!(response.contains("StartPrompt"), "Expected 'StartPrompt' in the output");
    assert!(response.contains("EndPrompt"), "Expected 'EndPrompt' in the output");
    assert!(!response.contains("__fig_preexec") || response.contains("bash post"), "bash pre should not focus on __fig_preexec");
    
    println!("✅ Kiro Cli init --verbose bash post subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_verbose_zsh_pre_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init --verbose zsh pre ... | Description: Tests the <code> kiro-cli init --verbose zsh pre  </code> subcommand to verify verbose zsh pre response.");

    println!("\n🔍 Executing 'kiro-cli init -verbose zsh pre' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","--verbose","zsh","pre"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("SHOULD_QTERM_LAUNCH"), "Expected 'SHOULD_QTERM_LAUNCH' in the output");
    assert!(response.contains("mkdir -p"), "Expected 'mkdir -p' in the output");
    assert!(response.contains("Q_NEW_SESSION"), "Expected 'Q_NEW_SESSION' in the output");
    
    assert!(response.contains("Q_SET_PARENT_CHECK"), "Expected 'Q_SET_PARENT_CHECK' in the output");
    assert!(response.contains("kiro-cli-term"), "Expected 'kiro-cli-term' in the output");
    assert!(response.contains("Q_EXECUTION_STRING"), "Expected 'Q_EXECUTION_STRING' in the output");
    
    assert!(response.contains("exec -a"), "Expected 'exec -a' in the output");
    assert!(response.contains("Q_IS_LOGIN_SHELL"), "Expected 'Q_IS_LOGIN_SHELL' in the output");
    assert!(!response.contains("Q_DOTFILES_SOURCED") || response.contains("zsh post"), "zsh pre should not contain autosuggestions");
    
    println!("✅ Kiro Cli init --verbose zsh pre subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_verbose_zsh_post_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init --verbose zsh post ... | Description: Tests the <code> kiro-cli init --verbose zsh post  </code> subcommand to verify verbose zsh post response.");

    println!("\n🔍 Executing 'kiro-cli init -verbose zsh post' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","--verbose","zsh","post"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Q_DOTFILES_SOURCED"), "Expected 'Q_DOTFILES_SOURCED' in the output");
    assert!(response.contains("KIRO_CLI_AUTOSUGGEST"), "Expected 'KIRO_CLI_AUTOSUGGEST' in the output");
    assert!(response.contains("_kiro_cli_autosuggest"), "Expected '_kiro_cli_autosuggest' in the output");
   
    assert!(response.contains("fig_preexec"), "Expected 'fig_preexec' in the output");
    assert!(response.contains("fig_precmd"), "Expected 'fig_precmd' in the output");
    assert!(response.contains("precmd_functions"), "Expected 'precmd_functions' in the output");
   
    assert!(response.contains("preexec_functions"), "Expected 'preexec_functions' in the output");
    assert!(response.contains("Q_USER_PS1"), "Expected 'Q_USER_PS1' in the output");
    assert!(response.contains("inline_shell_completion"), "Expected 'inline_shell_completion' in the output");
    
    assert!(!response.contains("SHOULD_QTERM_LAUNCH") || response.contains("zsh pre"), "zsh post should not contain terminal launch logic");

    println!("✅ Kiro Cli init --verbose zsh post subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_verbose_fish_pre_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init --verbose fish pre ... | Description: Tests the <code> kiro-cli init --verbose fish pre  </code> subcommand to verify verbose fish pre response.");

    println!("\n🔍 Executing 'kiro-cli init --verbose fish pre' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","--verbose","fish","pre"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("SHOULD_QTERM_LAUNCH"), "Expected 'SHOULD_QTERM_LAUNCH' in the output");
    assert!(response.contains("command mkdir -p"), "Expected 'command mkdir -p' in the output");
    assert!(response.contains("Q_NEW_SESSION"), "Expected 'Q_NEW_SESSION' in the output");
    
    assert!(response.contains("Q_PARENT"), "Expected 'Q_PARENT' in the output");
    assert!(response.contains("kiro-cli-term"), "Expected 'kiro-cli-term' in the output");
    assert!(response.contains("Q_SHELL"), "Expected 'Q_SHELL' in the output");
    
    assert!(response.contains("exec bash -c"), "Expected 'exec bash -c' in the output");
    assert!(response.contains("Q_IS_LOGIN_SHELL"), "Expected 'Q_IS_LOGIN_SHELL' in the output");
    assert!(!response.contains("fig_preexec"), "fish pre should not contain fig_preexec hooks");
    
    println!("✅ Kiro Cli init --verbose fish pre subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_verbose_fish_post_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init --verbose fish post ... | Description: Tests the <code> kiro-cli init --verbose fish post  </code> subcommand to verify verbose fish post response.");

    println!("\n🔍 Executing 'kiro-cli init --verbose fish post' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","--verbose","fish","post"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("fig_preexec"), "Expected 'fig_preexec' in the output");
    assert!(response.contains("fig_precmd"), "Expected 'fig_precmd' in the output");
    assert!(response.contains("fig_wrap_prompt"), "Expected 'fig_wrap_prompt' in the output");
    
    assert!(response.contains("fig_copy_fn"), "Expected 'fig_copy_fn' in the output");
    assert!(response.contains("StartPrompt"), "Expected 'StartPrompt' in the output");
    assert!(response.contains("EndPrompt"), "Expected 'EndPrompt' in the output");
    
    assert!(response.contains("fish_prompt"), "Expected 'fish_prompt' in the output");
    assert!(response.contains("QTERM_SESSION_ID"), "Expected 'QTERM_SESSION_ID' in the output");
    assert!(!response.contains("SHOULD_QTERM_LAUNCH"), "fish post should not contain terminal launch logic");
    
    println!("✅ Kiro Cli init --verbose fish post subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_v_bash_pre_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init -v bash pre ... | Description: Tests the <code> kiro-cli init -v bash pre  </code> subcommand to verify verbose bash pre response.");

    println!("\n🔍 Executing 'kiro-cli init -v bash pre' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","-v","bash","pre"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("Q_SHELL="));
    assert!(response.contains("SHOULD_QTERM_LAUNCH="));
    assert!(response.contains("__fig_source_bash_preexec"));
    assert!(!response.contains("__fig_pre_prompt"));
    println!("✅ Kiro Cli init -v bash pre subcommand executed successfully!");
    
    Ok(())
}

#[test]
#[cfg(all(feature = "init", feature = "sanity"))]
fn test_kiro_cli_init_v_bash_post_subommand() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Testing kiro-cli init -v bash post ... | Description: Tests the <code> kiro-cli init -v bash post  </code> subcommand to verify verbose bash post response.");

    println!("\n🔍 Executing 'kiro-cli init -v bash post' subcommand...");
    let response = q_chat_helper::execute_q_subcommand("kiro-cli", &["init","-v","bash","post"])?;
    
    println!("📝 FULL OUTPUT:");
    println!("{}", response);
    println!("📝 END OUTPUT");

    assert!(response.contains("__fig_pre_prompt"));
    assert!(response.contains("__fig_post_prompt"));
    assert!(response.contains("fig_osc"));
    assert!(response.contains("kiro-cli _ pre-cmd"));
    println!("✅ Kiro Cli init -v bash post subcommand executed successfully!");
    Ok(())
}