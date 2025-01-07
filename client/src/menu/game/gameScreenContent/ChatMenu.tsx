import React, { ReactElement, useCallback, useEffect, useMemo, useRef, useState } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "../gameScreen.css";
import "./chatMenu.css"
import { PlayerClientType, PlayerIndex } from "../../../game/gameState.d";
import ChatElement, { translateChatMessage } from "../../../components/ChatMessage";
import { ContentMenu, ContentTab } from "../GameScreen";
import { HistoryPoller, HistoryQueue } from "../../../history";
import { Button } from "../../../components/Button";
import Icon from "../../../components/Icon";
import StyledText, { KeywordDataMap, PLAYER_KEYWORD_DATA, PLAYER_SENDER_KEYWORD_DATA } from "../../../components/StyledText";
import { useGameState, useLobbyOrGameState, usePlayerState } from "../../../components/useHooks";
import { Virtuoso } from 'react-virtuoso';
import Select from "../../../components/Select";


export default function ChatMenu(): ReactElement {
    const filter = usePlayerState(
        playerState => playerState.chatFilter,
        ["filterUpdate"]
    );

    const sendChatGroups = usePlayerState(
        playerState => playerState.sendChatGroups,
        ["yourSendChatGroups"]
    );

    const filterString = useMemo(() => {
        if (filter === undefined || filter === null) {
            return "";
        } else if (filter.type === "playerNameInMessage") {
            return GAME_MANAGER.getPlayerNames()[filter.player];
        } else if (filter.type === "myWhispersWithPlayer") {
            return GAME_MANAGER.getPlayerNames()[filter.player];
        }else{
            return "";
        }
    }, [filter]);

    return <div className="chat-menu chat-menu-colors">
        <ContentTab close={ContentMenu.ChatMenu} helpMenu={"standard/chat"}>{translate("menu.chat.title")}</ContentTab>
        {filter === undefined || filter === null || <div className="chat-filter-zone highlighted">
            <StyledText>{translate("menu.chat.playerFilter", filterString)}</StyledText>
            <Button 
                onClick={()=> GAME_MANAGER.updateChatFilter(null)}
                highlighted={true}
                aria-label={translate("menu.chat.clearFilter")}
            >
                <Icon>filter_alt_off</Icon>
            </Button>
        </div>}
        <ChatMessageSection filter={filter}/>
        {sendChatGroups === undefined || <>
            <div className="chat-menu-icons">
                {!sendChatGroups.includes("all") && translate("noAll.icon")}
                {sendChatGroups.map((group) => {
                    return translate("chatGroup."+group+".icon");
                })}
            </div>
            <ChatTextInput disabled={sendChatGroups.length === 0}/>
        </>}
    </div>
}

export type ChatFilter = {
    type: "playerNameInMessage",
    player: PlayerIndex
} | {
    type: "myWhispersWithPlayer",
    player: PlayerIndex,
} | null;

export function ChatMessageSection(props: Readonly<{
    filter?: ChatFilter,
}>): ReactElement {
    const players = useGameState((gameState)=>{return gameState.players}, ["gamePlayers"])!;
    const filter = useMemo(() => props.filter ?? null, [props.filter]);
    const messages = useLobbyOrGameState(
        state => state.chatMessages,
        ["addChatMessages"]
    )!;
    const myPlayerIndex = usePlayerState(
        (gameState)=>gameState.myIndex,
        ["yourPlayerIndex"]
    );

    const allMessages = messages
        .filter((msg)=>{
            if(filter === null || filter === undefined)
                return true;

            switch(filter.type){
                case "playerNameInMessage":
                    let msgTxt = "";
                    //special case messages, where translate chat message doesnt work properly, or it should be let through anyway
                    switch (msg.variant.type) {
                        //translateChatMessage errors for playerDied type.
                        case "playerDied":
                        case "phaseChange":
                            return true
                        case "normal":
                            switch(msg.variant.messageSender.type) {
                                case "player":
                                case "livingToDead":
                                    if(msg.variant.messageSender.player === filter.player)
                                        return true;
                                    break;
                            }
                            break;
                        case "targetsMessage":
                            msgTxt = translateChatMessage(msg.variant.message, GAME_MANAGER.getPlayerNames());
                            break;
                    }

                    msgTxt += translateChatMessage(msg.variant, GAME_MANAGER.getPlayerNames());
                    
                    return msgTxt.includes(GAME_MANAGER.getPlayerNames()[filter.player]);
                case "myWhispersWithPlayer":
                    switch (msg.variant.type) {
                        //translateChatMessage errors for playerDied type.
                        case "phaseChange":
                            return true
                        case "whisper":
                            if(
                                (msg.variant.fromPlayerIndex === filter.player && msg.variant.toPlayerIndex === myPlayerIndex) ||
                                (msg.variant.toPlayerIndex === filter.player && msg.variant.fromPlayerIndex === myPlayerIndex)
                            )
                                return true;
                            else
                                return false;
                        default:
                            return false;
                    }
            }
            return true;
        }).filter((msg, index, array)=>{
            //if there is a filter, remove repeat phaseChange message
            if(filter === null){return true}
            if(msg.variant.type !== "phaseChange"){return true}
            if(index+1===array.length){return true}
            if(array[index+1].variant.type !== "phaseChange"){return true}
            return false;
        }).map((msg, index) => {
            return <ChatElement
                key={index}
                message={msg}
                playerKeywordData={(() => {
                    if (filter===null) {return undefined}
                    if (filter.type === "myWhispersWithPlayer") {return undefined}

                    const newKeywordData: KeywordDataMap = {...PLAYER_KEYWORD_DATA};

                    newKeywordData[players[filter.player].toString()] = [
                        { style: "keyword-player-important keyword-player-number", replacement: (filter.player + 1).toString() },
                        { replacement: " " },
                        { style: "keyword-player-important keyword-player-sender", replacement: players[filter.player].name }
                    ];
                    
                    return newKeywordData;
                })()}
                playerSenderKeywordData={(() => {
                    if (filter===null) {return undefined}
                    if (filter.type === "myWhispersWithPlayer") {return undefined}

                    const newKeywordData: KeywordDataMap = {...PLAYER_SENDER_KEYWORD_DATA};

                    newKeywordData[players[filter.player].toString()] = [
                        { style: "keyword-player-important keyword-player-number", replacement: (filter.player + 1).toString() },
                        { replacement: " " },
                        { style: "keyword-player-important keyword-player-sender", replacement: players[filter.player].name }
                    ];
                    
                    return newKeywordData;
                })()}
            />;
        })

    return <div className="chat-message-section"><Virtuoso
        alignToBottom={true}
        totalCount={allMessages.length}
        followOutput={true}
        initialTopMostItemIndex={allMessages.length===0 ? 0 : allMessages.length-1}
        itemContent={(index) => allMessages[index]}
        atBottomThreshold={15}
    /></div>;
}

