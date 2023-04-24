import React from "react";
import { getChatString } from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./gameScreen.css";
import "./chatMenu.css"

export default class ChatMenu extends React.Component {
    static prependWhisper(playerIndex) {
        ChatMenu.instance.setState({
            chatField: "/w" + (playerIndex + 1) + " " + ChatMenu.instance.state.chatField,
        });
    }
    static instance = null;

    constructor(props) {
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

    handleInputChange = (event) => {
        const value = event.target.value.trimStart();
        ChatMenu.instance.setState({
            chatField: value
        });
    };

    handleInputKeyPress(event){
        if (event.code === "Enter") {
            event.preventDefault();
            ChatMenu.instance.sendChatField();
        }
    };

    sendChatField(){
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

    calcInputHeight = (value) => {
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

    renderChatMessage(msg, i) {return (
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
