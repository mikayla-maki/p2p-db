//Implementation inspired by studying the kafka tutorials on confluent: 
//https://www.confluent.io/blog/kafka-streams-tables-part-1-event-streaming/

use bytes::Bytes;
use std::path::Path;

struct LogSegment {
    logical_timestamp: u64,
    mesage: Message
}

struct Message {
    key: Bytes,
    msg: Bytes
}

struct Log {
    cur_timestamp: u64,
    contents: Vec<LogSegment> //Concurrent-ize this? 
}

impl Log {
    fn new() -> Log {
        Log {
            cur_timestamp: 0,
            contents: vec![]
        }
    }

    fn subscribe(start: usize) -> {

    }
}