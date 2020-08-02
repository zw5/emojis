use std::net::Ipv6Addr;
use std::env;
use clipboard::*;

const SEPARATOR: char = '|';
const EMOJIS: [&str;256] = ["😀", "😃", "😄", "😁", "😆", "😅", "😂", "🤣", "😊", "😇", "🙂", "🙃", "😉", "😌", "😍", "🥰", "😘", "😗", "😙", "😚", "😋", "😛", "😝", "😜", "🤪", "🤨", "🧐", "🤓", "😎", "🤩", "😏", "😒", "😞", "😔", "😟", "☹️", "😣", "😖", "😫", "😭", "😤", "😠", "😡", "🤬", "🤯", "😳", "🥵", "🥶", "😱", "😨", "😰", "😥", "😓", "🤗", "🤔", "🤭", "🤫", "🤥", "😶", "😐", "😑", "😬", "🙄", "😯", "😦", "😧", "😮", "😲", "😴", "🤤", "😪", "😵", "🤐", "🥴", "🤢", "🤮", "🤧", "😷", "🤒", "🤕", "🤑", "🤠", "😈", "👿", "👹", "👺", "🤡", "💩", "👻", "💀", "☠️", "👽", "👾", "🤖", "🎃", "😺", "😸", "😹", "😻", "😼", "😽", "🙀", "😿", "😾", "👋", "🤚", "🖐", "✋", "🖖", "👌", "✌️", "🤞", "🤟", "🤘", "🤙", "👈", "👉", "👆", "🖕", "👇", "☝️", "👍", "👎", "✊", "👊", "🤛", "🤜", "👏", "🙌", "👐", "🤲", "🤝", "🙏", "✍️", "💅", "🤳", "💪", "🦵", "🦶", "👂", "👃", "🧠", "🦷", "🦴", "👀", "👁", "👅", "👄", "💋", "👶", "🧒", "👦", "👧", "🧑", "👱", "👨", "🧔", "👩", "🧳", "🌂", "☂️", "🧵", "🧶", "👓", "🕶", "🥽", "🥼", "👔", "👕", "👖", "🧣", "🧤", "🧥", "🧦", "👗", "👘", "👙", "👚", "👛", "👜", "👝", "🎒", "👞", "👟", "🥾", "🥿", "👠", "👡", "👢", "👑", "👒", "🎩", "🎓", "🧢", "⛑", "💄", "💍", "💼", "🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼", "🐨", "🐯", "🦁", "🐮", "🐷", "🐽", "🐸", "🐵", "🙈", "🙉", "🙊", "🐦", "🐤", "🐣", "🐥", "🦆", "🦅", "🦉", "🦇", "🐺", "🐗", "🐴", "🦄", "🐝", "🕷", "🕸", "🦂", "🐢", "🐍", "🦎", "🦖", "🦕", "🐙", "🦑", "🐠", "🐟", "🐬", "🐳", "🐅", "🐆", "🦓", "🐃", "🐂", "🐄", "🐎", "🐖", "🐏", "🐑", "🦙", "🐐"];

fn main() {
    let mut args = env::args();
    let mode: &str = &args.nth(1).unwrap_or("-c".into());
    match mode {
        "-r" | "-b" => {
            let mut final_res = String::new();
            let emoji_addr = env::args().nth(2).unwrap();

            let list_thing: Vec<&str> = emoji_addr.split(&SEPARATOR.to_string()).collect();
            let mut first: Option<u8> = None;
            println!("{}", list_thing.len());
            for emoji in list_thing {
                println!("{}", emoji);
                if emoji == ":" {
                    final_res.push(':');
                    continue
                };
                let v8 = EMOJIS.iter().position(|&r| r == emoji).unwrap_or(0) as u8;
                match first {
                    Some(num) => {
                        let number = ((v8 as u16) << 8) | num as u16;
                        final_res.push_str(&format!("{:x}", &number).to_string());
                        first = None;
                    },
                    None => {
                        first = Some(v8);
                    },
                }
            }
            println!("Final res {}", final_res)
        },
        _ => {
            let mut result: String = "".into();
            let address: Ipv6Addr = env::args().nth(1).unwrap().parse().unwrap();
            let segments = address.segments();
            let addrs_iter = segments.iter();
            for segment in addrs_iter {
                let first = *segment as u8;
                let second = (*segment >> 8) as u8;
                result.push_str(turn_u8_into_emoji(first));
                result.push(SEPARATOR);
                result.push_str(turn_u8_into_emoji(second));
                result.push(SEPARATOR);
                result.push(':');
                result.push(SEPARATOR);
            }
            result.pop();
            result.pop();
            println!("{}", result);
            let mut ctx = ClipboardContext::new().expect("No clipboard found");
            ctx.set_contents(result).expect("Failed to set clipboard");
        },
    }
}


fn turn_u8_into_emoji<'a>(value: u8) -> &'a str {
    let index = value as usize;
    if value == 0 {
        return "".into()
    };
    EMOJIS[index].into()
}