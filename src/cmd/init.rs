use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use segment::SEGMENT_SIZE;

pub fn init<P>(path: P, num_segments: u32) -> io::Result<()>
where
    P: AsRef<Path>,
{
    assert!(num_segments > 0);

    let mut file = File::create(path.as_ref())?;
    let buf = [0u8; SEGMENT_SIZE];
    for _ in 0..num_segments {
        file.write_all(&buf)?;
    }
    file.sync_all()
}
