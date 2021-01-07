use crate::{
    MIDIInputMap,
    MIDIOutputMap,
};
pub struct MIDIAccess {
    inputs: MIDIInputMap,
    outputs: MIDIOutputMap,
}

impl MIDIAccess {
    // pub fn new() -> Self {
    //     // Self {
    //     // }
    // }
    pub fn inputs(&self) -> &MIDIInputMap {
        &self.inputs
    }

    pub fn outputs(&self) -> &MIDIOutputMap {
        &self.outputs
    }
}
