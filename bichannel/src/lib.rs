#![feature(panic_update_hook)]
#![feature(const_trait_impl)]
#![feature(str_split_remainder)]
#![feature(let_chains)]

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use crossbeam::channel::{
    unbounded, Receiver, RecvError, SendError, Sender, TryRecvError, TrySendError,
};

pub struct Bichannel<Send, Recv>(Sender<Send>, Receiver<Recv>);

pub struct BichannelMonitor<Left, Right>(
    #[allow(clippy::type_complexity)]
    Arc<Mutex<(Vec<Bichannel<Left, Right>>, Vec<Bichannel<Right, Left>>)>>,
)
where
    Left: Send + Sync + Clone + 'static,
    Right: Send + Sync + Clone + 'static;

impl<Left: Send + Sync + Clone, Right: Send + Sync + Clone> BichannelMonitor<Left, Right> {
    fn new() -> Self {
        Self(Arc::new(Mutex::new((vec![], vec![]))))
    }

    pub fn spawn() -> (Self, JoinHandle<()>) {
        let this = Self::new();
        let fanout;
        {
            let this = Self(Arc::clone(&this.0));
            fanout = thread::spawn(move || loop {
                {
                    let channels = this.0.lock().unwrap();
                    channels.0.iter().for_each(|receiver| {
                        while let Ok(mesg) = receiver.try_recv() {
                            channels.1.iter().for_each(|sender| {
                                sender.send(mesg.clone()).expect(
                                "Monitor failed to forward upstream message to downstream channel",
                            )
                            });
                        }
                    });
                    channels.1.iter().for_each(|receiver| {
                        while let Ok(mesg) = receiver.try_recv() {
                            channels.0.iter().for_each(|sender| {
                                sender.send(mesg.clone()).expect(
                                "Monitor failed to forward upstream message to downstream channel",
                            )
                            });
                        }
                    });
                }
                thread::sleep(Duration::from_millis(5));
            });
        }
        (this, fanout)
    }

    pub fn new_left(&mut self) -> Bichannel<Left, Right> {
        let (send_left, recv_left) = unbounded::<Left>();
        let (send_right, recv_right) = unbounded::<Right>();
        self.0
            .lock()
            .unwrap()
            .1
            .push(Bichannel(send_right, recv_left));
        Bichannel(send_left, recv_right)
    }

    pub fn new_right(&mut self) -> Bichannel<Right, Left> {
        let (send_left, recv_left) = unbounded::<Left>();
        let (send_right, recv_right) = unbounded::<Right>();
        self.0
            .lock()
            .unwrap()
            .0
            .push(Bichannel(send_left, recv_right));
        Bichannel(send_right, recv_left)
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
