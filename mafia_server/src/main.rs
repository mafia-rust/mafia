use mafia_server::lobby::Lobby;

fn main() {
    println!("Hello, world!");

    let lobbies: Vec<Lobby>;    // Multiple lobbies? For what?
}


/*
    options for internet communication.
    Option 1: 
        What i did last time, have different set message types.
        Each message type allows you to run a function on the other machine.
        Very generic

    Option 2:
        Syncronize values across both. 
        Have a way to just say, i want these 2 values to be in sync, then whenever it changes on the server, it updates the clients.
        This requires the concept of a machine to own a value.
            If the server owns the value then it gets to change it
            If the client owns the value then it gets to change it

    Option 3:
        Both. Do both of them at the same time.

*/
