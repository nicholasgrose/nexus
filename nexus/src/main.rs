use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tasks = JoinSet::new();

    tasks.spawn(nexus::start());
    #[cfg(feature = "suite")]
    tasks.spawn(gateway::start());

    let task_join_result = tasks.join_next().await;
    tasks.abort_all();

    return task_join_result.unwrap()?;
}
