import React from "react";
import { getChatString } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import GameState, { PlayerIndex } from "../../../game/gameState.d";
import { ChatMessage } from "../../../game/net/chatMessage";

interface ChatMenuProps {
}

interface ChatMenuState {
    gameState: GameState,
    chatField: string,
}

export default class ChatMenu extends React.Component<ChatMenuProps, ChatMenuState> {
    bottomOfPage: HTMLBRElement | null;

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
        this.bottomOfPage = null;

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

    componentDidUpdate() {
        if(this.bottomIsInViewport(500))   //used to be 500
            this.scrollToBottom();
    }

    scrollToBottom() {
        this.bottomOfPage?.scrollIntoView({ behavior: "smooth" });
    }
    bottomIsInViewport(offset = 0) {
        if (!this.bottomOfPage) return false;
        const top = this.bottomOfPage.getBoundingClientRect().top;
        //if top is between 0 and height then true
        //else false
        return (top + offset) >= 0 && (top - offset) <= window.innerHeight;
    }


    handleInputChange = (event: { target: { value: string; }; }) => {
        //turns all 2 spaces into 1 space. turn all tabs into 1 space. turn all new lines into 1 space
        const value = event.target.value.replace(/  +/g, ' ').replace(/\t/g, ' ').replace(/\n/g, ' ');

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
                Send LANG TODO
            </button>
        </div>
    );}

    renderChatMessage(msg: ChatMessage, i: number) {return (
        //gets the type of chat message and adds it as a subclassName to the chat-message
        <div key={i} className={"chat-message " + msg.type}>
            {getChatString(msg)}
        </div>
    );}
    //renders the chat menu with each chat message having the subclassName of chat-message

    render(){return (
        <div className="chat-menu">
            <div className="chat-messages">
                {this.state.gameState.chatMessages.map((msg, i) => {
                    return this.renderChatMessage(msg, i);
                })}
                <br ref={(el) => { this.bottomOfPage = el; }}/>
            </div>
            {this.renderTextInput()}
        </div>
    );}
    
}
