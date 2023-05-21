import React from "react";
import translate, { getChatElement } from "../../../game/lang";
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
            chatField: "/w " + (playerIndex + 1) + " " + ChatMenu.instance!.state.chatField,
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
                let textSplit = text.split(' ');
                if(textSplit.length < 2){
                    throw new Error("Invalid whisper");
                }
                const playerIndex = parseInt(textSplit[1]) - 1;
                if (isNaN(playerIndex)) {
                    throw new Error("Invalid player index");
                }
                const message = text.substring(textSplit[0].length + textSplit[1].length + 2);
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

    

    renderChatMessage(msg: ChatMessage, i: number) {return (
        // <div key={i}>
            getChatElement(msg)
        // </div>
    );}

    renderTextInput() {return (
        <div className="send-section">
            <textarea
                style={{color:"black"}}
                value={this.state.chatField}
                onChange={this.handleInputChange}
                onKeyPress={this.handleInputKeyPress}
            />
            <button onClick={this.sendChatField}>
                {translate("menu.chat.button.send")}
            </button>
        </div>
    );}

    render(){return(
        <div className="chat-menu">
            <div className="message-section">
                {this.state.gameState.chatMessages.map((msg, i) => {
                    return this.renderChatMessage(msg, i);
                })}
                {/* <br ref={(el) => { this.bottomOfPage = el; }}/> */}
            </div>
            {this.renderTextInput()}
        </div>
    )}
}
