use crate::{
    MIDIClient,
    MIDIInput,
};

pub struct MIDIOutputMap {
    inner: Vec<MIDIInput>,
}

impl MIDIOutputMap {
    fn new(client: midir::MidiInput) -> Self {
        let z = client.ports();
        for e in 0..client.port_count() {}
        todo!()
    }
}

pub trait MIDIMap: std::ops::Index<usize> {
    fn len(&self) -> usize;
}

impl std::ops::Index<usize> for MIDIOutputMap {
    type Output = crate::MIDIOutput;
    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

pub struct MIDIPortIterator<T: MIDIMap>
where
    <T as std::ops::Index<usize>>::Output: Sized,
{
    index: usize,
    len: usize,
    inner: T,
}

impl<T: MIDIMap> MIDIPortIterator<T>
where
    <T as std::ops::Index<usize>>::Output: Sized,
{
    fn new(inner: T) -> Self {
        Self {
            index: 0,
            inner,
            len: todo!(),
        }
    }
}

impl<T: MIDIMap> Iterator for MIDIPortIterator<T>
where
    <T as std::ops::Index<usize>>::Output: Sized,
{
    type Item = <T as std::ops::Index<usize>>::Output;
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
