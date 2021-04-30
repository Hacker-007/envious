use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, RecvError},
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crossterm::event::{poll, read, KeyCode, KeyEvent};

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: Receiver<Event<KeyEvent>>,
    ignore_exit_key: Arc<AtomicBool>,
    _handle: JoinHandle<()>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let tick_rate = Duration::from_millis(250);
        let handle = {
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or_else(|| Duration::from_secs(0));
                    if poll(timeout).unwrap() {
                        if let Ok(crossterm::event::Event::Key(key)) = read() {
                            if tx.send(Event::Input(key)).is_err() {
                                return;
                            }

                            if !ignore_exit_key.load(Ordering::Relaxed)
                                && key.code == KeyCode::Char('q')
                            {
                                return;
                            }
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if tx.send(Event::Tick).is_err() {
                            return;
                        }

                        last_tick = Instant::now();
                    }
                }
            })
        };

        Events {
            rx,
            ignore_exit_key,
            _handle: handle,
        }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, RecvError> {
        self.rx.recv()
    }

    pub fn disable_exit_key(&mut self) {
        self.ignore_exit_key.store(true, Ordering::Relaxed);
    }

    pub fn enable_exit_key(&mut self) {
        self.ignore_exit_key.store(false, Ordering::Relaxed);
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}