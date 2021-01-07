pub enum MIDIPortKind {}
pub enum MIDIPortDeviceState {}

pub enum MIDIPortConnectionState {}
pub trait MIDIPort {
    fn id(&self) -> i32;
    fn manufacturer(&self) -> String;

    fn name(&self) -> String;

    /// .input (for MIDIInput) or .output (for MIDIOutput)
    fn kind(&self) -> MIDIPortKind;

    fn version(&self) -> u32;

    /// .connected | .disconnected,
    /// indicates if the port's endpoint is connected or not
    fn state(&self) -> MIDIPortDeviceState;

    /// .open | .closed
    fn connection(&self) -> MIDIPortConnectionState;

    /// open the port, is called implicitly when MIDIInput's onMIDIMessage is set or MIDIOutputs' send is called
    fn open(&mut self);

    /// closes the port
    fn close(&mut self);
}
