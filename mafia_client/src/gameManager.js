console.log("Hello Client");


let ws = new WebSocket("ws://127.0.0.1:8081");
ws.onopen = (event) => {
    ws.send("Here's some text that the server is urgently awaiting!");
};
ws.onmessage = (event) => {
    console.log(event.data);
}

let gameManager = create_gameManager();

function create_gameManager(){
    return {
        myName: null,
        lobbyName: null,
    }
}

export default gameManager;