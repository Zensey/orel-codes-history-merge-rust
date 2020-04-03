#![allow(warnings)]
use std::io::{self, BufRead, Write, BufWriter, BufReader};
use std::fs::{File, OpenOptions};
use std::time::{Instant};
use std::io::prelude::*;

fn merge<W: Write, I:Iterator<Item=i32>>(server: &mut I, local: &mut I, out_merged: &mut W, out_diff: &mut W) {
    let mut nxt_b = server.next();
    let mut nxt_a = local.next();
    let mut va = 0;
    let mut vb = 0;

    while nxt_a.is_some() || nxt_b.is_some() {
        if nxt_a.is_some() {
            va = nxt_a.unwrap();
            if !nxt_b.is_some() || va < nxt_b.unwrap() {
                writeln!(out_merged, "{}", va);
                nxt_a = local.next();
            }
        }
        if nxt_b.is_some() {
            vb = nxt_b.unwrap();
            let mut ok = false;
            if !nxt_a.is_some() || vb < nxt_a.unwrap() {
                writeln!(out_merged, "{}", vb);
                writeln!(out_diff, "{}", vb);
                nxt_b = server.next();
            }
        }
        if nxt_a.is_some() && nxt_b.is_some() {
            vb = nxt_b.unwrap();
            va = nxt_a.unwrap();
            if vb == va {
                writeln!(out_merged, "{}", va);
                nxt_b = server.next();
                nxt_a = local.next();
            }
        }
    }
}

fn get_reader (file: &str) -> io::Result<BufReader<Box< dyn Read >>>{
    let read : Box<dyn Read> = match file {
        "stdin" => Box::new(io::stdin()),
        _       => Box::new(File::open(file)? )
    };
    Ok(BufReader::new(read))
}

fn get_writer (file_name: &str) -> io::Result<BufWriter<File>> {
    let f = OpenOptions::new().create(true).write(true).open(file_name)?;
    let f = BufWriter::new(f);
    Ok(f)
}

fn main() -> io::Result<()> {
    let f_local = get_reader("./input_local.txt")?;
    let f_server = get_reader("./input_server.txt")?;

    let mut list_local = Reader::IteratorInt32::open(f_local);
    let mut list_server = Reader::IteratorInt32::open(f_server);

    let mut f_merged = get_writer("output_result.txt")?;
    let mut f_diff = get_writer("output_missing.txt")?;

    let now = Instant::now();
    merge(&mut list_server, &mut list_local, &mut f_merged, &mut f_diff);
    println!("Time elapsed: {:?}", now.elapsed());

    f_merged.flush();
    f_diff.flush();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng, StdRng};
    use std::intrinsics::transmute;
    use std::error::Error;
    use std::path::Iter;
    use std::ptr::null;
    use std::io::Cursor;
    use std::io::prelude::*;

    #[test]
    fn test_merge() {
        use std::io::Cursor;
        use std::io::prelude::*;

        let mut input_local = Cursor::new(Vec::new());
        input_local.write(b"1\n2\n3\n4\n5\n8\n9\n");
        input_local.set_position(0);
        let mut input_local = BufReader::new(Box::new(input_local) as Box<dyn Read>);
        let mut list_local = Reader::IteratorInt32::open(input_local);

        let mut input_server = Cursor::new(Vec::new());
        input_server.write(b"5\n6\n7\n8\n9\n10\n");
        input_server.set_position(0);
        let mut input_server = BufReader::new(Box::new(input_server) as Box<dyn Read>);
        let mut list_server = Reader::IteratorInt32::open(input_server);

        let mut out = Cursor::new(Vec::new());
        let mut out2 = Cursor::new(Vec::new());

        merge(&mut list_server, &mut list_local, &mut out, &mut out2);

        let mut c = Cursor::new(Vec::new());
        c.write(b"6\n7\n10\n");
        assert!(out2.eq(&c));

        let mut c = Cursor::new(Vec::new());
        c.write(b"1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
        assert!(out.eq(&c));
    }
}

mod Reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };
    use std::thread::sleep;
    use std::time::Duration;
    use std::rc::Rc;

    pub struct IteratorInt32 {
        reader: io::BufReader<Box<dyn Read>>,
        buf: String,
    }

    impl IteratorInt32 {
        pub fn open (f: io::BufReader<Box<dyn Read>>) -> Self {
            let buf = String::with_capacity(1024);
            let reader = f;

            Self { reader, buf }
        }
    }

    impl Iterator for IteratorInt32 {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.buf.clear();
            self.reader
                .read_line(&mut self.buf)
                .map(|u| if u == 0 { None } else { Some(self.buf.trim().parse::<i32>().unwrap()) })
                .unwrap()
        }
    }
}
