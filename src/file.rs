use std::fs::{File, OpenOptions};
use std::io::{self, Seek, SeekFrom};
#[cfg(target_os = "linux")]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

#[cfg(target_os = "linux")]
use libc;

use segment::SEGMENT_SIZE;

pub fn num_segments<P>(path: P) -> io::Result<u32>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path.as_ref())?;
    let file_size = file.seek(SeekFrom::End(0))?;
    Ok((file_size / SEGMENT_SIZE as u64) as u32)
}

pub fn open<P>(path: P, write: bool, direct: bool) -> io::Result<File>
where
    P: AsRef<Path>,
{
    let mut options = OpenOptions::new();

    if write {
        options.write(true);
    } else {
        options.read(true);
    }

    if direct {
        #[cfg(target_os = "linux")]
        options.custom_flags(libc::O_DIRECT);
    }

    options.open(path.as_ref())
}
