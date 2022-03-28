//Implementation inspired by studying the kafka impl: 
// https://jaceklaskowski.gitbooks.io/apache-kafka/content/kafka-log-Log.html

use bytes::{Bytes, BytesMut, Buf, BufMut};

struct LogSegment {
    logicalTimestamp: u64,
    msg: Bytes
}

struct Log {
    contents: Vec<LogSegment> //Concurrent-ize this? 
}