export function ChatTextInput(props: Readonly<{
    disabled?: boolean,
    whispering?: PlayerIndex | null,
}>): ReactElement {
    const [chatBoxText, setChatBoxText] = useState<string>("");
    const [drawAttentionSeconds, setDrawAttentionSeconds] = useState<number>(0);
    const ref = useRef<HTMLTextAreaElement>(null);
    const [whisperingState, setWhispering] = useState<PlayerIndex | null>(null);

    const whispering = useMemo(() => {
        if (props.whispering === undefined) {
            return whisperingState;
        } else {
            return props.whispering;
        }
    }, [props.whispering, whisperingState]);

    const gamePlayers = useGameState(
        gameState => gameState.players,
        ["gamePlayers"]
    );
    const myIndex = usePlayerState(
        playerState => playerState.myIndex,
        ["yourPlayerIndex"]
    );
    const stateType = useLobbyOrGameState(
        state => state.stateType,
        ["acceptJoin", "gameInitializationComplete", "startGame", "backToLobby"]
    )!;
    const playerStrings = useLobbyOrGameState(
        state => {
            if (state.stateType === "game") {
                return state.players.map(player => player.toString())
            } else if (state.stateType === "lobby") {
                return Array.from(state.players.values())
                    .filter(player => player.clientType.type === "player")
                    .map(player => (player.clientType as PlayerClientType).name)
            }
        }
    )!;

    const whisperingPlayer = useMemo(() => {
        return whispering!==null ? playerStrings[whispering] : null
    }, [playerStrings, whispering])
    
    const prependWhisper = useCallback((index: PlayerIndex) => {
        if (gamePlayers !== undefined && index < gamePlayers.length && index !== myIndex) {
            setWhispering(index);
            setDrawAttentionSeconds(1.5);
            ref.current?.focus()
        }
    }, [gamePlayers, myIndex]);

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
        let text = chatBoxText.trim();
        setWhispering(null);
        setChatBoxText("");
        if (text === "") return;
        history.push(text);
        historyPoller.reset();
        if (stateType === "game") {
            if (whispering !== null) {
                GAME_MANAGER.sendSendWhisperPacket(whispering, text);
            } else {
                GAME_MANAGER.sendSendChatMessagePacket(text, false);
            }
        } else if (stateType === "lobby") {
            GAME_MANAGER.sendSendLobbyMessagePacket(text);
        }
    }, [chatBoxText, history, historyPoller, stateType, whispering]);

    const handleInputChange = useCallback((event: React.ChangeEvent<HTMLTextAreaElement>) => {
        const text = event.target.value;
        const whisperCommandMatch = RegExp(/\/w(\d+) /).exec(text);
        if (whispering === null && whisperCommandMatch !== null) {
            const index = parseInt(whisperCommandMatch[1]) - 1;
            if (gamePlayers !== undefined && index < gamePlayers.length && index >= 0 && index !== myIndex) {
                setWhispering(index);
                setChatBoxText(text.slice(whisperCommandMatch[0].length));
            } else {
                setWhispering(null);
                setChatBoxText(text);
            }
        } else {
            setChatBoxText(text);
        }
    }, [gamePlayers, myIndex, whispering]);

    const handleInputKeyDown = useCallback((event: React.KeyboardEvent<HTMLTextAreaElement>) => {
        
        //if press enter while holding shift
        if(event.key === "Enter" && event.shiftKey){
            event.preventDefault();
            setChatBoxText(chatBoxText+"\n");
        } else if (event.key === "Enter") {
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
    }, [sendChatField, history, historyPoller, chatBoxText]);

    return <>
        {whisperingPlayer !== null && <div className="chat-whisper-notification">
            <StyledText className="discreet">{translate("youAreWhispering", whisperingPlayer)}</StyledText>
            {props.whispering === undefined ? <Button
                highlighted={true}
                onClick={() => setWhispering(null)}
            >
                {translate("cancelWhisper")}
            </Button>:null}
        </div>}
        <div className="chat-send-section">
            <textarea
                className={drawAttentionSeconds * 2 % 2 === 1 ? "highlighted" : undefined}
                ref={ref}
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
