import { ReactElement } from "react";
import { useWebSocketContext, WebsocketContext } from "./WebsocketContext";
import React from "react";

export function WebsocketComponent(): ReactElement{

    const ctx = useWebSocketContext();

    return <WebsocketContext.Provider value={ctx}>
        {ctx.content}
    </WebsocketContext.Provider>
}