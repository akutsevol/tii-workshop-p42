mod song;

use song::song::{numbered_song_iter, DuplicateIter, SongIter};

fn main() {
    let song_iter = SongIter::new();
    for line in song_iter {
        println!("{}", line);
    }

    println!("\nNumbered Song:");
    for line in numbered_song_iter() {
        println!("{}", line);
    }

    println!("\nDuplicated Song:");
    let duplicated_song_iter = DuplicateIter::new(SongIter::new(), 2);
    for line in duplicated_song_iter {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use song::song::{SongIter, numbered_song_iter, DuplicateIter};

    #[test]
    fn test_song_iter() {
        let expected = vec![
            "On the first day of Christmas my true love sent to me: A partridge in a pear tree",
            "On the second day of Christmas my true love sent to me: Two turtle doves, and, and A partridge in a pear tree",
            "On the third day of Christmas my true love sent to me: Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the fourth day of Christmas my true love sent to me: Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the fifth day of Christmas my true love sent to me: Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the sixth day of Christmas my true love sent to me: Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the seventh day of Christmas my true love sent to me: Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the eighth day of Christmas my true love sent to me: Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the ninth day of Christmas my true love sent to me: Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the tenth day of Christmas my true love sent to me: Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the eleventh day of Christmas my true love sent to me: Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the twelfth day of Christmas my true love sent to me: Twelve drummers drumming, Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree"
        ];
        let song_iter = SongIter::new();
        let mut expected_itr = expected.iter();
        for line in song_iter {
            assert_eq!(Some(&line), expected_itr.next().map(|s| s.to_string()).as_ref());
        }
    }
    #[test]
    fn test_numbered_song_iter() {
        let expected = vec![
            "01: On the first day of Christmas my true love sent to me: A partridge in a pear tree",
            "02: On the second day of Christmas my true love sent to me: Two turtle doves, and, and A partridge in a pear tree",
            "03: On the third day of Christmas my true love sent to me: Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "04: On the fourth day of Christmas my true love sent to me: Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "05: On the fifth day of Christmas my true love sent to me: Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "06: On the sixth day of Christmas my true love sent to me: Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "07: On the seventh day of Christmas my true love sent to me: Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "08: On the eighth day of Christmas my true love sent to me: Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "09: On the ninth day of Christmas my true love sent to me: Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "10: On the tenth day of Christmas my true love sent to me: Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "11: On the eleventh day of Christmas my true love sent to me: Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "12: On the twelfth day of Christmas my true love sent to me: Twelve drummers drumming, Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree"
        ];
        let numbered_iter = numbered_song_iter();
        let mut expected_itr = expected.iter();
        for line in numbered_iter {
            assert_eq!(Some(&line), expected_itr.next().map(|s| s.to_string()).as_ref());
        }
    }
    #[test]
    fn test_duplicate_iter() {
        let song_iter = SongIter::new();
        let expected = vec![
            "On the second day of Christmas my true love sent to me: Two turtle doves, and, and A partridge in a pear tree",
            "On the third day of Christmas my true love sent to me: Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the fifth day of Christmas my true love sent to me: Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the sixth day of Christmas my true love sent to me: Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the eighth day of Christmas my true love sent to me: Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the ninth day of Christmas my true love sent to me: Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the eleventh day of Christmas my true love sent to me: Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree",
            "On the twelfth day of Christmas my true love sent to me: Twelve drummers drumming, Eleven pipers piping, Ten lords a-leaping, Nine ladies dancing, Eight maids a-milking, Seven swans a-swimming, Six geese a-laying, Five golden rings, Four calling birds, Three French hens, Two turtle doves, and, and A partridge in a pear tree"
        ];
        let duplicated_iter = DuplicateIter::new(song_iter, 2);
        let mut expected_itr = expected.iter();
        for line in duplicated_iter {
            assert_eq!(Some(&line), expected_itr.next().map(|s| s.to_string()).as_ref());
        }
    }
}