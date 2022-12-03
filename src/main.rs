use task_manager_linux::{app::layout::tui_execute, sysinfo::Sysinfo};

#[tokio::main]
async fn main() -> () {
    let sys_data = Sysinfo::new();
    tui_execute(sys_data);
}
