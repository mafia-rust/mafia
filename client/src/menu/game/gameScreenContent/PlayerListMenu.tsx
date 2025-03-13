import React, { ReactElement } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "./playerListMenu.css"
import "./../gameScreen.css"
import { PlayerIndex } from "../../../game/gameState.d";
import { ContentMenu, ContentTab } from "../GameScreen";
import StyledText from "../../../components/StyledText";
import Icon from "../../../components/Icon";
import { Button } from "../../../components/Button";
import { useGameState, usePlayerNames, usePlayerState, useSpectator } from "../../../components/useHooks";
import PlayerNamePlate from "../../../components/PlayerNamePlate";
import ChatMessage, { translateChatMessage } from "../../../components/ChatMessage";
import GraveComponent, { translateGraveRole } from "../../../components/grave";
import { ChatMessageSection, ChatTextInput } from "./ChatMenu";

export default function PlayerListMenu(): ReactElement {
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers", "playerAlive", "yourPlayerTags", "yourRoleLabels", "playerVotes"]
    )!

    const graves = useGameState(
        gameState => gameState.graves,
        ["addGrave"]
    )!


    return <div className="player-list-menu player-list-menu-colors">
        <ContentTab close={ContentMenu.PlayerListMenu} helpMenu={"standard/playerList"}>{translate("menu.playerList.title")}</ContentTab>

        <div className="player-list">
            {players
                .filter(player => player.alive)
                .map(player => <div key={player.index} className="player-card-holder"><PlayerCard playerIndex={player.index}/></div>)}

            {graves.length === 0 || <>
                <div className="dead-players-separator">
                    <StyledText>{translate("grave.icon")} {translate("graveyard")}</StyledText>
                </div>
                {graves.map((grave, index) => <div key={grave.player} className="player-card-holder"><PlayerCard graveIndex={index} playerIndex={grave.player}/></div>)}
            </>}
            {players
                .filter(
                    player => !player.alive && 
                    graves.find(grave => grave.player === player.index) === undefined
                ).length === 0 || <>
                <div className="dead-players-separator">
                    <StyledText>{translate("grave.icon")} {translate("graveyard")}</StyledText>
                </div>
                {players
                    .filter(player => !player.alive)
                    .map(player => <div key={player.index} className="player-card-holder"><PlayerCard playerIndex={player.index}/></div>)}
            </>}
        </div>
    </div>
}

