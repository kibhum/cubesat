#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}
impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}
#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;
impl GroundStation {
    // &self indicates that GroundStation.send() only requires
    // a read-only reference to self. The recipient takes a
    // mutable borrow (&mut) of the CubeSat instance, and
    // msg takes full ownership of its Message instance.
    fn send(&self, to: &mut CubeSat, msg: Message) {
        // Ownership of the Message
        // instance transfers from
        // msg to messages.push()
        // as a local variable.
        to.mailbox.messages.push(msg);
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main() {
    // Each satellite variable
    // is represented by an
    // integer.
    // let sat_a = CubeSat {
    //     id: 100,
    //     mailbox: Mailbox { messages: vec![] },
    // };
    // let sat_b = CubeSat {
    //     id: 101,
    //     mailbox: Mailbox { messages: vec![] },
    // };
    // let sat_c = CubeSat {
    //     id: 102,
    //     mailbox: Mailbox { messages: vec![] },
    // };

    // let sat_a = check_status(sat_a);
    // let sat_b = check_status(sat_b);
    // let sat_c = check_status(sat_c);

    // // "waiting" ...
    // let sat_a = check_status(sat_a);
    // let sat_b = check_status(sat_b);
    // let sat_c = check_status(sat_c);

    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };

    println!("t0: {:?}", sat_a);
    // We don’t have a
    // completely ergonomic
    // way to create Message
    // instances yet. Instead,
    // we’ll take advantage of
    // the String.from() method
    // that converts &str to
    // String (aka Message).
    base.send(&mut sat_a, Message::from("hello there!"));
    println!("t1: {:?}", sat_a);
    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);
    println!("msg: {:?}", msg);
}
