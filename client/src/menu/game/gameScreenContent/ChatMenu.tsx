import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER, { replaceMentions } from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import { Player, PlayerIndex } from "../../../game/gameState.d";
import { ChatMessage, translateChatMessage } from "../../../components/ChatMessage";
import ChatElement from "../../../components/ChatMessage";
import { ContentTab } from "../GameScreen";
import { HistoryPoller, HistoryQueue } from "../../../history";
import { StateEventType } from "../../../game/gameManager.d";

type ChatMenuProps = {
}

type ChatMenuState = {
    chatField: string,
    filter: RegExp | null,
    topMessageIndex: number,

    chatMessages: ChatMessage[],
    players: Player[]
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
    listener: (type: StateEventType | undefined) => void;

    constructor(props: ChatMenuProps) {
        super(props);
        
        
        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                chatField: "",
                filter: null,
                topMessageIndex: 0,

                chatMessages: GAME_MANAGER.state.chatMessages,
                players: GAME_MANAGER.state.players
            };

        this.listener = (type) => {
            
            let scrollTop = this.messageSection?.scrollTop;
            let scrollHeight = this.messageSection?.scrollHeight;
            let clientHeight = this.messageSection?.clientHeight;
            let atBottom = false;

            if(clientHeight !== undefined && scrollTop !== undefined && scrollHeight !== undefined)
                atBottom = scrollHeight - 40 <= clientHeight + scrollTop;
            
            //scrollTop     //top of screen
            //scrollHeight  //max height of scrollbar, height of div off screen
            //clientHeight  //height of on screen
            //clientHeight + scrollTop = scrollheight if at bottom,


            if(GAME_MANAGER.state.stateType === "game" && type === "addChatMessages")
                this.setState({
                    chatMessages: GAME_MANAGER.state.chatMessages
                }, () => {
                    if(this.messageSection !== null && atBottom){
                        this.messageSection.scrollTop = this.messageSection.scrollHeight;
                    }
                });
            if(GAME_MANAGER.state.stateType === "game" && type === "gamePlayers")
                this.setState({
                    players: GAME_MANAGER.state.players
                });
        };
        this.messageSection = null;
        ChatMenu.instance = this;
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
        ChatMenu.instance = this;
        if(this.messageSection !== null){
            this.messageSection.scrollTop = this.messageSection.scrollHeight;
        }
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
            const recipient = this.state.players.find(player => 
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
    render(){
        ChatMenu.instance = this;

        let messages = this.state.chatMessages.filter((msg) => {
            if (this.state.filter === null) {
                return true;
            } else {
                return msg.type === "phaseChange" || this.state.filter.test(translateChatMessage(msg));
            }
        });

        return(
            <div className="chat-menu chat-menu-colors">
                <ContentTab close={false}>{translate("menu.chat.title")}</ContentTab>
                <div className="message-section" ref={(el) => { this.messageSection = el; }}>
                    <div className="message-list">
                        {messages.map((msg, index) => {
                            return <ChatElement key={index} message={msg}/>;
                        })}
                    </div>
                </div>
                {this.renderTextInput()}
            </div>
        )
    }
}