function PlayerCard(props: Readonly<{
    graveIndex?: number,
    playerIndex: number
}>): ReactElement{
    const isPlayerSelf = usePlayerState(
        playerState => playerState.myIndex === props.playerIndex,
        ["yourPlayerIndex"],
        false
    )!;
    const chatFilter = usePlayerState(
        playerState => playerState.chatFilter,
        ["filterUpdate"],
    );
    const playerAlive = useGameState(
        gameState => gameState.players[props.playerIndex].alive,
        ["gamePlayers", "playerAlive"]
    )!;
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!
    const numVoted = useGameState(
        gameState => gameState.players[props.playerIndex].numVoted,
        ["gamePlayers", "playerVotes"]
    )!;
    const sendChatGroups = usePlayerState(
        playerState => playerState.sendChatGroups,
        ["yourSendChatGroups"]
    );
    const playerNames = usePlayerNames();

    type NonAnonymousBlockMessage = {
        variant: {
            type: "normal", 
            messageSender: {
                type: "player", 
                player: PlayerIndex
            } | {
                type: "livingToDead",
                player: PlayerIndex,
            },
            text: string,
            block: true
        }
        chatGroup: "all"
    }

    const mostRecentBlockMessage: undefined | NonAnonymousBlockMessage = useGameState(
        gameState => findLast(gameState.chatMessages, message =>
                message.chatGroup === "all" && 
                message.variant.type === "normal" &&
                message.variant.block &&
                (message.variant.messageSender.type === "player" || message.variant.messageSender.type === "livingToDead") &&
                message.variant.messageSender.player === props.playerIndex
            ),
        ["addChatMessages", "gamePlayers"]
    ) as undefined | NonAnonymousBlockMessage;

    const [alibiOpen, setAlibiOpen] = React.useState(false);
    const [graveOpen, setGraveOpen] = React.useState(false);
    const [whisperChatOpen, setWhisperChatOpen] = React.useState(false);
    const whispersDisabled = useGameState(
        gameState => gameState.enabledModifiers.includes("noWhispers"),
        ["enabledModifiers"]
    )!;

    const grave = useGameState(
        gameState => {
            if(props.graveIndex === undefined) return undefined;
            return gameState.graves[props.graveIndex]
        },
        ["addGrave"]
    )!

    const whisperNotification = usePlayerState(
        gameState =>
            gameState.missedWhispers.some(player => player === props.playerIndex) &&
            !isPlayerSelf &&
            !whisperChatOpen,
        ["addChatMessages", "whisperChatOpenOrClose"],
        false
    );

    const spectator = useSpectator();

    return <><div 
        className={`player-card`}
        key={props.playerIndex}
    >
        <PlayerNamePlate playerIndex={props.playerIndex}/>
        
        {mostRecentBlockMessage !== undefined ? 
            <Button onClick={()=>setAlibiOpen(!alibiOpen)}>
                <StyledText noLinks={true}>
                    {
                        translateChatMessage(mostRecentBlockMessage.variant, playerNames)
                            .split("\n")[1]
                            .trim()
                            .substring(0,30)
                            .trim()
                    }
                </StyledText>
            </Button>
        : null}
        {grave !== undefined ? 
            <Button onClick={()=>setGraveOpen(!graveOpen)}>
                <StyledText noLinks={true}>
                    {translateGraveRole(grave)} {translate(grave.diedPhase+".icon")}{grave.dayNumber.toString()}
                </StyledText>
            </Button>
        : null}
        
        {
            phaseState.type === "nomination" && playerAlive && 
            <StyledText>{translate("menu.playerList.player.votes", numVoted)}</StyledText>
        }
        {spectator ||
            <Button 
                disabled={isPlayerSelf || whispersDisabled}
                onClick={()=>{
                    // GAME_MANAGER.prependWhisper(props.playerIndex); return true;
                    setWhisperChatOpen(!whisperChatOpen);
                    if(GAME_MANAGER.state.stateType === 'game' && GAME_MANAGER.state.clientState.type === 'player'){
                        GAME_MANAGER.state.clientState.missedWhispers = 
                            GAME_MANAGER.state.clientState.missedWhispers.filter(player => player !== props.playerIndex);
                    }
                    GAME_MANAGER.invokeStateListeners("whisperChatOpenOrClose");
                }}
                pressedChildren={() => <Icon>done</Icon>}
            >
                {whisperChatOpen===true?<Icon>close</Icon>:<Icon>chat</Icon>}
                {whisperNotification===true && <div className="chat-notification highlighted">!</div>}
            </Button>
        }
        {spectator || (() => {
            const filter = props.playerIndex;
            const isFilterSet = chatFilter?.type === "playerNameInMessage" && (chatFilter.player === filter);
            
            return <Button 
                className={"filter"} 
                highlighted={isFilterSet}
                onClick={() => {
                    GAME_MANAGER.updateChatFilter(isFilterSet ? null : filter);
                    return true;
                }}
                pressedChildren={result => <Icon>{result ? "done" : "warning"}</Icon>}
                aria-label={translate("menu.playerList.button.filter")}
            >
                <Icon>filter_alt</Icon>
            </Button>
        })()}
    </div>
    {alibiOpen && mostRecentBlockMessage !== undefined ? <div onClick={()=>setAlibiOpen(false)}>
        <ChatMessage message={mostRecentBlockMessage}/>
    </div> : null}
    {graveOpen && grave !== undefined ? <div onClick={()=>setGraveOpen(false)}>
        <GraveComponent grave={grave}/>
    </div> : null}
    {(whisperChatOpen && !isPlayerSelf) && <div className="chat-menu-colors player-list-chat-section">
        <div className="player-list-chat-message-section">
            <ChatMessageSection filter={{
                type: "myWhispersWithPlayer",
                player: props.playerIndex
            }}/>
        </div>
        {sendChatGroups === undefined || <ChatTextInput
            disabled={sendChatGroups.length === 0}
            whispering={props.playerIndex}
        />}
    </div>}
    </>
}

function findLast<T>(array: T[], predicate: (e: T, i: number, array: T[])=>boolean): T | undefined {
    for(let i = array.length - 1; i >= 0; i--) 
        if(predicate( array[i], i, array )) return array[i];
    return undefined;
};