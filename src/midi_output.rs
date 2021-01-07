use crate::{
    MIDIPort,
    MIDIPortConnectionState,
    MIDIPortDeviceState,
    MIDIPortKind,
};
pub struct MIDIOutput {
    inner: midir::MidiOutput,
    connection: midir::MidiOutputConnection,
}

impl MIDIOutput {
    fn new(inner: midir::MidiOutput, connection: midir::MidiOutputConnection) -> Self {
        todo!()
    }

    pub fn send(&self) {
        todo!()
    }
}

impl MIDIPort for MIDIOutput {
    fn id(&self) -> i32 {
        todo!()
    }

    fn manufacturer(&self) -> &str {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    /// .input (for MIDIInput) or .output (for MIDIOutput)
    fn kind(&self) -> MIDIPortKind {
        MIDIPortKind::Output
    }

    fn version(&self) -> u32 {
        todo!()
    }

    /// .connected | .disconnected,
    /// indicates if the port's endpoint is connected or not
    fn state(&self) -> MIDIPortDeviceState {
        todo!()
    }

    /// .open | .closed
    fn connection(&self) -> MIDIPortConnectionState {
        todo!()
    }

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self) {
        todo!()
    }

    /// closes the port
    fn close(&mut self) {
        todo!()
    }
}
