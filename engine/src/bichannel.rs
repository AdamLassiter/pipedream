use crossbeam::channel::{unbounded, Receiver, RecvError, SendError, Sender, TryRecvError, TrySendError};

pub struct Channel<Send, Recv>(Sender<Send>, Receiver<Recv>);
pub type Bichannel<T, U> = (Channel<T, U>, Channel<U, T>);

pub fn bichannel<T, U>() -> Bichannel<T, U> {
    let (send_t, recv_t) = unbounded::<T>();
    let (send_u, recv_u) = unbounded::<U>();
    (Channel(send_t, recv_u), Channel(send_u, recv_t))
}

impl<T, U> Channel<T, U> {
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