use std::mem::replace;
use std::sync::{Arc, RwLock, mpsc::{Receiver, Sender, channel}};
use std::thread::{self, JoinHandle};

use super::{ChunkCoords, ChunkData};

pub struct ChunkSaver {
    sender: Sender<Job>,
    worker: JoinHandle<()>,
}

impl ChunkSaver {
    pub fn new() -> Self {
        let (sender, recv) = channel();
        Self {
            sender,
            worker: spawn_worker(recv),
        }
    }

    pub fn save(&mut self, coords: ChunkCoords, data: Arc<RwLock<ChunkData>>) {
        if let Err(err) = self.sender.send(Job(coords, data)) {
            self.replace_worker();
            self.sender.send(err.0).unwrap();
        }
    }

    pub fn wait_completion(&mut self) {
        let old_worker = self.replace_worker();
        let _ = old_worker.join();
    }

    fn replace_worker(&mut self) -> JoinHandle<()> {
        let (new_send, new_recv) = channel();
        self.sender = new_send;
        let new_worker = spawn_worker(new_recv);
        replace(&mut self.worker, new_worker)
    }
}

fn spawn_worker(recv: Receiver<Job>) -> JoinHandle<()> {
    thread::Builder::new()
        .name("chunk saving".into())
        .spawn(move || {
            worker(recv)
        }).unwrap()
}

fn worker(recv: Receiver<Job>) {
    while let Ok(job) = recv.recv() {
        let Job(coords, data) = job;
        if let Ok(data) = data.read() {
            if let Err(err) = data.save(coords) {
                eprintln!("Error saving chunk at ({}, {}): {}", 
                    coords.0, coords.1, err)
            }
        };
    }
}

struct Job(ChunkCoords, Arc<RwLock<ChunkData>>);
