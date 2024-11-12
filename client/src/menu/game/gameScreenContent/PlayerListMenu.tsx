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
import { useGameState, usePlayerState } from "../../../components/useHooks";
import PlayerNamePlate from "../../../components/PlayerNamePlate";
import ChatMessage, { translateChatMessage } from "../../../components/ChatMessage";
import GraveComponent, { translateGraveRole } from "../../../components/grave";

function usePlayerVotedFor(): PlayerIndex | null {
    return usePlayerState(
        (playerState, gameState) => gameState.phaseState.type === "nomination" ? playerState.voted ?? null : null,
        ["phase", "yourVoting"]
    ) ?? null;
}

export default function PlayerListMenu(): ReactElement {
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers", "yourButtons", "playerAlive", "yourPlayerTags", "yourRoleLabels", "playerVotes"]
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
                .map(player => <PlayerCard key={player.index} playerIndex={player.index}/>)}

            {graves.length === 0 || <>
                <div className="dead-players-separator">
                    <StyledText>{translate("grave.icon")} {translate("graveyard")}</StyledText>
                </div>
                {graves.map((grave, index) => <PlayerCard key={grave.player} graveIndex={index} playerIndex={grave.player}/>)}
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
                    .map(player => <PlayerCard key={player.index} playerIndex={player.index}/>)}
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
    const canWhisper = usePlayerState(
        gameState => gameState.sendChatGroups.includes("all"),
        ["yourSendChatGroups"],
        false
    )!;
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!
    const numVoted = useGameState(
        gameState => gameState.players[props.playerIndex].numVoted,
        ["gamePlayers", "playerVotes"]
    )!;

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

    const grave = useGameState(
        gameState => {
            if(props.graveIndex === undefined) return undefined;
            return gameState.graves[props.graveIndex]
        },
        ["addGrave"]
    )!

    return <><div 
        className={`player-card`}
        key={props.playerIndex}
    >
        {GAME_MANAGER.getMySpectator() || (() => {
            const filter = props.playerIndex;
            const isFilterSet = chatFilter === filter;
            
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

        <PlayerNamePlate playerIndex={props.playerIndex}/>
        
        {mostRecentBlockMessage !== undefined ? 
            <Button onClick={()=>setAlibiOpen(!alibiOpen)}>
                <StyledText noLinks={true}>
                    {
                        translateChatMessage(mostRecentBlockMessage.variant)
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
        
        
        {GAME_MANAGER.getMySpectator() || (!isPlayerSelf && playerAlive && canWhisper && 
            <Button 
                onClick={()=>{GAME_MANAGER.prependWhisper(props.playerIndex); return true;}}
                pressedChildren={() => <Icon>done</Icon>}
            >
                <Icon>chat</Icon>
            </Button>
        )}
        <VoteButton playerIndex={props.playerIndex} />
        {
            phaseState.type === "nomination" && playerAlive && 
            <StyledText>{translate("menu.playerList.player.votes", numVoted)}</StyledText>
        }

    </div>
    {alibiOpen && mostRecentBlockMessage !== undefined ? <div onClick={()=>setAlibiOpen(false)}>
        <ChatMessage message={mostRecentBlockMessage}/>
    </div> : null}
    {graveOpen && grave !== undefined ? <div onClick={()=>setGraveOpen(false)}>
        <GraveComponent grave={grave}/>
    </div> : null}
    </>
}

function VoteButton(props: Readonly<{
    playerIndex: PlayerIndex
}>): ReactElement | null {
    const playerVotedFor = usePlayerVotedFor();

    const canVote = usePlayerState(
        (playerState, gameState) => gameState.players[props.playerIndex].buttons.vote,
        ["yourButtons"],
        false
    )!;
        

    if (canVote) {
        return <Button 
            onClick={()=>GAME_MANAGER.sendVotePacket(props.playerIndex)}
        >{translate("menu.playerList.button.vote")}</Button>
    } else if (playerVotedFor === props.playerIndex) {
        return <Button
            highlighted={true}
            onClick={() => GAME_MANAGER.sendVotePacket(null)}
        >{translate("button.clear")}</Button>
    } else {
        return null
    }
}

function findLast<T>(array: T[], predicate: (e: T, i: number, array: T[])=>boolean): T | undefined {
    for(let i = array.length - 1; i >= 0; i--) 
        if(predicate( array[i], i, array )) return array[i];
    return undefined;
};