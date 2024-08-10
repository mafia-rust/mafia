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
import { Button } from "../../../components/Button";
import Icon from "../../../components/Icon";
import StyledText from "../../../components/StyledText";


export default function ChatMenu(): ReactElement {

    const [filter, setFilter] = useState<PlayerIndex | null>(null);
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game" && type === "filterUpdate" && GAME_MANAGER.state.clientState.type === "player") {
                setFilter(GAME_MANAGER.state.clientState.chatFilter);
            }
        }
        if (GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
            setFilter(GAME_MANAGER.state.clientState.chatFilter);
        GAME_MANAGER.addStateListener(stateListener);
        return () => GAME_MANAGER.removeStateListener(stateListener);
    }, [setFilter]);
    

    const [sendChatGroups, setSendChatGroups] = useState<string[]>([]);
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player" && type === "yourSendChatGroups") {
                setSendChatGroups(GAME_MANAGER.state.clientState.sendChatGroups);
            }
        }
        if (GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
            setSendChatGroups(GAME_MANAGER.state.clientState.sendChatGroups);
        GAME_MANAGER.addStateListener(stateListener);
        return () => GAME_MANAGER.removeStateListener(stateListener);
    }, [setSendChatGroups]);

    return <div className="chat-menu chat-menu-colors">
        <ContentTab close={ContentMenu.ChatMenu} helpMenu={"standard/chat"}>{translate("menu.chat.title")}</ContentTab>
        <ChatMessageSection filter={filter}/>
        {filter !== null && <Button 
            onClick={()=>{
                if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                    GAME_MANAGER.state.clientState.chatFilter = null;
                    GAME_MANAGER.invokeStateListeners("filterUpdate");
                }
            }}
            highlighted={true}
            aria-label={translate("menu.chat.clearFilter")}
        >
            <Icon>filter_alt_off</Icon>
        </Button>}
        <div className="chat-menu-icons">
            {!sendChatGroups.includes("all") && translate("noAll.icon")}
            {sendChatGroups.map((group) => {
                return translate("chatGroup."+group+".icon");
            })}
        </div>
        <ChatTextInput disabled={sendChatGroups.length === 0}/>
    </div>
}


