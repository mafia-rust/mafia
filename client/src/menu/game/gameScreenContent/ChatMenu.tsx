import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER, { replaceMentions } from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import GameState, { PlayerIndex } from "../../../game/gameState.d";
import { translateChatMessage } from "../../../components/ChatMessage";
import ChatElement from "../../../components/ChatMessage";
import { ContentTab } from "../GameScreen";
import { HistoryPoller, HistoryQueue } from "../../../history";

type ChatMenuProps = {
}

type ChatMenuState = {
    gameState: GameState,
    chatField: string,
    filter: RegExp | null,
}

export default class ChatMenu extends React.Component<ChatMenuProps, ChatMenuState> {
    static prependWhisper(playerIndex: PlayerIndex) {
        
        if(ChatMenu.instance !== null){
            ChatMenu.instance.setState({
                chatField: "/w" + (playerIndex + 1) + " " + ChatMenu.instance.state.chatField,
            });
        }
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
    history: HistoryQueue<string> = new HistoryQueue(40);
    history_poller: HistoryPoller<string> = new HistoryPoller();
    listener: () => void;

    constructor(props: ChatMenuProps) {
        super(props);
        
        
        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                gameState: GAME_MANAGER.state,
                chatField: "",
                filter: null,
            };

        this.listener = () => {
            let atTop = this.messageSection !== null && this.messageSection.scrollTop >= this.messageSection.scrollHeight - this.messageSection.clientHeight - 100;            
            
            
            if(GAME_MANAGER.state.stateType === "game")
                this.setState({
                    gameState: GAME_MANAGER.state
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

        const value = event.target.value
            .replace(/  +/g, ' ')
            .replace(/\t/g, ' ')
            .replace(/\n/g, ' ');

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
            let text = ChatMenu.instance.history_poller.poll(ChatMenu.instance.history);
            if (text !== undefined) 
                ChatMenu.instance.setState({ chatField: text })
        } else if (event.code === "ArrowDown") {
            event.preventDefault();
            let text = ChatMenu.instance.history_poller.pollPrevious(ChatMenu.instance.history);
            if (text === undefined) 
                ChatMenu.instance.setState({ chatField: '' })
            else
                ChatMenu.instance.setState({ chatField: text })
        }
    };
    sendChatField(){
        if(ChatMenu.instance === null) return;
        let text = ChatMenu.instance.state.chatField.replace("\n", "").replace("\r", "").trim();
        if (text === "") return;
        ChatMenu.instance.history.push(text);
        ChatMenu.instance.history_poller.reset();
        if (text.startsWith("/w")) {
            const recipient = ChatMenu.instance.state.gameState.players.find(player => 
                RegExp(`^${player.index+1} +`).test(text.substring(2))
            );
            if (recipient !== undefined) {
                let whisperText = text.substring(3 + recipient.index.toString().length);
                if(GAME_MANAGER.state.stateType === "game")
                    whisperText = replaceMentions(whisperText, GAME_MANAGER.state.players);
                
                GAME_MANAGER.sendSendWhisperPacket(
                    recipient.index,
                    whisperText
                ); 
            } else {
                // Malformed whisper
                if(GAME_MANAGER.state.stateType === "game")
                    text = replaceMentions(text, GAME_MANAGER.state.players);
                GAME_MANAGER.sendSendMessagePacket(text);
            }
        } else {
            if(GAME_MANAGER.state.stateType === "game")
                text = replaceMentions(text, GAME_MANAGER.state.players);
            GAME_MANAGER.sendSendMessagePacket(text);
        }
        ChatMenu.instance.setState({
            chatField: ""
        });
    };

    renderTextInput() {return (
        <div className="send-section">
            {this.state.filter && <button 
                onClick={()=>{
                    GAME_MANAGER.invokeStateListeners("tick")
                    ChatMenu.setFilter(null)
                }}
                className="material-icons-round highlighted"
                aria-label={translate("menu.chat.clearFilter")}
            >
                filter_alt_off
            </button>}
            <div>
                <textarea
                    value={this.state.chatField}
                    onChange={this.handleInputChange}
                    onKeyDown={this.handleInputKeyDown}
                />
                <button 
                    className="material-icons-round"
                    onClick={this.sendChatField}
                    aria-label={translate("menu.chat.button.send")}
                >
                    send
                </button>
            </div>
        </div>
    );}
    render(){return(
        <div className="chat-menu chat-menu-colors">
            <ContentTab close={false}>{translate("menu.chat.title")}</ContentTab>
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
