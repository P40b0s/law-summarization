use chrono::{NaiveTime, Local, Duration as ChronoDuration};
use futures::future::BoxFuture;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::time::{sleep_until, Instant};

pub type Callback = Arc<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync + 'static>;


pub struct ShedulerNew
{
    duration: std::time::Duration,
}
impl ShedulerNew
{
    pub async fn start(duration: std::time::Duration) -> anyhow::Result<()>
    {
        sleep_until(Instant::now() + duration).await;
        Ok(())
    }
}



pub struct Scheduler 
{
    time_tx: watch::Sender<NaiveTime>,
}



impl Scheduler 
{
    pub fn start(initial_time: NaiveTime, callback: Callback) -> Self 
    {
        let (tx, mut rx) = watch::channel(initial_time);
        // spawn background task
        tokio::spawn(async move 
        {
            loop 
            {
                let target_time = *rx.borrow();
                // compute next occurrence today or tomorrow
                let now = Local::now();
                let today = now.date_naive();
                let mut target_dt = today.and_time(target_time);
                // if the target time already passed today, schedule for tomorrow
                if target_dt <= now.naive_local() 
                {
                    target_dt = (today + ChronoDuration::days(1)).and_time(target_time);
                }
                let dur_secs = (target_dt - now.naive_local()).num_seconds();
                let dur = if dur_secs > 0 { std::time::Duration::from_secs(dur_secs as u64) } else { std::time::Duration::from_secs(0) };
                let when = Instant::now() + dur;

                tokio::select! 
                {
                    _ = sleep_until(when) => 
                    {
                        // time reached — call callback
                        (callback)().await;
                    }
                    res = rx.changed() => 
                    {
                        if res.is_err() 
                        {
                            // sender dropped; exit
                            break;
                        }
                        // loop again to recalc next target
                        continue;
                    }
                }
            }
        });

        Scheduler { time_tx: tx }
    }

    pub fn update_time(&self, new_time: NaiveTime) -> Result<(), ()> {
        self.time_tx.send(new_time).map_err(|_| ())
    }

    pub fn current_time(&self) -> NaiveTime {
        *self.time_tx.borrow()
    }
}


mod tests
{
    use super::*;
    use chrono::NaiveTime;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_scheduler() 
    {
        let counter = Arc::new(AtomicUsize::new(0));
        let callback_counter = Arc::clone(&counter);
        let callback: Callback = Arc::new(move || {
            let callback_counter = Arc::clone(&callback_counter);
            Box::pin(async move {
                callback_counter.fetch_add(1, Ordering::SeqCst);
            })
        });

        let scheduler = Scheduler::start(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), callback);

        // Update time to 1 second from now
        let now = Local::now().naive_local();
        let next_sec = (now + ChronoDuration::seconds(1)).time();
        scheduler.update_time(next_sec).unwrap();

        // Wait for 2 seconds to ensure the callback has been called
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_sheduler_new()
    {
        let start = std::time::Instant::now();
        ShedulerNew::start(std::time::Duration::from_secs(2)).await.unwrap();
        let elapsed = start.elapsed();
        assert!(elapsed >= std::time::Duration::from_secs(2));
    }
}