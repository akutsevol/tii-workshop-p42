pub mod song_mod {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader, Write};
    use std::net::{TcpListener, TcpStream};

    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const GIFTS: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    pub struct SongIter {
        day: usize,
    }

    impl SongIter {
        pub fn new() -> Self {
            SongIter { day: 0 }
        }
    }

    impl Iterator for SongIter {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            if self.day >= 12 {
                return None;
            }

            let mut verse = format!(
                "On the {} day of Christmas my true love sent to me:",
                DAYS[self.day]
            );

            for gift_day in (0..=self.day).rev() {
                if self.day > 0 && gift_day == 0 {
                    verse.push_str(" and ");
                } else {
                    verse.push(' ');
                }
                verse.push_str(GIFTS[gift_day]);
                if gift_day > 0 {
                    verse.push(',');
                }
            }

            self.day += 1;
            Some(verse)
        }
    }

    pub fn numbered_song_iter() -> impl Iterator<Item = String> {
        SongIter::new()
            .enumerate()
            .map(|(i, line)| format!("{:02}: {}", i + 1, line))
    }

    pub struct DuplicateIter<I> {
        iter: I,
        count: usize,
        n: usize,
    }

    impl<I> DuplicateIter<I> {
        pub fn new(iter: I, n: usize) -> Self {
            DuplicateIter { iter, count: 0, n }
        }
    }

    impl<I> Iterator for DuplicateIter<I>
    where
        I: Iterator,
        I::Item: Clone,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
            } else {
                self.count = self.n - 1;
                self.iter.next()?;
            }
            self.iter.next()
        }
    }

    #[allow(dead_code)]
    pub fn song_to_string<I>(iter: I) -> String
    where
        I: Iterator<Item = String>,
    {
        iter.collect::<Vec<String>>().join("\n")
    }

    #[allow(dead_code)]
    pub fn song_to_file<I>(iter: I, path: &str) -> io::Result<()>
    where
        I: Iterator<Item = String>,
    {
        let mut file = File::create(path)?;
        for line in iter {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn song_to_tcp<I>(iter: I, address: &str) -> io::Result<()>
    where
        I: Iterator<Item = String>,
    {
        let mut stream = TcpStream::connect(address)?;
        for line in iter {
            writeln!(stream, "{}", line)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn song_from_tcp(port: u16) -> io::Result<String> {
        let listener = TcpListener::bind(("0.0.0.0", port))?;
        // accept connections and process them, spawning a new thread for each one
        if let Some(stream) = listener.incoming().next() {
            match stream {
                Ok(stream) => {
                    // connection succeeded
                    let reader = BufReader::new(stream);
                    let mut received_data = String::new();
                    for line in reader.lines().map_while(Result::ok) {
                        received_data.push_str(&line);
                        received_data.push('\n');
                    }
                    // Stop listening after handling the first connection
                    drop(listener);
                    return Ok(received_data);
                }
                Err(e) => {
                    /* connection failed */
                    drop(listener);
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Connection failed, {}", e),
                    ));
                }
            }
        }
        Err(io::Error::new(io::ErrorKind::Other, "No data received"))
    }
}
