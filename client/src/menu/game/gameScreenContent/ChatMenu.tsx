import React from "react";
import { getChatString } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "./gameScreen.css";
import "./chatMenu.css"
import GameState, { PlayerIndex } from "../../../game/gameState.d";

interface ChatMenuProps {
}

interface ChatMenuState {
    gameState: GameState,
    chatField: string,
}

export default class ChatMenu extends React.Component<ChatMenuProps, ChatMenuState> {
    static prependWhisper(playerIndex: PlayerIndex) {
        
        if(ChatMenu.instance === null)
            return;
        ChatMenu.instance!.setState({
            chatField: "/w" + (playerIndex + 1) + " " + ChatMenu.instance!.state.chatField,
        });
    }

    static instance: ChatMenu | null = null;
    listener: () => void;

    constructor(props: ChatMenuProps) {
        super(props);

        this.state = {
            gameState: GAME_MANAGER.gameState,
            chatField: "",
        };

        this.listener = () => {
            this.setState({
                gameState: GAME_MANAGER.gameState
            });
        };
    }

    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
        ChatMenu.instance = this;
    }

    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    handleInputChange = (event: { target: { value: string; }; }) => {
        const value = event.target.value.trim();
        if(ChatMenu.instance === null) return;
        ChatMenu.instance.setState({
            chatField: value
        });
    };

    handleInputKeyPress(event: { code: string; preventDefault: () => void; }){
        if (event.code === "Enter") {
            event.preventDefault();
            if(ChatMenu.instance === null) return;
            ChatMenu.instance.sendChatField();
        }
    };

    sendChatField(){
        if(ChatMenu.instance === null) return;
        const text = ChatMenu.instance.state.chatField.trim();
        if (text.startsWith("/w")) {
            try {
                const indexOfFirstSpace = text.indexOf(' ');
                const playerIndex = parseInt(text.substring(2, indexOfFirstSpace)) - 1;
                if (isNaN(playerIndex)) {
                    throw new Error("Invalid player index");
                }
                const message = text.substring(indexOfFirstSpace + 1);
                GAME_MANAGER.sendSendWhisperPacket(playerIndex, message);
            } catch (e) {
                GAME_MANAGER.sendSendMessagePacket(text);                
            }
        } else {
            if (text.replace("\n", "").replace("\r", "").trim() !== "") {
                GAME_MANAGER.sendSendMessagePacket(text);
            }
        }
        ChatMenu.instance.setState({
            chatField: ""
        });
    };

    calcInputHeight = (value: string) => {
        const numberOfLineBreaks = (value.match(/\n/g) || []).length;
        // min-height + lines x line-height + padding + border
        const newHeight = 20 + numberOfLineBreaks * 20 + 12 + 2;
        return newHeight;
    };

    renderTextInput() {return (
        <div className="chat-input-container">
            <textarea
                className="chat-input"
                value={this.state.chatField}
                onChange={this.handleInputChange}
                onKeyPress={this.handleInputKeyPress}
                style={{ height: this.calcInputHeight(this.state.chatField) }}
            />
            <button
                className="gm-button"
                onClick={this.sendChatField}
                >
                Send
            </button>
        </div>
    );}

    renderChatMessage(msg: string, i: number) {return (
        <div key={i} className="chat-message">
            {getChatString(msg)}
        </div>
    );}

    render(){return (
        <div className="chat-menu">
            <div className="chat-messages">
                {this.state.gameState.chatMessages.map((msg, i) => {
                    return this.renderChatMessage(msg, i);
                })}
            </div>
            {this.renderTextInput()}
        </div>
    );}
}
