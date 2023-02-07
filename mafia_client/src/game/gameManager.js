import { create_gameState } from "./gameState";

console.log("gameManager open");

let gameManager = create_gameManager();
//gameManager.Server.open();

function create_gameManager(){

    
    return {
        //ws: ws,
        lobbyName: null,
        myName: null,

        Server : create_server(),

        gameState : create_gameState(),

        host_button : () => {
            gameManager.Server.send("Host");
        },
        join_button: () => {
            gameManager.Server.send("Join");
        },

        messageListener: (serverMessage)=>{
            console.log(serverMessage);

            //need to do rust match statement here
        },
    }
}
function create_server(){
 

    let Server = {
        ws: null,

        openListener : (event)=>{
            //Server.ws.send("Hello to Server");
        },
        closeListener : (event)=>{
            console.log(event);
        },
        messageListener: (event)=>{
            console.log("Server: "+event.data);

            gameManager.messageListener(
                JSON.parse(event.data)
            );
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
        send : (packets)=>{
            Server.ws.send(packets);
        },
        close : ()=>{
            if(Server.ws==null) return;
            
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


/*
rust side code of packets i need to make
pub enum ToServerPacket{
    
    Join
    Host

    //
    StartGame,
    Kick,
    SetRoleList,
    SetPhaseTimes,
    SetInvestigatorResults,

    //
    Vote,   //Accusation
    Target,
    DayTarget,
    Judgement,  //Vote
    Whisper,
    SendMessage,
    SaveWill,
}
*/

/*
"Join": {
    "name": "Sammy"
}
*/