// 27. Create a `Message` enum for different types of notifications.
#[derive(Debug)]

enum Message{
    Text(String),
    Email {from :String, subject:String},
    Alert(String),
    FriendRequest{from:String},
    DownloadComplete{filename:String, size_mb:u32},
}

impl Message{
    fn display(&self){
        match self{
            Message::Text(content) => {
                println!("Text message: {}", content);
            }
            Message::Email{from,subject} => {
                println!("email from {}: {}", from, subject);
            }
            Message::Alert(content) => {
                println!("Alert: {}", content);
            }
            Message::FriendRequest{from} => {
                println!("Friend request from {}", from);
            }
            Message::DownloadComplete{filename, size_mb} => {
                println!("Download complete: {} ({} MB)", filename, size_mb);
            }
        }
    }
}

fn main() {
    let messages = vec![
        Message::Text(String::from("Hey, are you free tonight?")),
        Message::Email {
            from: String::from("admin@rustlang.org"),
            subject: String::from("Your account has been approved"),
        },
        Message::Alert(String::from("Battery low")),
        Message::FriendRequest {
            from: String::from("alice123"),
        },
        Message::DownloadComplete {
            filename: String::from("rust_book.pdf"),
            size_mb: 8,
        },
    ];

    for msg in messages {
        msg.display();
    }
}