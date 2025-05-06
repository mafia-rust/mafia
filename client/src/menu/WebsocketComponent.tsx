import { ReactElement } from "react";
import { useWebSocketContext, WebsocketContext } from "./WebsocketContext";

export default function WebsocketComponent(props: {children: JSX.Element}): ReactElement{

    const ctx = useWebSocketContext();

    return <WebsocketContext.Provider value={ctx}>
        {props.children}
    </WebsocketContext.Provider>
}