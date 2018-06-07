use std::fs::File;
use std::io::{self, Seek, SeekFrom};
use std::path::Path;

use segment::SEGMENT_SIZE;

pub fn num_segments<P>(path: P) -> io::Result<u32>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path.as_ref())?;
    let file_size = file.seek(SeekFrom::End(0))?;
    Ok((file_size / SEGMENT_SIZE as u64) as u32)
}
