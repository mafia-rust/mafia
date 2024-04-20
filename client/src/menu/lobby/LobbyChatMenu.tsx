import React, { useEffect, useState } from "react";
import { ReactElement } from "react";
import ChatElement from "../../components/ChatMessage";
import GAME_MANAGER from "../..";
import { StateListener } from "../../game/gameManager.d";
import "./lobbyChatMenu.css";
import translate from "../../game/lang";


export default function LobbyChatMenu(): ReactElement {
    const [chatMessages, setChatMessages] = useState(()=>{
        if(GAME_MANAGER.state.stateType === "lobby")
            return GAME_MANAGER.state.chatMessages
        else 
            return [];
    });
    const [chatInput, setChatInput] = useState("");

    useEffect(() => {
        const listener: StateListener = (type) => {

            
            if(GAME_MANAGER.state.stateType === "lobby"){
                switch (type) {
                    case "addChatMessages":
                        setChatMessages([...GAME_MANAGER.state.chatMessages].reverse());
                        break;
                }
            }
        }

        GAME_MANAGER.addStateListener(listener);
        return () => {GAME_MANAGER.removeStateListener(listener);}
    }, [setChatMessages]);
    
    return <section className="lobby-chat-menu chat-menu-colors selector-section">
        <h3>{translate("menu.chat.title")}</h3>
        <div className="lobby-chat-menu-chat">{/* the div that lets the one inside move expands*/}
            <div>   {/* the div that expands*/}
                {chatMessages.map((message, index) => {
                    return <div key={index}>
                        <ChatElement message={message} playerNames={[]}/>
                    </div>
                })}
            </div>
        </div>
        <div>
            <input
                type="text" 
                value={chatInput}
                onChange={(e)=>{
                    setChatInput(e.target.value);
                }}
                onKeyUp={(e)=>{
                    if(e.key !== 'Enter') return;
                    GAME_MANAGER.sendSendLobbyMessagePacket(chatInput);
                    setChatInput("");
                }}
            />
            <button
                className="material-icons-round"
                onClick={() => {
                    GAME_MANAGER.sendSendLobbyMessagePacket(chatInput);
                    setChatInput("");
                }}
            >send</button>
        </div>
    </section>
}