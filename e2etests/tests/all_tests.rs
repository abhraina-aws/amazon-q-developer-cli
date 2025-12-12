// Main integration test file that includes all subdirectory tests
mod agent;
mod ai_prompts;
mod context;
mod core_session;
mod integration;
mod mcp;
mod model;
mod kiro_cli_subcommand;
mod save_load;
mod session_mgmt;
mod tools;
mod todos;
mod experiment;
mod kiro_steering;
mod sub_integrations;
mod setup_subcommands;
mod diagnostics;
mod init;

use q_cli_e2e_tests::q_chat_helper;

#[ctor::dtor]
fn cleanup_session() {
    let _ = q_chat_helper::close_session();
}