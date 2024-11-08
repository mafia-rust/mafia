import React, { ReactElement } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "./playerListMenu.css"
import "./../gameScreen.css"
import { ChatGroup, PhaseState, Player, PlayerIndex } from "../../../game/gameState.d";
import { ContentMenu, ContentTab } from "../GameScreen";
import StyledText from "../../../components/StyledText";
import { RoleState } from "../../../game/roleState.d";
import Icon from "../../../components/Icon";
import { Button } from "../../../components/Button";
import Counter from "../../../components/Counter";
import { useGameState, usePlayerState } from "../../../components/useHooks";
import PlayerNamePlate from "../../../components/PlayerNamePlate";
import ChatMessage, { translateChatMessage } from "../../../components/ChatMessage";

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
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!

    const forfeitVote = usePlayerState(
        gameState => gameState.forfeitVote,
        ["yourForfeitVote"]
    )
    const roleState = usePlayerState(
        gameState => gameState.roleState,
        ["yourRoleState"]
    )
    const myIndex = usePlayerState(
        gameState => gameState.myIndex,
        ["yourPlayerIndex"]
    )
    const chatGroups = usePlayerState(
        gameState => gameState.sendChatGroups,
        ["yourSendChatGroups"]
    )

    const chatFilter = usePlayerState(
        playerState => playerState.chatFilter,
        ["filterUpdate"]
    );

    return <div className="player-list-menu player-list-menu-colors">
        <ContentTab close={ContentMenu.PlayerListMenu} helpMenu={"standard/playerList"}>{translate("menu.playerList.title")}</ContentTab>

        {(myIndex !== undefined && phaseState.type === "discussion" && players[myIndex!].alive) ? <Button
            className={forfeitVote ? "highlighted" : ""}
            onClick={()=>{
                GAME_MANAGER.sendForfeitVotePacket(!forfeitVote);
            }}
        >
            <StyledText noLinks={true}>{translate("forfeitVote")}</StyledText>
        </Button> : null}

        <div className="player-list">
            {players
                .filter(player => player.alive)
                .map(player => <PlayerCard 
                    key={player.name} 
                    player={player}
                    myIndex={myIndex}
                    roleState={roleState}
                    chatFilter={chatFilter}
                    phaseState={phaseState}
                    chatGroups={chatGroups}
                />)}
            {players
                .filter(player => !player.alive).length === 0 || <>
                <div className="dead-players-separator">{translate("dead.icon")} {translate("dead")}</div>
                {players
                    .filter(player => !player.alive)
                    .map(player => <PlayerCard 
                        key={player.name} 
                        player={player}
                        myIndex={myIndex}
                        roleState={roleState}
                        chatFilter={chatFilter}
                        phaseState={phaseState}
                        chatGroups={chatGroups}
                    />)}
            </>}
        </div>
    </div>
}

function PlayerCard(props: Readonly<{ 
    player: Player, 
    myIndex: PlayerIndex | undefined, 
    roleState: RoleState | undefined, 
    chatFilter: number | null | undefined
    phaseState: PhaseState,
    chatGroups: ChatGroup[] | undefined
}>): ReactElement{
    const isPlayerSelf = props.player.index === props.myIndex;

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
                message.variant.messageSender.player === props.player.index
            ),
        ["addChatMessages", "gamePlayers"]
    ) as undefined | NonAnonymousBlockMessage;

    const [alibiOpen, setAlibiOpen] = React.useState(false);

    return <><div 
        className={`player-card`}
        key={props.player.index}
    >
        {GAME_MANAGER.getMySpectator() || (() => {
            const filter = props.player.index;
            const isFilterSet = props.chatFilter === filter;
            
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

        <PlayerNamePlate playerIndex={props.player.index}/>
        {mostRecentBlockMessage !== undefined ? 
            <Button onClick={()=>setAlibiOpen(!alibiOpen)}>
                <StyledText noLinks={true}>{translateChatMessage(mostRecentBlockMessage.variant).split("\n")[1]}</StyledText>
            </Button>
        : null}
        
        
        {GAME_MANAGER.getMySpectator() || (!isPlayerSelf && props.player.alive && (props.chatGroups ?? []).includes("all") && 
            <Button 
                onClick={()=>{GAME_MANAGER.prependWhisper(props.player.index); return true;}}
                pressedChildren={() => <Icon>done</Icon>}
            >
                <Icon>chat</Icon>
            </Button>
        )}
        <VoteButton player={props.player}/>
        {
            props.phaseState.type === "nomination" && props.player.alive && 
            <Counter 
                max={GAME_MANAGER.getVotesRequired()!} 
                current={props.player.numVoted}
            >
                <StyledText>{translate("menu.playerList.player.votes", props.player.numVoted)}</StyledText>
            </Counter>
        }

    </div>
    {alibiOpen && mostRecentBlockMessage !== undefined ? <div onClick={()=>setAlibiOpen(false)}>
        <ChatMessage message={mostRecentBlockMessage}/>
    </div> : null}
    </>
}

function VoteButton(props: Readonly<{
    player: Player
}>): ReactElement | null {
    const playerVotedFor = usePlayerVotedFor();

    if (props.player.buttons.vote) {
        return <Button 
            onClick={()=>GAME_MANAGER.sendVotePacket(props.player.index)}
        >{translate("menu.playerList.button.vote")}</Button>
    } else if (playerVotedFor === props.player.index) {
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