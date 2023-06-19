import React from "react";
import translate, { getChatElement } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import { PlayerIndex } from "../../../game/gameState.d";
import { ChatMessage } from "../../../game/chatMessage";

interface ChatMenuProps {
}

interface ChatMenuState {
    chatMessages: ChatMessage[],
    chatField: string,
    filterFunction: ((message: ChatMessage) => boolean) | null,
}

export default class ChatMenu extends React.Component<ChatMenuProps, ChatMenuState> {
    static prependWhisper(playerIndex: PlayerIndex) {
        
        if(ChatMenu.instance === null)
            return;
        ChatMenu.instance!.setState({
            chatField: "/w " + (playerIndex + 1) + " " + ChatMenu.instance!.state.chatField,
        });
    }
    static setFilterFunction(func: null | ((message: ChatMessage) => boolean)) {
        if(ChatMenu.instance === null)
            return;
        ChatMenu.instance!.setState({
            filterFunction: func,
        });
    }

    static instance: ChatMenu | null = null;
    messageSection: HTMLDivElement | null;
    history: ChatHistory = new ChatHistory(40);
    history_poller: ChatHistoryPoller = new ChatHistoryPoller();
    listener: () => void;

    constructor(props: ChatMenuProps) {
        super(props);
        
        this.state = {
            chatMessages: GAME_MANAGER.gameState.chatMessages,
            chatField: "",
            filterFunction: null,
        };

        this.listener = () => {
            let atTop = this.messageSection !== null && this.messageSection.scrollTop >= this.messageSection.scrollHeight - this.messageSection.clientHeight - 100;            
            this.setState({
                chatMessages: GAME_MANAGER.gameState.chatMessages
            }, () => {
                if(this.messageSection !== null && atTop){
                    this.messageSection.scrollTop = this.messageSection.scrollHeight;
                }
            });
        };
        this.messageSection = null;
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener("addChatMessages", this.listener);
        ChatMenu.instance = this;
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener("tick", this.listener);
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
                if(this.state.filterFunction === null) return null;
                return <button className="highlighted" onClick={()=>ChatMenu.setFilterFunction(null)}>
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
            <div className="message-section" ref={(el) => { this.messageSection = el; }}>
                <div className="message-list">
                    {this.state.chatMessages.filter((msg) =>
                        this.state.filterFunction ? this.state.filterFunction(msg) : true
                    ).map((msg, index) => {
                        return getChatElement(msg, index);
                    })}
                </div>
            </div>
            {this.renderTextInput()}
        </div>
    )}
}

/**
 * Traverse any props.children to get their combined text content.
 *
 * This does not add whitespace for readability: `<p>Hello <em>world</em>!</p>`
 * yields `Hello world!` as expected, but `<p>Hello</p><p>world</p>` returns
 * `Helloworld`, just like https://mdn.io/Node/textContent does.
 *
 * NOTE: This may be very dependent on the internals of React.
 */
export function textContent(elem: React.ReactElement | string): string {
    if (!elem) {
        return '';
    }
    if (typeof elem === 'string') {
        return elem;
    }
    // Debugging for basic content shows that props.children, if any, is either a
    // ReactElement, or a string, or an Array with any combination. Like for
    // `<p>Hello <em>world</em>!</p>`:
    //
    //   $$typeof: Symbol(react.element)
    //   type: "p"
    //   props:
    //     children:
    //       - "Hello "
    //       - $$typeof: Symbol(react.element)
    //         type: "em"
    //         props:
    //           children: "world"
    //       - "!"
    const children = elem.props && elem.props.children;
    if (children instanceof Array) {
        return children.map(textContent).join('');
    }
    return textContent(children);
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
