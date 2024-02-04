import React, { ReactElement, useCallback, useEffect, useMemo, useRef, useState } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER, { replaceMentions } from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import { Player, PlayerIndex } from "../../../game/gameState.d";
import { ChatMessage, translateChatMessage } from "../../../components/ChatMessage";
import ChatElement from "../../../components/ChatMessage";
import { ContentTab } from "../GameScreen";
import { HistoryPoller, HistoryQueue } from "../../../history";
import { StateListener } from "../../../game/gameManager.d";

type ChatMenuProps = {
}

type ChatMenuState = {
    filter: RegExp | null,
}

export default class ChatMenu extends React.Component<ChatMenuProps, ChatMenuState> {
    static instance: ChatMenu | null = null;
    static prependWhisper: (index: PlayerIndex) => void = () => {};

    static setFilter(regex: RegExp | null) {
        ChatMenu.instance?.setState({ filter: regex });
    }
    static getFilter(): RegExp | null {
        return ChatMenu.instance?.state.filter ?? null;
    }

    constructor(props: ChatMenuProps) {
        super(props);
        
        
        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                filter: null,
            };
    }
    componentDidMount() {
        ChatMenu.instance = this;
    }
    componentWillUnmount() {
        ChatMenu.instance = null;
    }
    render(){
        return <div className="chat-menu chat-menu-colors">
            <ContentTab close={false}>{translate("menu.chat.title")}</ContentTab>
            <ChatMessageSection filter={this.state.filter} />
            {this.state.filter && <button 
                onClick={()=>{
                    // TODO: Sammy wtf??
                    GAME_MANAGER.invokeStateListeners("tick")
                    ChatMenu.setFilter(null)
                }}
                className="material-icons-round highlighted"
                aria-label={translate("menu.chat.clearFilter")}
            >
                filter_alt_off
            </button>}
            <ChatTextInput setWhisperRef={setWhisper => ChatMenu.prependWhisper = setWhisper}/>
        </div>
    }
}

function ChatMessageSection(props: { filter: RegExp | null }): ReactElement {
    const [messages, setMessages] = useState<ChatMessage []>(GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.chatMessages : []);
    const self = useRef<HTMLDivElement>(null);

    const filteredMessages = useMemo(() => {
        if (props.filter === null) return messages;
        else return messages.filter(msg => props.filter?.test(translateChatMessage(msg)) || msg.type === "phaseChange")
    }, [messages, props.filter]);

    // Keep chat scrolled to bottom
    useEffect(() => {
        if (self.current === null) return;
        const el = self.current;

        const scrollDistanceFromTop = el.scrollTop;
        const totalHeight = el.scrollHeight;
        const visibleHeight = el.clientHeight;

        // If at bottom: scrollDistanceFromTop + visibleHeight = totalHeight
        
        const AT_BOTTOM_THRESHOLD_PIXELS = 40;
        const lastMessage = (self.current.firstChild?.lastChild ?? null) as HTMLElement | null;

        const scrollThreshold = AT_BOTTOM_THRESHOLD_PIXELS + (lastMessage ? lastMessage.scrollHeight : 0)

        if (scrollDistanceFromTop + visibleHeight >= totalHeight - scrollThreshold) {
            el.scrollTop = totalHeight;
        }

    }, [self, messages, props.filter])

    // Update with new messages
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game" && type === "addChatMessages") {
                setMessages(GAME_MANAGER.state.chatMessages.filter(msg => 
                    props.filter === null || msg.type === "phaseChange" || props.filter.test(translateChatMessage(msg))
                ))
            }
        }

        GAME_MANAGER.addStateListener(stateListener);
        return () => GAME_MANAGER.removeStateListener(stateListener);
    }, [props.filter, setMessages]);

    //scroll chat to bottom when filter is shut off or loaded
    useEffect(() => {
        if (self.current === null) return;
        self.current.scrollTop = self.current.scrollHeight;
    }, [props.filter])

    return <div className="message-section" ref={self}>
        <div className="message-list">
            {filteredMessages.map((msg, index) => {
                return <ChatElement key={index} message={msg}/>;
            })}
        </div>
    </div>
}

function ChatTextInput(props: { 
    setWhisperRef: (setWhisper: (index: PlayerIndex) => void) => void
}): ReactElement {
    const [chatField, setChatField] = useState<string>("");
    const [players, setPlayers] = useState<Player[]>(GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.players : []);
    const history: HistoryQueue<string> = useMemo(() => new HistoryQueue(40), []);
    const historyPoller: HistoryPoller<string> = useMemo(() => new HistoryPoller(), []);

    const setWhisper = useCallback((index: PlayerIndex) => {
        setChatField("/w" + (index + 1) + " " + chatField)
    }, [chatField, setChatField]);

    props.setWhisperRef(setWhisper);

    useEffect(() => {
        const playersListener: StateListener = (type) => {
            if(GAME_MANAGER.state.stateType === "game" && type === "gamePlayers")
                setPlayers(GAME_MANAGER.state.players);
        };

        GAME_MANAGER.addStateListener(playersListener);
        return () => GAME_MANAGER.removeStateListener(playersListener);
    });

    const sendChatField = useCallback(() => {
        let text = chatField.replace("\n", "").replace("\r", "").trim();
        if (text === "") return;
        history.push(text);
        historyPoller.reset();
        if (text.startsWith("/w")) {
            const recipient = players.find(player => 
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
        setChatField("");
    }, [players, history, historyPoller, chatField]);

    const handleInputChange = useCallback((event: React.ChangeEvent<HTMLTextAreaElement>) => {
        setChatField(
            event.target.value
                .replace(/  +/g, ' ')
                .replace(/\t/g, ' ')
                .replace(/\n/g, ' ')
        );
    }, [setChatField]);

    const handleInputKeyDown = useCallback((event: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (event.code === "Enter") {
            event.preventDefault();
            sendChatField();
        } else if (event.code === "ArrowUp") {
            event.preventDefault();
            const text = historyPoller.poll(history);
            if (text !== undefined) 
                setChatField(text);
        } else if (event.code === "ArrowDown") {
            event.preventDefault();
            const text = historyPoller.pollPrevious(history);
            setChatField(text ?? "");
        }
    }, [sendChatField, history, historyPoller]);

    return <div className="send-section">
        <div>
            <textarea
                value={chatField}
                onChange={handleInputChange}
                onKeyDown={handleInputKeyDown}
            />
            <button 
                className="material-icons-round"
                onClick={sendChatField}
                aria-label={translate("menu.chat.button.send")}
            >
                send
            </button>
        </div>
    </div>
}