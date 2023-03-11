import { create_gameState } from "./gameState";
import { Main } from "../Main";
import { StartMenu } from "../openMenus/StartMenu";
import gameManager from "../index.js";
import { messageListener } from "./messageListener";
import CONFIG from "../resources/config.json"


//let gameManager = create_gameManager();
//gameManager.Server.open();

export function create_gameManager(){

    console.log("gameManager created");
    
    let gameManager = {
        roomCode: null,

        Server : create_server(),

        gameState : create_gameState(),

        listeners : [],
        addStateListener : (listener)=>{
            gameManager.listeners.push(listener);
        },
        removeStateListener : (listener)=>{
            gameManager.listeners.splice(gameManager.listeners.indexOf(listener));
        },
        invokeStateListeners : (type=null)=>{
            for(let i = 0; i < gameManager.listeners.length; i++){
                if(typeof(gameManager.listeners[i])==="function"){
                    gameManager.listeners[i](type);
                }
            }
        },

        host_button : () => {
            gameManager.Server.send(`"Host"`);
        },
        join_button: () => {
            gameManager.Server.send(JSON.stringify({
                "Join":{
                    "room_code":gameManager.roomCode
                }
            },null,false));
        },

        setName_button: (name)=>{
            // if(name)
                gameManager.Server.send(JSON.stringify({
                    "SetName":{
                        "name":name
                    }
                }, null, false));
        },
        startGame_button: ()=>{
            gameManager.Server.send(`"StartGame"`);
        },
        phaseTimesButton: (morning, discussion, voting, testimony, judgement, evening, night)=>{
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
        roleList_button: (roleListEntries)=>{
            gameManager.Server.send(JSON.stringify({
                "SetRoleList":{
                    "role_list": {
                        "role_list": roleListEntries
                    }
                }
            }, null, false));
        },

        judgement_button: (judgement)=>{
            if(judgement===1) judgement="Innocent";
            if(judgement===-1) judgement="Guilty";
            if(judgement===0) judgement="Abstain";
            gameManager.Server.send(JSON.stringify({
                "Judgement":{
                    "verdict":judgement
                }
            }, null, false));
        },
        vote_button: (votee_index)=>{
            gameManager.Server.send(JSON.stringify({
                "Vote":{
                    "player_index":votee_index
                }
            }, null, false));
        },
        target_button: (target_index_list)=>{
            gameManager.Server.send(JSON.stringify({
                "Target":{
                    "player_index_list":target_index_list
                }
            }, null, false));
        },
        dayTarget_button: (target_index)=>{
            gameManager.Server.send(JSON.stringify({
                "DayTarget":{
                    "player_index":target_index
                }
            }, null, false));
        },

        saveWill_button: (will)=>{
            gameManager.Server.send(JSON.stringify({
                "SaveWill":{
                    "will":will
                }
            }, null, false));
        },
        sendMessage_button: (text)=>{
            gameManager.Server.send(JSON.stringify({
                "SendMessage":{
                    "text":text
                }
            }, null, false));
        },
        sendWhisper_button: (playerIndex, text)=>{
            gameManager.Server.send(JSON.stringify({
                "SendWhisper":{
                    "player_index":playerIndex,
                    "text":text
                }
            }, null, false));
        },
        
        messageListener: (serverMessage)=>{
            messageListener(serverMessage);
        },
    
        tick : (timePassedms)=>{
            console.log("tick");
            gameManager.gameState.secondsLeft = Math.round(gameManager.gameState.secondsLeft - timePassedms/1000)
            if(gameManager.gameState.secondsLeft < 0)
                gameManager.gameState.secondsLeft = 0;
            gameManager.invokeStateListeners("tick");
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

            Main.instance.setContent(<StartMenu/>);
        },
        messageListener: (event)=>{
            // console.log("Server: "+event.data);

            gameManager.messageListener(
                JSON.parse(event.data)
            );
        },

        open : ()=>{
            let address = CONFIG.server_ip + ":" + CONFIG.port;
            Server.ws = new WebSocket("ws://"+address);   //TODO
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

// export default gameManager;


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