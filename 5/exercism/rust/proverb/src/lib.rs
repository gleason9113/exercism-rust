use std::collections::HashMap;

pub fn build_proverb(list: &[&str]) -> String {
    let poem = build_map();
    let options = vec!["shoe", "horse", "rider", "message", "battle", "kingdom", "nail"];
    let mut answer = String::new();
    

    for key in options {
        if list.contains(&key) {
            match poem.get(&key as &str) {
                Some(line) => answer.push_str(line),
                None => println!("No key found!"),
            }
        }
    }
    return answer;
    
}

pub fn build_map() -> HashMap<String, String> {
    let mut lines = HashMap::new();
    
    lines.insert(
        "shoe".to_owned(),
        "For want of a nail the shoe was lost.\n".to_owned(),
    );    
    
    lines.insert(
        "horse".to_owned(),
        "For want of a shoe the horse was lost.\n".to_owned(),
    );

    lines.insert(
        "rider".to_owned(),
        "For want of a horse the rider was lost.\n".to_owned(),
    );

    lines.insert(
        "message".to_owned(),
        "For want of a rider the message was lost.\n".to_owned(),
    );

    lines.insert(
        "battle".to_owned(),
        "For want of a message the battle was lost.\n".to_owned(),
    );

    lines.insert(
        "kingdom".to_owned(),
        "For want of a battle the kingdom was lost.\n".to_owned(),
    );

    lines.insert(
        "nail".to_owned(),
        "And all for the want of a nail.".to_owned(),
    );
    
    return lines;
}
