use std::{any::Any, sync::atomic::AtomicU64, thread::ScopedJoinHandle};

fn get_finished_index<T>(threads: &[Option<ScopedJoinHandle<T>>]) -> Option<usize> {
    threads
        .iter()
        .position(|thread| thread.as_ref().is_none_or(|x| x.is_finished()))
}

fn is_finished<T>(threads: &[Option<ScopedJoinHandle<T>>]) -> bool {
    threads
        .iter()
        .all(|thread| thread.as_ref().is_none_or(|x| x.is_finished()))
}

type ProgressReporter = fn(progress: &AtomicU64, time_start: std::time::Instant) -> ();

///
/// # Errors
/// Returns Err if the operation being targeted by the threads panics
///
pub fn apply_threads<U, V>(
    nthreads: usize,
    operation: fn(U, Option<&AtomicU64>) -> V,
    arguments: &[U],
    progress_reporter: Option<ProgressReporter>,
) -> Result<Vec<V>, Box<dyn Any + Send + 'static>>
where
    U: Send + Clone,
    V: Send,
{
    let progress = if progress_reporter.is_some() {
        Some(AtomicU64::new(0u64))
    } else {
        None
    };
    let progress = progress.as_ref();
    let mut results = Vec::new();
    std::thread::scope(|spawner| {
        let time_start = std::time::Instant::now();
        //prevent fully spinlocking
        let sleep = std::time::Duration::from_millis(10);
        let mut threads: Vec<Option<ScopedJoinHandle<V>>> = (0..nthreads).map(|_| None).collect();
        for arg in arguments {
            loop {
                if let Some(idx) = get_finished_index(&threads) {
                    if let Some(thread) = threads[idx].take() {
                        results.push(thread.join()?);
                    }
                    let arg = arg.clone();
                    threads[idx] = Some(spawner.spawn(|| operation(arg, progress)));
                    break;
                }
                progress_reporter.inspect(|reporter| {
                    progress.inspect(|val| {
                        reporter(val, time_start);
                    });
                });
                std::thread::sleep(sleep);
            }
        }
        while !is_finished(&threads) {
            progress_reporter.inspect(|reporter| {
                progress.inspect(|val| {
                    reporter(val, time_start);
                });
            });
            std::thread::sleep(sleep);
        }
        Ok(results)
    })
}
