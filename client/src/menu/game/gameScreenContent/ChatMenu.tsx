import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import GameState, { PlayerIndex } from "../../../game/gameState.d";
import { translateChatMessage } from "../../../components/ChatMessage";
import ChatElement from "../../../components/ChatMessage";
import StyledText from "../../../components/StyledText";

interface ChatMenuProps {
}

interface ChatMenuState {
    gameState: GameState,
    chatField: string,
    filter: RegExp | null,
}

export default class ChatMenu extends React.Component<ChatMenuProps, ChatMenuState> {
    static prependWhisper(playerIndex: PlayerIndex) {
        
        if(ChatMenu.instance === null)
            return;
        ChatMenu.instance!.setState({
            chatField: "/w " + (playerIndex + 1) + " " + ChatMenu.instance!.state.chatField,
        });
    }
    static setFilter(regex: RegExp | null) {
        if(ChatMenu.instance === null)
            return;
        ChatMenu.instance.setState({ filter: regex });
    }
    static getFilter(): RegExp | null {
        if(ChatMenu.instance === null)
            return null;
        return ChatMenu.instance!.state.filter;
    }

    static instance: ChatMenu | null = null;
    messageSection: HTMLDivElement | null;
    history: ChatHistory = new ChatHistory(40);
    history_poller: ChatHistoryPoller = new ChatHistoryPoller();
    listener: () => void;

    constructor(props: ChatMenuProps) {
        super(props);
        
        this.state = {
            gameState: GAME_MANAGER.gameState,
            chatField: "",
            filter: null,
        };

        this.listener = () => {
            let atTop = this.messageSection !== null && this.messageSection.scrollTop >= this.messageSection.scrollHeight - this.messageSection.clientHeight - 100;            
            this.setState({
                gameState: GAME_MANAGER.gameState
            }, () => {
                if(this.messageSection !== null && atTop){
                    this.messageSection.scrollTop = this.messageSection.scrollHeight;
                }
            });
        };
        this.messageSection = null;
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
        ChatMenu.instance = this;
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    componentDidUpdate() {
    }


    handleInputChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
        if(ChatMenu.instance === null) return;
        //turns all 2 spaces into 1 space. turn all tabs into 1 space. turn all new lines into 1 space
        const value = event.target.value.replace(/  +/g, ' ').replace(/\t/g, ' ').replace(/\n/g, ' ');

        ChatMenu.instance.setState({
            chatField: value
        });
    };
    handleInputKeyDown(event: React.KeyboardEvent<HTMLTextAreaElement>){
        if(ChatMenu.instance === null) return;
        else if (event.code === "Enter") {
            event.preventDefault();
            ChatMenu.instance.sendChatField();
        } else if (event.code === "ArrowUp") {
            event.preventDefault();
            let text = ChatMenu.instance.history_poller.poll_next(ChatMenu.instance.history);
            if (text !== undefined) 
                ChatMenu.instance.setState({ chatField: text })
        } else if (event.code === "ArrowDown") {
            event.preventDefault();
            let text = ChatMenu.instance.history_poller.poll_previous(ChatMenu.instance.history);
            if (text === undefined) 
                ChatMenu.instance.setState({ chatField: '' })
            else
                ChatMenu.instance.setState({ chatField: text })
        }
    };
    sendChatField(){
        if(ChatMenu.instance === null) return;
        const text = ChatMenu.instance.state.chatField.trim();
        ChatMenu.instance.history.push(text);
        ChatMenu.instance.history_poller.reset();
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

    renderTextInput() {return (
        <div className="send-section">
            {(()=>{
                if(this.state.filter === null) return null;
                return <button className="highlighted" onClick={()=>ChatMenu.setFilter(null)}>
                    {translate("menu.chat.clearFilter")}
                </button>
            })()}
            <div>
                <textarea
                    value={this.state.chatField}
                    onChange={this.handleInputChange}
                    onKeyDown={this.handleInputKeyDown}
                />
                <button onClick={this.sendChatField}>
                    {translate("menu.chat.button.send")}
                </button>
            </div>
        </div>
    );}
    render(){return(
        <div className="chat-menu">
            <div>
                <div>
                    <StyledText>
                        {translate("menu.chat.title")}
                    </StyledText>
                </div>
            </div>
            <div className="message-section" ref={(el) => { this.messageSection = el; }}>
                <div className="message-list">
                    {this.state.gameState.chatMessages.filter((msg) => {
                        if (this.state.filter === null) {
                            return true;
                        } else {
                            return msg.type === "phaseChange" || this.state.filter.test(translateChatMessage(msg));
                        }
                    }).map((msg, index) => {
                        return <ChatElement key={index} message={msg}/>;
                    })}
                </div>
            </div>
            {this.renderTextInput()}
        </div>
    )}
}

// A utility for keeping track of how we're polling the chat history
class ChatHistoryPoller {
    index: number; // -1 indicates we are not using the history.

    constructor() {
        this.index = -1;
    }

    reset(): ChatHistoryPoller {
        this.index = -1;
        return this;
    }

    poll_next(history: ChatHistory): string | undefined {
        this.index++;
        let result = history.poll(this.index);
        if (result === undefined) {
            this.index--;
        }
        return result;
    }

    poll_previous(history: ChatHistory): string | undefined {
        this.index--;
        if (this.index < 0) {
            this.index = -1;
            return undefined;
        } else {
            let result = history.poll(this.index);
            // History shrunk for some reason. Should be impossible but might as well account for it.
            if (result === undefined) {
                return this.poll_previous(history);
            } else {
                return result;
            }
        }
    }
}

// A queue with a max length
class ChatHistory {
    max_length: number;
    values: string[];

    constructor(max_length: number) {
        this.max_length = max_length;
        this.values = [];
    }

    poll(n: number): string | undefined {
        return this.values.at(n);
    }

    push(message: string) {
        this.values = [message].concat(this.values);
        if (this.values.length > this.max_length) {
            this.values.pop()
        }
    }
}
