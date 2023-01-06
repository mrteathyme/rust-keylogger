use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};




#[derive(Debug, Default)]
enum KeyEventState {
    Release,
    #[default] Press,
    Held,
}

#[derive(Debug, Default)]
struct KeyEvent {
    code: u8,
    state: KeyEventState,
}

fn main() {
    let key_map:HashMap<u8, &str> = HashMap::from([(1, "Esc"), (2, "1"), (30, "a")]);

    let mut file_options = OpenOptions::new();
            file_options.read(true);
            file_options.write(false);
    let mut file = file_options.open("/dev/input/event8").expect("You fucked up fool");
    
    let mut read_buffer = [0u8; 24];
    let mut key_buffer= vec!();
    let mut keyevent: KeyEvent;

    let mut held_keys:HashMap<u8, bool>= HashMap::new();

    file_options.read(true);
    file_options.append(true);
    file_options.create(true);
    let mut log = file_options.open("/tmp/keylog.txt").unwrap();
    loop {
        file.read(&mut read_buffer).ok();
        key_buffer.push(read_buffer) ;
        if key_buffer.last().unwrap()[18] == 0{
            if key_buffer.len() == 2{
                if held_keys.get(&key_buffer[0][18]) == Some(&false) || held_keys.contains_key(&key_buffer[0][18]) == false {
                    keyevent = KeyEvent{code:key_buffer[0][18], state:KeyEventState::Held};
                    held_keys.insert(key_buffer[0][18], true);
                    println!("{:?} {:?}",key_map.get(&keyevent.code), keyevent.state);
                    let write_buffer = format!("{:?} {:?} \n",key_map.get(&keyevent.code), keyevent.state);
                    log.write(write_buffer.as_bytes()).ok();
                }
            }
            else if key_buffer[1][20] == 0 {
                keyevent = KeyEvent{code:key_buffer[1][18], state:KeyEventState::Release};
                held_keys.insert(key_buffer[1][18], false);
                println!("{:?} {:?}",key_map.get(&keyevent.code), keyevent.state);
                let write_buffer = format!("{:?} {:?} \n",key_map.get(&keyevent.code), keyevent.state);
                log.write(write_buffer.as_bytes()).ok();
            }
            else {
                keyevent = KeyEvent{code:key_buffer[1][18], state:KeyEventState::Press};
                println!("{:?} {:?}",key_map.get(&keyevent.code), keyevent.state);
                let write_buffer = format!("{:?} {:?} \n",key_map.get(&keyevent.code), keyevent.state);
                log.write(write_buffer.as_bytes()).ok();
            }
            key_buffer.clear();  
        }
    }
}
