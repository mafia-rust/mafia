import React from "react";
import translate, { getChatElement } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import GameState, { PlayerIndex } from "../../../game/gameState.d";
import { ChatMessage } from "../../../game/chatMessage";

interface ChatMenuProps {
}

interface ChatMenuState {
    gameState: GameState,
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
    listener: () => void;

    constructor(props: ChatMenuProps) {
        super(props);
        
        this.state = {
            gameState: GAME_MANAGER.gameState,
            chatField: "",
            filterFunction: null,
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

    renderTextInput() {return (
        <div className="send-section">
            {(()=>{
                if(this.state.filterFunction === null) return null;
                return <button onClick={()=>ChatMenu.setFilterFunction(null)}>
                    {translate("menu.chat.clearFilter")}
                </button>
            })()}
            <div>
                <textarea
                    value={this.state.chatField}
                    onChange={this.handleInputChange}
                    onKeyPress={this.handleInputKeyPress}
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
                    {this.state.gameState.chatMessages.filter((msg)=>
                        this.state.filterFunction?this.state.filterFunction(msg):true
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

