import { create_gameState, create_grave, create_player } from "./gameState";
import { Main } from "../Main";
import { LobbyMenu } from "../openMenus/LobbyMenu";
import { PlayerListMenu } from "../gameMenus/PlayerListMenu";
import { StartMenu } from "../openMenus/StartMenu";
import gameManager from "../index.js";
import { ChatMenu } from "../gameMenus/ChatMenu";
import { PhaseRowMenu } from "../gameMenus/PhaseRowMenu";
import { TitleMenu } from "../openMenus/TitleMenu";
import { WillMenu } from "../gameMenus/WillMenu";
import { GraveyardMenu } from "../gameMenus/GraveyardMenu";


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
        invokeStateListeners : ()=>{
            for(let i = 0; i < gameManager.listeners.length; i++){
                if(typeof(gameManager.listeners[i])==="function"){
                    gameManager.listeners[i]();
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

                //InLobby/Game

                
                case"YourName":
                    gameManager.gameState.myName = serverMessage.name;
                break;
                case"YourPlayerIndex":
                    gameManager.gameState.myIndex = serverMessage.player_index;
                break;
                case"Players":
                    for(let i = 0; i < serverMessage.names.length; i++){
                        if(gameManager.gameState.players.length > i){
                            gameManager.gameState.players[i].name = serverMessage.names[i];
                        }else{
                            //if this player index isnt in the list, create a new player and then sync
                            gameManager.gameState.players.push(create_player());
                            gameManager.gameState.players[i].name = serverMessage.names[i];
                        }
                    }
                break;
                case"Kicked":
                    gameManager.gameState = create_gameState();
                    Main.instance.setState({
                        rows: [<TitleMenu/>],
                        panels : [<StartMenu/>]
                    })
                break;
                case "OpenGameMenu":
                    Main.instance.setState({
                        panels : [<GraveyardMenu/> ,<ChatMenu/>, <PlayerListMenu/>, <WillMenu/>],
                        rows : [<PhaseRowMenu/>]
                    });
                break;
                case"PhaseTimes":
                    gameManager.gameState.phaseTimes.morning    = serverMessage.phase_times.morning.secs;
                    gameManager.gameState.phaseTimes.discussion = serverMessage.phase_times.discussion.secs;
                    gameManager.gameState.phaseTimes.voting     = serverMessage.phase_times.voting.secs;
                    gameManager.gameState.phaseTimes.testimony  = serverMessage.phase_times.testimony.secs;
                    gameManager.gameState.phaseTimes.judgement  = serverMessage.phase_times.judgement.secs;
                    gameManager.gameState.phaseTimes.evening    = serverMessage.phase_times.evening.secs;
                    gameManager.gameState.phaseTimes.night      = serverMessage.phase_times.night.secs;
                break;
                case"Phase":
                    gameManager.gameState.phase = serverMessage.phase;
                    gameManager.gameState.dayNumber = serverMessage.day_number;
                    gameManager.gameState.secondsLeft = serverMessage.seconds_left;
                break;
                case"PlayerOnTrial":
                    gameManager.gameState.playerOnTrial = serverMessage.player_index;
                break;
                case"PlayerButtons":
                    for(let i = 0; i < gameManager.gameState.players.length && i < serverMessage.buttons.length; i++){
                        gameManager.gameState.players[i].buttons.vote = serverMessage.buttons[i].vote;
                        gameManager.gameState.players[i].buttons.target = serverMessage.buttons[i].target;
                        gameManager.gameState.players[i].buttons.dayTarget = serverMessage.buttons[i].day_target;
                    }
                break;
                case"PlayerAlive":
                    for(let i = 0; i < gameManager.gameState.players.length && i < serverMessage.alive.length; i++){
                        gameManager.gameState.players[i].alive = serverMessage.alive[i];
                    }
                break;
                case"PlayerVotes":
                    for(let i = 0; i < gameManager.gameState.players.length && i < serverMessage.voted_for_player.length; i++){
                        gameManager.gameState.players[i].numVoted = serverMessage.voted_for_player[i];
                    }
                break;
                case"YourWill":
                    gameManager.gameState.will = serverMessage.will;
                break;
                case"YourRole":
                    gameManager.gameState.role = serverMessage.role;
                break;
                case"YourTarget":
                    gameManager.gameState.targets = serverMessage.player_indices;
                break;
                case"YourVoting":
                    gameManager.gameState.voted = serverMessage.player_index;
                break;
                case"YourJudgement":
                    gameManager.gameState.judgement = serverMessage.verdict;
                break;
                case"AddChatMessages":
                    for(let i = 0; i < serverMessage.chat_messages.length; i++){
                        gameManager.gameState.chatMessages.push(serverMessage.chat_messages[i]);
                    }
                break;
                case"AddGrave":
                    let grave = create_grave();
                    grave.playerIndex = serverMessage.grave.player_index;
                    grave.role =        serverMessage.grave.role;
                    grave.killer =      serverMessage.grave.killer;
                    grave.will =        serverMessage.grave.will;
                    grave.diedPhase =   serverMessage.grave.died_phase;
                    grave.dayNumber =   serverMessage.grave.day_number;

                    gameManager.gameState.graves.push(grave);
                break;
                default:
                    console.log("incoming_message response not implemented "+type);
                    console.log(serverMessage);
                break;
            }


            
            gameManager.invokeStateListeners();
        },
    
        tick : (timePassedms)=>{
            console.log("tick");
            gameManager.gameState.secondsLeft = Math.round(gameManager.gameState.secondsLeft - timePassedms/1000)
            if(gameManager.gameState.secondsLeft < 0)
                gameManager.gameState.secondsLeft = 0;
            gameManager.invokeStateListeners();
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