pub mod song {
    // use std::fmt;

    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
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
        "Twelve drummers drumming"
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

            let mut verse = format!("On the {} day of Christmas my true love sent to me:", DAYS[self.day]);

            for gift_day in (0..=self.day).rev() {
                if self.day > 0 && gift_day == 0 {
                    verse.push_str(" and ");
                } else {
                    verse.push_str(" ");
                }
                verse.push_str(GIFTS[gift_day]);
                if gift_day > 0 {
                    verse.push_str(",");
                }
            }

            self.day += 1;
            Some(verse)
        }
    }

    pub fn numbered_song_iter() -> impl Iterator<Item = String> {
        SongIter::new().enumerate().map(|(i, line)| format!("{:02}: {}", i + 1, line))
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
            self.iter.next().map(|item| item.clone())
        }
    }
}
