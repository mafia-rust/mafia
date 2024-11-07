import React, { ReactElement, useEffect, useState } from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import "./playerListMenu.css"
import "./../gameScreen.css"
import { ChatGroup, PhaseState, Player, PlayerIndex } from "../../../game/gameState.d";
import { ContentMenu, ContentTab } from "../GameScreen";
import { StateEventType } from "../../../game/gameManager.d";
import StyledText from "../../../components/StyledText";
import { RoleState } from "../../../game/roleState.d";
import Icon from "../../../components/Icon";
import { Button } from "../../../components/Button";
import Counter from "../../../components/Counter";
import { useGameState, usePlayerState } from "../../../components/useHooks";
import PlayerNamePlate from "../../../components/PlayerNamePlate";
type PlayerFilter = "all"|"living"|"usable";


function usePlayerVotedFor(): PlayerIndex | null {
    return usePlayerState(
        (playerState, gameState) => gameState.phaseState.type === "nomination" ? playerState.voted ?? null : null,
        ["phase", "yourVoting"]
    ) ?? null;
}

function useSelectedPlayers(): PlayerIndex[] {
    return usePlayerState(
        (playerState, gameState) => gameState.phaseState.type === "night" ? playerState.targets : [],
        ["phase", "yourSelection"],
        []
    )!;
}

function useDayTargetedPlayers(): PlayerIndex[] {
    const roleState = usePlayerState(
        playerState => playerState.roleState,
        ["yourRoleState"],
    );

    switch (roleState?.type){
        case "godfather":
        case "retrainer":
        case "counterfeiter":
            if (roleState.backup !== null) return [roleState.backup]
            break;
        case "jailor":
        case "kidnapper":
            if (roleState.jailedTargetRef !== null) return [roleState.jailedTargetRef]
            break;
        case "medium":
            if (roleState.seancedTarget !== null) return [roleState.seancedTarget]
            break;
        case "reporter":
            if (roleState.interviewedTarget !== null) return [roleState.interviewedTarget]
            break;
        case "marksman":
            if (roleState.state.type === "marks") return roleState.state.marks
            break
    }

    return []
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

    const [playerFilter, setPlayerFilter] = useState<PlayerFilter>(GAME_MANAGER.getMySpectator() ? "all" : "living");

    useEffect(() => {
        const listener = (type?: StateEventType) => {
            if (type !== "phase" && type !== "gamePlayers" && type !== "yourVoting" && type !== "yourSelection" && type !== "yourRoleState") {
                return;
            }

            if(GAME_MANAGER.getMySpectator()){
                setPlayerFilter("all");
                return;
            }
    
            if(playerFilter !== "all" && players[myIndex!]?.alive){
                if(phaseState.type === "night"){
                    setPlayerFilter("usable");
                }else if(phaseState.type === "obituary"){
                    setPlayerFilter("living");
                }
            }
            //if there are no usable players, switch to living
            if(playerFilter==="usable" && !players.some((player)=>{return Object.values(player.buttons).includes(true)})){
                setPlayerFilter("living");
            }
            //if there are no living players, switch to all
            if(playerFilter==="living" && !players.some((player)=>{return player.alive})){
                setPlayerFilter("all");
            }
        };

        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [myIndex, players, phaseState]);

    const chatFilter = usePlayerState(
        playerState => playerState.chatFilter,
        ["filterUpdate"]
    );

    const chosenPlayers = useSelectedPlayers()
        .concat(useDayTargetedPlayers())
        .concat(usePlayerVotedFor() ?? []);

    const shouldShowPlayer = function (player: Player) {
        switch(playerFilter){
            case "all": return true;
            case "living": return player.alive;
            case "usable": return Object.values(player.buttons).includes(true) || chosenPlayers.includes(player.index)
            default: return false;
        }
    }

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

        {GAME_MANAGER.getMySpectator() || <div>
            {(["all", "living", "usable"] as const)
                .map(filter => 
                    <Button key={filter}
                        highlighted={playerFilter === filter}
                        onClick={()=>setPlayerFilter(filter)}
                    >
                        {translate("menu.playerList.button." + filter)}
                    </Button>
                )}
        </div>}

        <div className="player-list">
            {players
                .filter(shouldShowPlayer)
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
            {players.filter(shouldShowPlayer).filter(player => !player.alive).length === 0 || <>
                <div className="dead-players-separator">{translate("dead.icon")} {translate("dead")}</div>
                {players
                    .filter(shouldShowPlayer)
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

    const chosenPlayers = useSelectedPlayers()
        .concat(useDayTargetedPlayers())
        .concat(usePlayerVotedFor() ?? []);

    return <div 
        className={`player-card ${chosenPlayers.includes(props.player.index) ? "highlighted" : ""}`}
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
        
        
        {GAME_MANAGER.getMySpectator() || (!isPlayerSelf && props.player.alive && (props.chatGroups ?? []).includes("all") && 
            <Button 
                onClick={()=>{GAME_MANAGER.prependWhisper(props.player.index); return true;}}
                pressedChildren={() => <Icon>done</Icon>}
            >
                <Icon>chat</Icon>
            </Button>
        )}
        <VoteButton player={props.player} roleState={props.roleState}/>
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
}

function VoteButton(props: Readonly<{
    player: Player,
    roleState: RoleState | undefined
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