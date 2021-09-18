use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let shared = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = shared.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::future::executor::new_executor_and_spawner;

    #[test]
    fn it_works() {
        let (executor, spawner) = new_executor_and_spawner();

        spawner.spawn(async {
            println!("howdy!");
            TimerFuture::new(Duration::new(2, 0)).await;
            println!("done!");
        });

        drop(spawner);

        executor.run();
    }

    #[tokio::test]
    async fn async_move() {
        let my_string = "foo".to_string();

        let future_one = async {
            println!("{}", my_string); // println! consumes &my_string
        };

        let future_two = async {
            println!("{}", my_string);
        };

        let ((), ()) = futures::join!(future_one, future_two);
    }
}
