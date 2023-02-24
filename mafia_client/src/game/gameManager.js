import { create_gameState } from "./gameState";
import { Main } from "../Main";
import { LobbyMenu } from "../openMenus/LobbyMenu";
import { PlayerListMenu } from "../gameMenus/PlayerListMenu";
import { StartMenu } from "../openMenus/StartMenu";

console.log("gameManager open");

let gameManager = create_gameManager();
//gameManager.Server.open();

function create_gameManager(){

    
    let gameManager = {
        roomCode: null,

        Server : create_server(),

        gameState : create_gameState(),

        listeners : [],
        addStateListner : (listener)=>{
            gameManager.listeners.push(listener);
        },
        removeStateListner : (listener)=>{
            gameManager.listeners.splice(gameManager.listeners.indexOf(listener));
        },
        invokeStateListners : ()=>{
            for(let i = 0; i < gameManager.listeners.length; i++){
                if(gameManager.listeners[i].func){
                    gameManager.listeners[i].func();
                }
            }
        },

        host_button : () => {
            gameManager.Server.send(`"Host"`);
        },
        join_button: () => {
            gameManager.Server.send(`"Join"`);
        },


        setName_button: (name)=>{
            if(name)
                gameManager.Server.send(JSON.stringify({
                    "SetName":{
                        "name":name
                    }
                }, null, false));
        },
        startGame_button: ()=>{
            gameManager.Server.send(`"StartGame"`);
        },
        phaseTimes_button: (morning, discussion, voting, testimony, judgement, evening, night)=>{
            gameManager.Server.send(JSON.stringify({
                "SetPhaseTimes":{
                    "phase_times":{
                        "morning": {"secs":morning, "nanos":0},
                        "discussion": {"secs":discussion, "nanos":0},
                        "voting": {"secs":voting, "nanos":0},
                        "testimony": {"secs":testimony, "nanos":0},
                        "judgement": {"secs":judgement, "nanos":0},
                        "evening": {"secs":evening, "nanos":0},
                        "night": {"secs":night, "nanos":0},
                    }
                }
            }, null, false))
        },

        messageListener: (serverMessage)=>{

            let type;
            if(typeof(serverMessage)==="string"){
                type = serverMessage;
            }else{
                //object, THIS ASSUMES THAT SERVER MESSAGE IS AN OBJECT WITH AT LEAST 1 KEY
                type = Object.keys(serverMessage)[0];
                serverMessage = serverMessage[type];
            }


            //In each of the cases, ensure that your not interpreting anything as an object when it isnt.
            //on the rust side, this is an enum called ToClientPacket
            switch(type){
                case "AcceptJoin":
                    Main.instance.setState({panels : [<LobbyMenu/>]});
                break;
                case "RejectJoin":
                    let reason = serverMessage.reason
                    alert(reason);
                    
                break;
                case "AcceptHost":
                    gameManager.roomCode = serverMessage.room_code;

                    Main.instance.setState({panels : [<LobbyMenu/>]});
                break;
                case"YourName":
                    gameManager.gameState.myName = serverMessage.name;
                break;
                case "OpenGameMenu":
                    Main.instance.setState({panels : [<PlayerListMenu/>]})
                break;
                default:
                    console.log("incoming_message response not implemented "+type);
                break;
            }


            
            gameManager.invokeStateListners();
        },
    }
    return gameManager;
}
function create_server(){
 

    let Server = {
        ws: null,

        openListener : (event)=>{
            //Server.ws.send("Hello to Server");
        },
        closeListener : (event)=>{
            console.log(event);

            Main.instance.setState({panels: [<StartMenu/>]});
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