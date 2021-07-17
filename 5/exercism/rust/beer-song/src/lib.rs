pub fn verse(n: u32) -> String {
    if n > 2 {
        let new_verse = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1);
        return new_verse;    
    }
    else {
        match n {
            2 => {
                let new_verse = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n - 1);
                return new_verse;
                }
            1 => {
                let new_verse = format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n);
                return new_verse;
                }
            _ => {
                let new_verse = String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
                return new_verse;    
                }
            }
        }
}

pub fn sing(start: u32, end: u32) -> String {
   let mut song = String::new();
   let mut count: u32 = start;
   while count >= end {
    song.push_str(&verse(count));
    if count == 0 {
        break;
    }
    count -= 1;
    if count >= end {
        song.push('\n');
    }    
   }
   return song;
}
