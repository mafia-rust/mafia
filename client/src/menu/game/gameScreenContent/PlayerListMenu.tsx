import React, { ReactElement, useEffect, useState } from "react";
import translate, { translateAny } from "../../../game/lang";
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
import { Grave } from "../../../game/graveState";
import RoleSpecificSection from "../../../components/RoleSpecific";
import Counter from "../../../components/Counter";
import SelectionInformation from "../../../components/SelectionInformation";
import { useGameState, usePlayerState } from "../../../components/useHooks";

type PlayerFilter = "all"|"living"|"usable";


//indexed by player index, returns the role on the players grave
function getGraveRolesStrings(graves: Grave[], playerCount: number): (string | null)[] {


    let rolesStrings: (string | null)[] = [];
    
    for(let i = 0; i < playerCount; i++){
        rolesStrings.push(null);
    }

    graves.forEach((grave: Grave)=>{
        if (grave.information.type === "normal"){
            rolesStrings[grave.player] = "("+translate("role."+grave.information.role+".name")+")";
        } else {
            rolesStrings[grave.player] = "("+translate("obscured")+")";
        }
    });

    return rolesStrings;
}

function votedForPlayer(player: Player) {
    return GAME_MANAGER.state.stateType === "game"
        && GAME_MANAGER.state.clientState.type === "player" 
        && GAME_MANAGER.state.phaseState.type === "nomination"
        && GAME_MANAGER.state.clientState.voted === player.index;
}

function selectedPlayer(player: Player) {
    return GAME_MANAGER.state.stateType === "game" 
        && GAME_MANAGER.state.clientState.type === "player" 
        && GAME_MANAGER.state.phaseState.type === "night" 
        && GAME_MANAGER.state.clientState.targets.includes(player.index);
}

function dayTargetedPlayer(player: Player) {
    if (GAME_MANAGER.state.stateType !== "game" || GAME_MANAGER.state.clientState.type !== "player") {
        return false;
    }
    const roleState = GAME_MANAGER.state.clientState.roleState;

    return GAME_MANAGER.state.stateType === "game"
        && GAME_MANAGER.state.clientState.type === "player"
        && (
            (roleState?.type === "godfather" && roleState.backup === player.index)
            ||
            (roleState?.type === "jailor" && roleState.jailedTargetRef === player.index)
            || 
            (roleState?.type === "medium" && roleState.seancedTarget === player.index)
            || 
            (roleState?.type === "journalist" && roleState.interviewedTarget === player.index)
            || 
            (
                roleState?.type === "marksman" && 
                roleState.state.type === "marks" &&
                (
                    (roleState.state.marks.type === "one" && roleState.state.marks.a === player.index) ||
                    (roleState.state.marks.type === "two" && (
                        roleState.state.marks.a === player.index || 
                        roleState.state.marks.b === player.index
                    ))
                )
            )
        )
}