export function ChatMessageSection(props:{
    filter?: PlayerIndex | null
}): ReactElement {
    const filter = useMemo(() => props.filter ?? null, [props.filter]);
    const [messages, setMessages] = useState<ChatMessage []>(() => {
        if (GAME_MANAGER.state.stateType === "game" || GAME_MANAGER.state.stateType === "lobby")
            return GAME_MANAGER.state.chatMessages;
        else
            return [];
    });
    const [scrolledToBottom, setScrolledToBottom] = useState<boolean>(true);
    
    const self = useRef<HTMLDivElement>(null);

    const AT_BOTTOM_THRESHOLD_PIXELS = 40;
    const handleScroll = (e: any) => {
        const { scrollTop, scrollHeight, clientHeight } = e.target;
        setScrolledToBottom(scrollTop + clientHeight >= scrollHeight - AT_BOTTOM_THRESHOLD_PIXELS);
    }

    // Update with new messages
    useEffect(() => {
        const stateListener: StateListener = (type) => {
            if (
                (GAME_MANAGER.state.stateType === "game" || GAME_MANAGER.state.stateType === "lobby") 
                && type === "addChatMessages"
            ) {
                setMessages(GAME_MANAGER.state.chatMessages)
            }
        }

        if (GAME_MANAGER.state.stateType === "game" || GAME_MANAGER.state.stateType === "lobby") {
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

    

    return <div className="chat-message-section" ref={self} onScroll={handleScroll}>
        <div className="chat-message-list">
            {messages.filter((msg)=>{

                if(filter === null)
                    return true;
                
                let msgTxt = "";
                //special case messages, where translate chat message doesnt work properly, or it should be let through anyway
                switch (msg.variant.type) {
                    //translateChatMessage errors for playerDied type.
                    case "playerDied":
                    case "phaseChange":
                        return true;
                    case "normal":
                        switch(msg.variant.messageSender.type) {
                            case "player":
                            case "livingToDead":
                                if(msg.variant.messageSender.player === filter)
                                    return true;
                                break;
                        }
                        break;
                    case "targetsMessage":
                        msgTxt = translateChatMessage(msg.variant.message, GAME_MANAGER.getPlayerNames());
                        break;
                }

                msgTxt += translateChatMessage(msg.variant, GAME_MANAGER.getPlayerNames());
                
                return msgTxt.includes(GAME_MANAGER.getPlayerNames()[filter]);
            }).map((msg, index) => {
                return <ChatElement key={index} message={msg}/>;
            })}
        </div>
    </div>
}

export function ChatTextInput(props: {disabled?: boolean}): ReactElement {
    const [chatBoxText, setChatBoxText] = useState<string>("");
    const [drawAttentionSeconds, setDrawAttentionSeconds] = useState<number>(0);
    const [whispering, setWhispering] = useState<PlayerIndex | null>(null);

    const whisperingPlayer = useMemo(() => {
        if (GAME_MANAGER.state.stateType === "game" && whispering !== null) {
            return GAME_MANAGER.state.players[whispering].toString();
        } else {
            return `${whispering}`
        }
    }, [whispering])
    
    const prependWhisper = useCallback((index: PlayerIndex) => {
        if (
            GAME_MANAGER.state.stateType === "game" 
            && index < GAME_MANAGER.state.players.length 
            && GAME_MANAGER.state.clientState.type === "player" 
            && index !== GAME_MANAGER.state.clientState.myIndex
        ) {
            setWhispering(index);
            setDrawAttentionSeconds(1.5);
        }
    }, []);

    useEffect(() => {
        if (drawAttentionSeconds === 0) {
            return;
        } else if (drawAttentionSeconds < 0) {
            setDrawAttentionSeconds(0);
        } else {
            setTimeout(() => {
                setDrawAttentionSeconds(drawAttentionSeconds - 0.5);
            }, 500)
        }
    }, [drawAttentionSeconds])

    useEffect(() => {
        GAME_MANAGER.setPrependWhisperFunction(prependWhisper);
        return () => GAME_MANAGER.setPrependWhisperFunction(() => {});
    }, [prependWhisper]);


    const history: HistoryQueue<string> = useMemo(() => new HistoryQueue(40), []);
    const historyPoller: HistoryPoller<string> = useMemo(() => new HistoryPoller(), []);


    const sendChatField = useCallback(() => {
        let text = chatBoxText.replace("\n", "").replace("\r", "").trim();
        setWhispering(null);
        setChatBoxText("");
        if (text === "") return;
        history.push(text);
        historyPoller.reset();
        if (GAME_MANAGER.state.stateType === "game") {
            if (whispering !== null) {
                GAME_MANAGER.sendSendWhisperPacket(whispering, text);
            } else {
                GAME_MANAGER.sendSendMessagePacket(text);
            }
        } else if (GAME_MANAGER.state.stateType === "lobby") {
            GAME_MANAGER.sendSendLobbyMessagePacket(text);
        }
    }, [chatBoxText, history, historyPoller, whispering]);

    const handleInputChange = useCallback((event: React.ChangeEvent<HTMLTextAreaElement>) => {
        const text = event.target.value;
        const whisperCommandMatch = RegExp(/\/w(\d+) /).exec(text);
        if (whispering === null && whisperCommandMatch !== null) {
            const index = parseInt(whisperCommandMatch[1]) - 1;
            if (
                GAME_MANAGER.state.stateType === "game" 
                && index < GAME_MANAGER.state.players.length 
                && index >= 0
                && GAME_MANAGER.state.clientState.type === "player" 
                && index !== GAME_MANAGER.state.clientState.myIndex
            ) {
                setWhispering(index);
                setChatBoxText(text.slice(whisperCommandMatch[0].length));
            } else {
                setWhispering(null);
                setChatBoxText(text);
            }
        } else {
            setChatBoxText(
                text
                    .replace(/  +/g, ' ')
                    .replace(/\t/g, ' ')
                    .replace(/\n/g, ' ')
            );
        }
    }, [whispering]);

    const handleInputKeyDown = useCallback((event: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (event.key === "Enter") {
            event.preventDefault();
            sendChatField();
        } else if (event.key === "ArrowUp") {
            event.preventDefault();
            const text = historyPoller.poll(history);
            if (text !== undefined) 
                setChatBoxText(text);
        } else if (event.key === "ArrowDown") {
            event.preventDefault();
            const text = historyPoller.pollPrevious(history);
            setChatBoxText(text ?? "");
        } else if (event.key === "Escape") {
            event.preventDefault();
            setWhispering(null);
        }
    }, [sendChatField, history, historyPoller]);

    return <>
        {whispering !== null && <div className="chat-whisper-notification">
            <StyledText>{translate("youAreWhispering", whisperingPlayer)}</StyledText>
            <Button
                highlighted={true}
                onClick={() => setWhispering(null)}
            >
                {translate("cancelWhisper")}
            </Button>
        </div>}
        <div className="chat-send-section">
            <textarea
                className={drawAttentionSeconds * 2 % 2 === 1 ? "highlighted" : undefined}
                value={chatBoxText}
                onChange={handleInputChange}
                onKeyDown={handleInputKeyDown}
            />
            <Button 
                disabled={props.disabled}
                onClick={sendChatField}
                aria-label={translate("menu.chat.button.send")}
            >
                <Icon>send</Icon>
            </Button>
        </div>
    </>
}
