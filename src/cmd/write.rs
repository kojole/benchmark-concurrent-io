use std::cmp;
use std::fs::{File, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use file::num_segments;
use segment::{Segment, SEGMENT_SIZE};

pub fn write(path: String, use_direct: bool, num_threads: u8) -> io::Result<Duration> {
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
            let mut writer = Writer::new(path, use_direct, start_segment, end_segment)?;
            writer.start()
        });
        handlers.push(t);
    }

    for handler in handlers {
        handler.join().unwrap()?;
    }

    Ok(start.elapsed())
}

struct Writer {
    file: File,
    range: Range<u32>,
}

impl Writer {
    fn new<P>(path: P, use_direct: bool, start_segment: u32, end_segment: u32) -> io::Result<Writer>
    where
        P: AsRef<Path>,
    {
        let file = OpenOptions::new().write(true).open(path)?;

        Ok(Writer {
            file,
            range: start_segment..end_segment,
        })
    }

    fn start(&mut self) -> io::Result<()> {
        let pos = self.range.start as u64 * SEGMENT_SIZE as u64;
        self.file.seek(SeekFrom::Start(pos))?;

        let mut segment = Segment::new();
        for id in self.range.clone() {
            segment.set_id(id);
            self.file.write_all(segment.as_ref())?;
        }
        Ok(())
    }
}
