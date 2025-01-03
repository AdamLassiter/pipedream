#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crossbeam_channel::{
    Receiver, RecvError, SendError, Sender, TryRecvError, TrySendError, unbounded,
};

const POLL_INTERVAL: Duration = Duration::from_millis(1);

pub struct Bichannel<Send, Recv>(Sender<Send>, Receiver<Recv>);

#[derive(Clone)]
pub struct BichannelMonitor<Left, Right>
where
    Left: Send + Sync + Clone + 'static,
    Right: Send + Sync + Clone + 'static,
{
    // Clone to make new bichannels
    send_l: Sender<Left>,
    send_r: Sender<Right>,

    // Attacned to returned bichannels, shared with thread to forward to
    frwd_l: Arc<Mutex<Vec<Sender<Left>>>>,
    frwd_r: Arc<Mutex<Vec<Sender<Right>>>>,
}

impl<Left: Send + Sync + Clone, Right: Send + Sync + Clone> BichannelMonitor<Left, Right> {
    pub fn spawn() -> (Self, JoinHandle<()>) {
        let (send_l, recv_l) = unbounded::<Left>();
        let (send_r, recv_r) = unbounded::<Right>();

        let this = Self {
            send_l,
            send_r,
            frwd_l: Arc::new(Mutex::new(vec![])),
            frwd_r: Arc::new(Mutex::new(vec![])),
        };

        let fanout_thread;
        {
            let this = this.clone();
            fanout_thread = thread::spawn(move || {
                loop {
                    {
                        let frwd_l = this.frwd_l.lock().unwrap();
                        while let Ok(mesg) = recv_l.try_recv() {
                            frwd_l.iter().for_each(|sender| {
                                sender.send(mesg.clone()).expect(
                                "Monitor failed to forward upstream message to downstream channel",
                            )
                            });
                        }
                    }
                    {
                        let frwd_r = this.frwd_r.lock().unwrap();
                        while let Ok(mesg) = recv_r.try_recv() {
                            frwd_r.iter().for_each(|sender| {
                                sender.send(mesg.clone()).expect(
                                "Monitor failed to forward upstream message to downstream channel",
                            )
                            });
                        }
                    }
                    thread::sleep(POLL_INTERVAL);
                }
            });
        }
        (this, fanout_thread)
    }

    pub fn new_left(&mut self) -> Bichannel<Left, Right> {
        let (send_r, recv_r) = unbounded::<Right>();
        self.frwd_r.lock().unwrap().push(send_r);
        Bichannel(self.send_l.clone(), recv_r)
    }

    pub fn new_right(&mut self) -> Bichannel<Right, Left> {
        let (send_l, recv_l) = unbounded::<Left>();
        self.frwd_l.lock().unwrap().push(send_l);
        Bichannel(self.send_r.clone(), recv_l)
    }
}

impl<T, U> Bichannel<T, U> {
    pub fn send(&self, mesg: T) -> Result<(), SendError<T>> {
        self.0.send(mesg)
    }

    pub fn try_send(&self, mesg: T) -> Result<(), TrySendError<T>> {
        self.0.try_send(mesg)
    }

    pub fn recv(&self) -> Result<U, RecvError> {
        self.1.recv()
    }

    pub fn try_recv(&self) -> Result<U, TryRecvError> {
        self.1.try_recv()
    }
}
