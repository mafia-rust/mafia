import { ReactElement, useContext, useEffect } from "react";
import { useWebSocketContext, WebsocketContext, WebSocketContextType } from "./WebsocketContext";
import React from "react";
import { ToClientPacket } from "../game/packet";
import { AnchorContext } from "./AnchorContext";
import translate from "../game/lang";

export default function WebsocketComponent(): ReactElement{
    const ctx = useWebSocketContext();

    useEffect(()=>{
        if(ctx.lastMessageRecieved){
            useMessageListener(ctx.lastMessageRecieved, ctx);
        }
    }, [ctx, ctx.lastMessageRecieved]);

    return <WebsocketContext.Provider value={ctx}>
        {ctx.content}
    </WebsocketContext.Provider>
}

function useMessageListener(packet: ToClientPacket, websocketContext: WebSocketContextType){
    console.log(JSON.stringify(packet, null, 2));

    const anchorController = useContext(AnchorContext)!;

    switch(packet.type) {
        case "pong":
            websocketContext.sendPacket({
                type: "ping"
            });
        break;
        case "rateLimitExceeded":
            anchorController.pushErrorCard({ title: translate("notification.rateLimitExceeded"), body: "" });
        break;
        case "forcedOutsideLobby":
            websocketContext.setContent({type:"gameBrowser"});
        break;
        case "forcedDisconnect":
            anchorController.setContent({type:"main"});
        break
        default:
            console.error(`incoming message response not implemented: ${(packet as any)?.type}`);
            console.error(packet);
        break;
    }
}


