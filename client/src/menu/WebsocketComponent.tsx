import { ReactElement } from "react";
import { useWebSocketContext, WebsocketContext } from "./WebsocketContext";

export default function WebsocketComponent(): ReactElement{

    const ctx = useWebSocketContext();

    return <WebsocketContext.Provider value={ctx}>
        {ctx.content}
    </WebsocketContext.Provider>
}