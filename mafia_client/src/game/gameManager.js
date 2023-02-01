import { create_gameState } from "./gameState";

console.log("gameManager open");

let gameManager = create_gameManager();
gameManager.Server.open();

function create_gameManager(){

    

    return {
        //ws: ws,
        lobbyName: null,
        myName: null,

        Server : create_server(),

        gameState : create_gameState(),
    }
}
function create_server(){
 

    let Server = {
        ws: null,

        openListener : (event)=>{
            Server.ws.send("Hello to Server");
        },
        closeListener : (event)=>{
            console.log(event);
        },
        messageListener: (event)=>{
            console.log("Server: "+event.data);
            
        },

        open : ()=>{
            Server.ws = new WebSocket("ws://127.0.0.1:8081");
            Server.ws.addEventListener("open", (event)=>{
                Server.openListener(event);
            });
            Server.ws.addEventListener("close", (event)=>{
                Server.closeListener(event);
            });
            Server.ws.addEventListener("message", (event)=>{
                Server.messageListener(event);
            });
        },
        send : ()=>{
            Server.ws.send("Hello");
        },
        close : ()=>{
            Server.ws.close();
            Server.ws.removeEventListener("close", Server.closeListener);
            Server.ws.removeEventListener("message", Server.messageListener);
            Server.ws.removeEventListener("open", Server.openListener);
            Server.ws = null;
        }
        
    }
    return Server;
}

export default gameManager;