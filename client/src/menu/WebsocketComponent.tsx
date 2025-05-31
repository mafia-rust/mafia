import { ReactElement, useContext, useEffect } from "react";
import { useWebSocketContext, WebsocketContext, WebSocketContextType } from "./WebsocketContext";
import React from "react";
import { ToClientPacket } from "../game/packet";
import { AnchorContext, AnchorContextType } from "./AnchorContext";
import translate from "../game/lang";
import { deleteReconnectData, loadSettingsParsed, saveReconnectData } from "../game/localStorage";

function sendDefaultName(websocketContext: WebSocketContextType) {
    const defaultName = loadSettingsParsed().defaultName;
    if(defaultName !== null && defaultName !== undefined && defaultName !== ""){
        websocketContext.sendSetNamePacket(defaultName)
    }
} 
export default function WebsocketComponent(): ReactElement{
    const websocketContext = useWebSocketContext();
    const anchorContext = useContext(AnchorContext)!;

    useEffect(()=>{
        if(websocketContext.lastMessageRecieved){
            websocketComponentMessageListener(websocketContext.lastMessageRecieved, websocketContext, anchorContext);
        }
    }, [anchorContext, websocketContext, websocketContext.lastMessageRecieved]);

    return <WebsocketContext.Provider value={websocketContext}>
        {websocketContext.content}
    </WebsocketContext.Provider>
}

function websocketComponentMessageListener(packet: ToClientPacket, websocketContext: WebSocketContextType, anchorContext: AnchorContextType){
    console.log(JSON.stringify(packet, null, 2));

    

    switch(packet.type) {
        case "pong":
            websocketContext.sendPacket({
                type: "ping"
            });
        break;
        case "rateLimitExceeded":
            anchorContext.pushErrorCard({ title: translate("notification.rateLimitExceeded"), body: "" });
        break;
        case "forcedOutsideLobby":
            websocketContext.setContent({type:"gameBrowser"});
        break;
        case "forcedDisconnect":
            anchorContext.setContent({type:"main"});
        break
        case "acceptJoin":
            if(packet.inGame && packet.spectator){
                websocketContext.setContent({type:"loading"});
            }else if(packet.inGame && !packet.spectator){
                websocketContext.setContent({type:"loading"});
            }else{
                websocketContext.setContent({type:"lobbyScreen"});
            }
            

            // if(GAME_MANAGER.state.type === "lobby" || GAME_MANAGER.state.stateType === "game"){
            //     GAME_MANAGER.state.roomCode = packet.roomCode;
            //     GAME_MANAGER.state.myId = packet.playerId;
            // }

            saveReconnectData(packet.roomCode, packet.playerId);
            sendDefaultName(websocketContext);
            anchorContext.clearCoverCard();
        break;
        case "rejectJoin":
            switch(packet.reason) {
                case "roomDoesntExist":
                    anchorContext.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.roomDoesntExist") });
                    // If the room doesn't exist, don't suggest the user to reconnect to it.
                    deleteReconnectData();
                    anchorContext.clearCoverCard();
                break;
                case "gameAlreadyStarted":
                    anchorContext.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.gameAlreadyStarted") });
                break;
                case "roomFull":
                    anchorContext.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.roomFull") });
                break;
                case "serverBusy":
                    anchorContext.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.serverBusy") });
                break;
                case "playerTaken":
                    anchorContext.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.playerTaken") });
                break;
                case "playerDoesntExist":
                    anchorContext.pushErrorCard({ title: translate("notification.rejectJoin"), body: translate("notification.rejectJoin.playerDoesntExist") });
                break;
                default:
                    anchorContext.pushErrorCard({ title: translate("notification.rejectJoin"), body: `${packet.type} message response not implemented: ${packet.reason}` });
                    console.error(`${packet.type} message response not implemented: ${packet.reason}`);
                    console.error(packet);
                break;
            }
            deleteReconnectData();
            
        break;
        default:
            console.error(`incoming message response not implemented: ${(packet as any)?.type}`);
            console.error(packet);
        break;
    }
}


