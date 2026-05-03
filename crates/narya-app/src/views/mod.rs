pub mod app_shell;
pub mod dashboard;
pub mod nodes;
pub mod splash;
pub mod subscriptions;
pub mod connections;
pub mod rules;
pub mod logs;
pub mod tools;
pub mod settings;
pub mod config;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveView {
    Dashboard,
    Nodes,
    Connections,
    Rules,
    Subscriptions,
    Config,
    Logs,
    Tools,
    Settings,
    About,
}
