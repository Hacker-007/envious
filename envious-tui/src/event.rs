use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, RecvError},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use termion::{event::Key, input::TermRead};

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: Receiver<Event<Key>>,
    ignore_exit_key: Arc<AtomicBool>,
    _input_handle: JoinHandle<()>,
    _tick_handle: JoinHandle<()>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let tx = tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for event in stdin.keys() {
                    if let Ok(key) = event {
                        if tx.send(Event::Input(key)).is_err() {
                            return;
                        }

                        if !ignore_exit_key.load(Ordering::Relaxed) && key == Key::Char('q') {
                            return;
                        }
                    }
                }
            })
        };

        let tick_handle = {
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }

                thread::sleep(Duration::from_millis(250));
            })
        };

        Events {
            rx,
            ignore_exit_key,
            _input_handle: input_handle,
            _tick_handle: tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, RecvError> {
        self.rx.recv()
    }

    pub fn disable_exit_key(&mut self) {
        self.ignore_exit_key.store(true, Ordering::Relaxed);
    }

    pub fn enable_exit_key(&mut self) {
        self.ignore_exit_key.store(false, Ordering::Relaxed);
    }
}
