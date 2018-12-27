#[derive(Debug)]
struct Conference {
    tickets: Vec<String>
}

#[derive(Debug)]
struct MemoryBuffer {
    buffer: [u8; 1024],
    permissions: u32,
    address: usize
}

fn main() {
    let conf = Conference {
        tickets: vec![
            "Jane".to_string(),
            "Max".to_string(),
            "Mario".to_string(),
            "Bob".to_string()
        ]
    };

    let some_buffer = MemoryBuffer {
        buffer: []
    }

    println!("{:?}", conf);
}