export default function PlayerListMenu(): ReactElement {
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers", "yourButtons", "playerAlive", "yourPlayerTags", "yourRoleLabels", "playerVotes"]
    )!
    const graveRolesStrings = useGameState(
        gameState => getGraveRolesStrings(gameState.graves, gameState.players.length),
        ["addGrave", "yourButtons", "playerAlive", "yourPlayerTags", "yourRoleLabels", "playerVotes"]
    )!
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!

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

    const shouldShowPlayer = function (player: Player) {
        const chosen = selectedPlayer(player) 
            || dayTargetedPlayer(player) 
            || votedForPlayer(player);
        switch(playerFilter){
            case "all": return true;
            case "living": return player.alive;
            case "usable": return Object.values(player.buttons).includes(true) || chosen
            default: return false;
        }
    }

    const [roleSpecificOpen, setRoleSpecificOpen] = useState<boolean>(true);


    return <div className="player-list-menu player-list-menu-colors">
        <ContentTab close={ContentMenu.PlayerListMenu} helpMenu={"standard/playerList"}>{translate("menu.playerList.title")}</ContentTab>

        {GAME_MANAGER.getMySpectator() 
            || <details className="role-specific-colors small-role-specific-menu" open={roleSpecificOpen}>
                <summary
                    onClick={(e)=>{
                        e.preventDefault();
                        setRoleSpecificOpen(!roleSpecificOpen);
                    }}
                >{translate("role."+roleState?.type+".name")}</summary>
                <SelectionInformation />
                <RoleSpecificSection/>
            </details>
        }
        
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
                    graveRolesStrings={graveRolesStrings}
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
                        graveRolesStrings={graveRolesStrings}
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
    graveRolesStrings: (string | null)[],
    phaseState: PhaseState,
    chatGroups: ChatGroup[] | undefined
}>): ReactElement{
    const { player, myIndex, roleState, graveRolesStrings, chatFilter, phaseState, chatGroups } = props;

    let roleString = (()=>{
        if(player.index === myIndex){
            return ("("+translate("role."+roleState?.type+".name")+")");
        }

        if(player.alive && player.roleLabel != null){
            return ("("+translate("role."+player.roleLabel+".name")+")");
        }
        
        if (!player.alive && graveRolesStrings[player.index] != null){
            return graveRolesStrings[player.index];
        }

        return "";
    })();

    const isPlayerSelf = player.index === myIndex;

    return <div 
        className={`player ${(votedForPlayer(player) || selectedPlayer(player) || dayTargetedPlayer(player)) ? "highlighted" : ""}`}
        key={player.index}
    >
        {votedForPlayer(player)
            ? <div className="voted-popup">{translate("menu.playerList.player.youAreVoting")}</div>
            : undefined}
        <div className="top">  
            {(() => {
                if (phaseState.type === "testimony" || phaseState.type === "judgement" || phaseState.type === "finalWords") {
                    if (phaseState.playerOnTrial === player.index) {
                        return <StyledText>{translate("trial.icon")} </StyledText>
                    }
                }
            })()}
            <StyledText>{(player.alive?"":translate("dead.icon"))} </StyledText>
            <StyledText>{player.toString()}</StyledText>
            {roleString!==null&&<StyledText> {roleString}</StyledText>}
            <StyledText>{player.playerTags.map((tag)=>{return translate("tag."+tag)})}</StyledText>
        </div>
        
        {phaseState.type === "nomination" && player.alive && 
            <Counter 
                max={GAME_MANAGER.getVotesRequired()!} 
                current={player.numVoted}
            >
                <StyledText>{translate("menu.playerList.player.votes", player.numVoted)}</StyledText>
            </Counter>}

        {GAME_MANAGER.getMySpectator() || <div className="buttons">
            <div className="chat-buttons">
                {(() => {

                    const filter = player.index;
                    const isFilterSet = chatFilter === filter;
                    
                    return <Button 
                        className={"filter"} 
                        highlighted={isFilterSet}
                        onClick={() => {
                            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player"){
                                GAME_MANAGER.state.clientState.chatFilter = isFilterSet ? null : filter;
                                GAME_MANAGER.invokeStateListeners("filterUpdate");
                            }
                            return true;
                        }}
                        pressedChildren={result => <Icon>{result ? "done" : "warning"}</Icon>}
                        aria-label={translate("menu.playerList.button.filter")}
                    >
                        <Icon>filter_alt</Icon>
                    </Button>
                })()}
                {!isPlayerSelf && player.alive && (chatGroups ?? []).includes("all") && <Button 
                    onClick={()=>{GAME_MANAGER.prependWhisper(player.index); return true;}}
                    pressedChildren={() => <Icon>done</Icon>}
                >
                    <Icon>chat</Icon>
                </Button>}
            </div>
            <div className="day-target">
                {player.buttons.dayTarget && <Button 
                    highlighted={dayTargetedPlayer(player)} 
                    onClick={()=>GAME_MANAGER.sendDayTargetPacket(player.index)}
                >
                    {translateAny(["role."+roleState?.type+".dayTarget", "dayTarget"])}
                </Button>}
            </div>
            <div className="target-or-vote">
                {((player) => {
                    if(player.buttons.target) {
                        return <button onClick={() => {
                            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player")
                                GAME_MANAGER.sendTargetPacket([...GAME_MANAGER.state.clientState.targets, player.index])
                        }}>
                            {translateAny(["role."+roleState?.type+".target", "target"])}
                        </button>
                    } else if (selectedPlayer(player) && GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player") {
                        let newTargets = [...GAME_MANAGER.state.clientState.targets];
                        newTargets.splice(newTargets.indexOf(player.index), 1);
                        return <Button highlighted={true} onClick={() => GAME_MANAGER.sendTargetPacket(newTargets)}>
                            {translate("cancel")}
                        </Button>
                    }
                })(player)}
                {(() => {
                    if (player.buttons.vote) {
                        return <button 
                            onClick={()=>GAME_MANAGER.sendVotePacket(player.index)}
                        >{translate("menu.playerList.button.vote")}</button>
                    } else if (votedForPlayer(player)) {
                        return <Button
                            highlighted={true}
                            onClick={() => GAME_MANAGER.sendVotePacket(null)}
                        >{translate("button.clear")}</Button>
                    }
                })()}
            </div>
        </div>}
    </div>
}
