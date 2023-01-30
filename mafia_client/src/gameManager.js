console.log("gameManager open");

let gameManager = create_gameManager();

function create_gameManager(){

    let ws = new WebSocket("ws://127.0.0.1:8081");
    ws.onopen = (event) => {
        ws.send("Hello to Server");
    };
    ws.onmessage = (event) => {
        console.log("Server: "+event.data);
    }
    ws.onclose = (event) =>{
        console.log(event);
    }

    return {
        ws: ws,
        myName: null,
        lobbyName: null,
    }
}

export default gameManager;