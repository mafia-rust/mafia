import React, { ReactElement, useCallback, useEffect, useMemo, useRef, useState } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER, { replaceMentions } from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import { Player, PlayerIndex } from "../../../game/gameState.d";
import { ChatMessage, translateChatMessage } from "../../../components/ChatMessage";
import ChatElement from "../../../components/ChatMessage";
import { ContentMenu, ContentTab } from "../GameScreen";
import { HistoryPoller, HistoryQueue } from "../../../history";
import { StateListener } from "../../../game/gameManager.d";


export default function ChatMenu(): ReactElement {

    const [filter, setFilter] = useState<RegExp | null>(GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.chatFilter : null);
    
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game" && type === "tick") 
                setFilter(GAME_MANAGER.state.chatFilter);
        }
        GAME_MANAGER.addStateListener(stateListener);
        return () => GAME_MANAGER.removeStateListener(stateListener);
    }, [setFilter]);

    return <div className="chat-menu chat-menu-colors">
        <ContentTab close={ContentMenu.ChatMenu} helpMenu={"standard/chat"}>{translate("menu.chat.title")}</ContentTab>
        <ChatMessageSection/>
        {filter && <button 
            onClick={()=>{
                // TODO: Sammy wtf??
                if(GAME_MANAGER.state.stateType === "game"){
                    GAME_MANAGER.state.chatFilter = null;
                    GAME_MANAGER.invokeStateListeners("tick");
                }
            }}
            className="material-icons-round highlighted"
            aria-label={translate("menu.chat.clearFilter")}
        >
            filter_alt_off
        </button>}
        <ChatTextInput/>
    </div>
}


function ChatMessageSection(): ReactElement {
    const [messages, setMessages] = useState<ChatMessage []>(GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.chatMessages : []);
    const [scrolledToBottom, setScrolledToBottom] = useState<boolean>(true);
    
    const self = useRef<HTMLDivElement>(null);

    const AT_BOTTOM_THRESHOLD_PIXELS = 40;
    const handleScroll = (e: any) => {
        const { scrollTop, scrollHeight, clientHeight } = e.target;
        setScrolledToBottom(scrollTop + clientHeight >= scrollHeight - AT_BOTTOM_THRESHOLD_PIXELS);
    }


    const filter = GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.chatFilter : null;
    const filteredMessages = useMemo(() => {
        if (filter === null) return messages;
        else return messages.filter(msg => filter?.test(translateChatMessage(msg, GAME_MANAGER.getPlayerNames())) || msg.type === "phaseChange")
    }, [messages, filter]);

    
    

    // Update with new messages
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game" && type === "addChatMessages") {
                setMessages(GAME_MANAGER.state.chatMessages.filter(msg => 
                    filter === null || msg.type === "phaseChange" || filter.test(translateChatMessage(msg, GAME_MANAGER.getPlayerNames()))
                ))
            }
        }

        GAME_MANAGER.addStateListener(stateListener);
        return () => GAME_MANAGER.removeStateListener(stateListener);
    }, [filter, setMessages]);

    // Keep chat scrolled to bottom
    useEffect(() => {
        if (scrolledToBottom && self.current !== null) {
            const el = self.current;
            el.scrollTop = el.scrollHeight;
        }
    }, [self, messages, filter, scrolledToBottom])

    //scroll chat to bottom when filter is shut off or loaded
    useEffect(() => {
        if (self.current === null) return;
        self.current.scrollTop = self.current.scrollHeight;
    }, [filter])

    

    return <div className="message-section" ref={self} onScroll={handleScroll}>
        <div className="message-list">
            {filteredMessages.map((msg, index) => {
                return <ChatElement key={index} message={msg}/>;
            })}
        </div>
    </div>
}

function ChatTextInput(): ReactElement {
    const [chatBoxText, setChatBoxText] = useState<string>("");
    
    const setWhisper = useCallback((index: PlayerIndex) => {
        setChatBoxText("/w" + (index + 1) + " " + chatBoxText)
    }, [chatBoxText, setChatBoxText]);

    useEffect(() => {
        GAME_MANAGER.setPrependWhisperFunction(setWhisper);
        return () => GAME_MANAGER.setPrependWhisperFunction(() => {});
    }, [setWhisper]);


    const [players, setPlayers] = useState<Player[]>(GAME_MANAGER.state.stateType === "game" ? GAME_MANAGER.state.players : []);
    const history: HistoryQueue<string> = useMemo(() => new HistoryQueue(40), []);
    const historyPoller: HistoryPoller<string> = useMemo(() => new HistoryPoller(), []);


    useEffect(() => {
        const playersListener: StateListener = (type) => {
            if(GAME_MANAGER.state.stateType === "game" && type === "gamePlayers")
                setPlayers(GAME_MANAGER.state.players);
        };

        GAME_MANAGER.addStateListener(playersListener);
        return () => GAME_MANAGER.removeStateListener(playersListener);
    });

    const sendChatField = useCallback(() => {
        let text = chatBoxText.replace("\n", "").replace("\r", "").trim();
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
                    whisperText = replaceMentions(whisperText, GAME_MANAGER.getPlayerNames());
                
                GAME_MANAGER.sendSendWhisperPacket(
                    recipient.index,
                    whisperText
                ); 
            } else {
                // Malformed whisper
                if(GAME_MANAGER.state.stateType === "game")
                    text = replaceMentions(text, GAME_MANAGER.getPlayerNames());
                GAME_MANAGER.sendSendMessagePacket(text);
            }
        } else {
            if(GAME_MANAGER.state.stateType === "game")
                text = replaceMentions(text, GAME_MANAGER.getPlayerNames());
            GAME_MANAGER.sendSendMessagePacket(text);
        }
        setChatBoxText("");
    }, [players, history, historyPoller, chatBoxText]);

    const handleInputChange = useCallback((event: React.ChangeEvent<HTMLTextAreaElement>) => {
        setChatBoxText(
            event.target.value
                .replace(/  +/g, ' ')
                .replace(/\t/g, ' ')
                .replace(/\n/g, ' ')
        );
    }, [setChatBoxText]);

    const handleInputKeyDown = useCallback((event: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (event.code === "Enter") {
            event.preventDefault();
            sendChatField();
        } else if (event.code === "ArrowUp") {
            event.preventDefault();
            const text = historyPoller.poll(history);
            if (text !== undefined) 
                setChatBoxText(text);
        } else if (event.code === "ArrowDown") {
            event.preventDefault();
            const text = historyPoller.pollPrevious(history);
                setChatBoxText(text ?? "");
        }
    }, [sendChatField, history, historyPoller]);

    return <div className="send-section">
        <div>
            <textarea
                value={chatBoxText}
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