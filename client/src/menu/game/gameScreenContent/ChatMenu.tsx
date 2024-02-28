import React, { ReactElement, useCallback, useEffect, useMemo, useRef, useState } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER, { replaceMentions } from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import { PlayerIndex } from "../../../game/gameState.d";
import { ChatMessage, translateChatMessage } from "../../../components/ChatMessage";
import ChatElement from "../../../components/ChatMessage";
import { ContentMenu, ContentTab } from "../GameScreen";
import { HistoryPoller, HistoryQueue } from "../../../history";
import { StateListener } from "../../../game/gameManager.d";


export default function ChatMenu(): ReactElement {

    const [filter, setFilter] = useState<PlayerIndex | null>(null);
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game" && type === "filterUpdate") {
                setFilter(GAME_MANAGER.state.chatFilter);
            }
        }
        if (GAME_MANAGER.state.stateType === "game")
            setFilter(GAME_MANAGER.state.chatFilter);
        GAME_MANAGER.addStateListener(stateListener);
        return () => GAME_MANAGER.removeStateListener(stateListener);
    }, [setFilter]);

    const [sendChatGroups, setSendChatGroups] = useState<string[]>([]);
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game" && type === "yourSendChatGroups") {
                setSendChatGroups(GAME_MANAGER.state.sendChatGroups);
            }
        }
        if (GAME_MANAGER.state.stateType === "game")
            setSendChatGroups(GAME_MANAGER.state.sendChatGroups);
        GAME_MANAGER.addStateListener(stateListener);
        return () => GAME_MANAGER.removeStateListener(stateListener);
    }, [setSendChatGroups]);

    return <div className="chat-menu chat-menu-colors">
        <ContentTab close={ContentMenu.ChatMenu} helpMenu={"standard/chat"}>{translate("menu.chat.title")}</ContentTab>
        <ChatMessageSection/>
        {filter !== null && <button 
            onClick={()=>{
                if(GAME_MANAGER.state.stateType === "game"){
                    GAME_MANAGER.state.chatFilter = null;
                    GAME_MANAGER.invokeStateListeners("filterUpdate");
                }
            }}
            className="material-icons-round highlighted"
            aria-label={translate("menu.chat.clearFilter")}
        >
            filter_alt_off
        </button>}
        <div className="chat-menu-icons">
            {!sendChatGroups.includes("all") && translate("noAll.icon")}
            {sendChatGroups.map((group) => {
                return translate("chatGroup."+group+".icon");
            })}
        </div>
        <ChatTextInput disabled={sendChatGroups.length === 0}/>
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
    
    
    const [filter, setFilter] = useState<PlayerIndex | null>(null);
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game" && type === "filterUpdate") {
                setFilter(GAME_MANAGER.state.chatFilter);
            }
        }

        if (GAME_MANAGER.state.stateType === "game") {
            setFilter(GAME_MANAGER.state.chatFilter);
        }

        GAME_MANAGER.addStateListener(stateListener);
        return () => GAME_MANAGER.removeStateListener(stateListener);
    }, [setFilter]);

    // Update with new messages
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game" && type === "addChatMessages") {
                setMessages(GAME_MANAGER.state.chatMessages)
            }
        }

        if (GAME_MANAGER.state.stateType === "game") {
            setMessages(GAME_MANAGER.state.chatMessages)
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
            {messages.filter((msg)=>{
                const msgtxt = translateChatMessage(msg, GAME_MANAGER.getPlayerNames());
                return filter === null || msg.type === "phaseChange" || msgtxt.includes(GAME_MANAGER.getPlayerNames()[filter]);
            }).map((msg, index) => {
                return <ChatElement key={index} message={msg}/>;
            })}
        </div>
    </div>
}

function ChatTextInput(props: {disabled: boolean}): ReactElement {
    const [chatBoxText, setChatBoxText] = useState<string>("");
    
    const setWhisper = useCallback((index: PlayerIndex) => {
        setChatBoxText("/w" + (index + 1) + " " + chatBoxText)
    }, [chatBoxText, setChatBoxText]);

    useEffect(() => {
        GAME_MANAGER.setPrependWhisperFunction(setWhisper);
        return () => GAME_MANAGER.setPrependWhisperFunction(() => {});
    }, [setWhisper]);


    const history: HistoryQueue<string> = useMemo(() => new HistoryQueue(40), []);
    const historyPoller: HistoryPoller<string> = useMemo(() => new HistoryPoller(), []);


    const sendChatField = useCallback(() => {
        let text = chatBoxText.replace("\n", "").replace("\r", "").trim();
        if (text === "") return;
        history.push(text);
        historyPoller.reset();
        if (text.startsWith("/w")) {
            //needs to work with multi digit numbers
            const match = text.match(/\/w(\d+) /);
            if (match === null || match.length < 2) return;
            const index = parseInt(match[1]) - 1;
            GAME_MANAGER.sendSendWhisperPacket(index, text.slice(match[0].length));

        } else {
            if(GAME_MANAGER.state.stateType === "game")
                text = replaceMentions(text, GAME_MANAGER.getPlayerNames());
            GAME_MANAGER.sendSendMessagePacket(text);
        }
        setChatBoxText("");
    }, [history, historyPoller, chatBoxText]);

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
        <textarea
            value={chatBoxText}
            onChange={handleInputChange}
            onKeyDown={handleInputKeyDown}
        />
        <button 
            disabled={props.disabled}
            className="material-icons-round"
            onClick={sendChatField}
            aria-label={translate("menu.chat.button.send")}
        >
            send
        </button>
    </div>
}