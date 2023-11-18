use std::future::Future;

/// Spawn a task for every future in the iterator and put the results in a `Vec`.
pub async fn spawn_join<F>(iter: impl Iterator<Item = F>) -> Vec<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let futures: Vec<_> = iter.map(tokio::spawn).collect();
    let mut results = Vec::new();
    for fut in futures {
        results.push(fut.await.unwrap());
    }
    results
}
