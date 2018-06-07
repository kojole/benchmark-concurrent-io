use std::cmp;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom};
use std::ops::Range;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use file::num_segments;
use segment::{Segment, SEGMENT_SIZE};

pub fn read(path: String, use_direct: bool, num_threads: u8) -> io::Result<(Duration, u64)> {
    let start = Instant::now();

    let num_segments = num_segments(&path)?;
    assert!(num_segments > 0);
    let count = (num_segments as f64 / num_threads as f64).ceil() as u32;

    let mut handlers = Vec::new();

    for i in 0..(num_threads as u32) {
        let start_segment = i * count;
        if start_segment >= num_segments {
            break;
        }

        let path = path.clone();
        let t = thread::spawn(move || {
            let end_segment = cmp::min(start_segment + count, num_segments);
            let mut reader = Reader::new(path, use_direct, start_segment, end_segment)?;
            reader.start()
        });
        handlers.push(t);
    }

    let mut sum = 0;

    for handler in handlers {
        sum += handler.join().unwrap()?;
    }

    Ok((start.elapsed(), sum))
}

struct Reader {
    file: File,
    range: Range<u32>,
}

impl Reader {
    fn new<P>(path: P, use_direct: bool, start_segment: u32, end_segment: u32) -> io::Result<Reader>
    where
        P: AsRef<Path>,
    {
        let file = OpenOptions::new().read(true).open(path)?;

        Ok(Reader {
            file,
            range: start_segment..end_segment,
        })
    }

    fn start(&mut self) -> io::Result<u64> {
        let pos = self.range.start as u64 * SEGMENT_SIZE as u64;
        self.file.seek(SeekFrom::Start(pos))?;

        let mut segment = Segment::new();
        let mut sum = 0;
        for _ in self.range.clone() {
            self.file.read_exact(segment.as_mut())?;
            sum += segment.id() as u64;
        }
        Ok(sum)
    }
}
