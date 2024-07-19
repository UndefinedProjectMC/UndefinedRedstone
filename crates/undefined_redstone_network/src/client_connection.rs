use flume::{Receiver, Sender, TryRecvError, TrySendError};
use tokio::task::JoinHandle;
use crate::packet::batch_packet::BatchPacket;
use crate::packet::packet_factory::PacketFactory;

pub struct ClientConnection {
    pub(crate) receive_loop: JoinHandle<()>,
    pub(crate) send_loop: JoinHandle<()>,
    pub(crate) packet_receiver: Receiver<BatchPacket>,
    pub(crate) packet_sender: Sender<(BatchPacket, bool)>
}

impl ClientConnection {

    pub fn try_send(&self, packets: BatchPacket, immediate: bool) -> Result<(), TrySendError<(BatchPacket, bool)>> {
        self.packet_sender.try_send((packets, immediate))
    }

    pub fn try_recv(&self) -> Result<BatchPacket, TryRecvError> {
        self.packet_receiver.try_recv()
    }
}