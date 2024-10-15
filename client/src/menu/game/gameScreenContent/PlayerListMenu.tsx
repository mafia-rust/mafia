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
import { roleSpecificMenuType } from "../../Settings";
import PlayerDropdown from "../../../components/PlayerDropdown";

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
        case "journalist":
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
    const graveRolesStrings = useGameState(
        gameState => getGraveRolesStrings(gameState.graves, gameState.players.length),
        ["addGrave", "yourButtons", "playerAlive", "yourPlayerTags", "yourRoleLabels", "playerVotes"]
    )!
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!
    const enabledRoles = useGameState(
        gameState => gameState.enabledRoles,
        ["enabledRoles"]
    )!

    const forfeitVote = usePlayerState(
        gameState => gameState.forfeitVote,
        ["yourForfeitVote"]
    )
    const pitchforkVote = usePlayerState(
        gameState => gameState.pitchforkVote,
        ["yourPitchforkVote"]
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

    const [roleSpecificOpen, setRoleSpecificOpen] = useState<boolean>(true);
    const [pitchforkVoteOpen, setPitchforkVoteOpen] = useState<boolean>(false);


    return <div className="player-list-menu player-list-menu-colors">
        <ContentTab close={ContentMenu.PlayerListMenu} helpMenu={"standard/playerList"}>{translate("menu.playerList.title")}</ContentTab>

        {!GAME_MANAGER.getMySpectator() && roleSpecificMenuType(roleState!.type) === "playerList"
        && <details className="role-specific-colors small-role-specific-menu" open={roleSpecificOpen}>
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
        {
            !GAME_MANAGER.getMySpectator() && 
            enabledRoles.includes("rabblerouser") && 
            phaseState.type !== "night" &&
            phaseState.type !== "obituary" &&
            <details className="role-specific-colors small-role-specific-menu" open={pitchforkVoteOpen}>
                <summary
                    onClick={(e)=>{
                        e.preventDefault();
                        setPitchforkVoteOpen(!pitchforkVoteOpen);
                    }}
                >{translate("pitchfork")}</summary>
                <div>
                    <StyledText>{translate("pitchfork.description")}</StyledText>
                    <div>
                    <PlayerDropdown 
                        value={pitchforkVote===undefined?null:pitchforkVote}
                        onChange={(player)=>{GAME_MANAGER.sendPitchforkVotePacket(player)}}
                        choosablePlayers={players.filter((player)=>player.alive).map((player)=>player.index)}
                        canChooseNone={true}
                    /></div>
                </div>
            </details>
        }

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

        if(player.alive && player.roleLabel !== null){
            return ("("+translate("role."+player.roleLabel+".name")+")");
        }
        
        if (!player.alive && graveRolesStrings[player.index] !== null){
            return graveRolesStrings[player.index];
        }

        return "";
    })();

    const chosenPlayers = useSelectedPlayers()
        .concat(useDayTargetedPlayers())
        .concat(usePlayerVotedFor() ?? []);

    return <div 
        className={`player ${chosenPlayers.includes(player.index) ? "highlighted" : ""}`}
        key={player.index}
    >
        {usePlayerVotedFor() === player.index
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
            {roleString !== null && <StyledText> {roleString}</StyledText>}
            <StyledText>{player.playerTags.map((tag)=>{return translate("tag."+tag)})}</StyledText>
        </div>
        
        {phaseState.type === "nomination" && player.alive && 
            <Counter 
                max={GAME_MANAGER.getVotesRequired()!} 
                current={player.numVoted}
            >
                <StyledText>{translate("menu.playerList.player.votes", player.numVoted)}</StyledText>
            </Counter>}

        {GAME_MANAGER.getMySpectator() || <PlayerButtons
            player={player}
            myIndex={myIndex}
            roleState={roleState}
            graveRolesStrings={graveRolesStrings}
            chatFilter={chatFilter}
            phaseState={phaseState}
            chatGroups={chatGroups}
        />}
    </div>
}

function PlayerButtons(props: Readonly<{
    player: Player, 
    myIndex: PlayerIndex | undefined, 
    roleState: RoleState | undefined, 
    chatFilter: number | null | undefined
    graveRolesStrings: (string | null)[],
    phaseState: PhaseState,
    chatGroups: ChatGroup[] | undefined
}>): ReactElement {
    const isPlayerSelf = props.player.index === props.myIndex;

    return <div className="buttons">
        <div className="chat-buttons">
            {(() => {

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
            {!isPlayerSelf && props.player.alive && (props.chatGroups ?? []).includes("all") && <Button 
                onClick={()=>{GAME_MANAGER.prependWhisper(props.player.index); return true;}}
                pressedChildren={() => <Icon>done</Icon>}
            >
                <Icon>chat</Icon>
            </Button>}
        </div>
        <div className="day-target">
            {props.player.buttons.dayTarget && <DayTargetButton player={props.player} roleState={props.roleState}/>}
        </div>
        <div className="target-or-vote">
            <TargetButton player={props.player} roleState={props.roleState}/>
            <VoteButton player={props.player} roleState={props.roleState}/>
        </div>
    </div>
}

function DayTargetButton(props: Readonly<{
    player: Player,
    roleState: RoleState | undefined
}>): ReactElement {
    return <Button 
        highlighted={useDayTargetedPlayers().includes(props.player.index)} 
        onClick={()=>GAME_MANAGER.sendDayTargetPacket(props.player.index)}
    >
        {translateAny(["role."+props.roleState?.type+".dayTarget", "dayTarget"])}
    </Button>
}

function TargetButton(props: Readonly<{
    player: Player,
    roleState: RoleState | undefined
}>): ReactElement | null {
    const targets = usePlayerState(
        playerState => playerState.targets,
        ["yourSelection"]
    )!;

    const selectedPlayers = useSelectedPlayers();

    if(props.player.buttons.target) {
        return <button onClick={() => GAME_MANAGER.sendTargetPacket([...targets, props.player.index])}>
            {translateAny(["role."+props.roleState?.type+".target", "target"])}
        </button>
    } else if (selectedPlayers.includes(props.player.index)) {
        let newTargets = [...targets];
        newTargets.splice(newTargets.indexOf(props.player.index), 1);
        return <Button highlighted={true} onClick={() => GAME_MANAGER.sendTargetPacket(newTargets)}>
            {translate("cancel")}
        </Button>
    } else {
        return null;
    }
}

function VoteButton(props: Readonly<{
    player: Player,
    roleState: RoleState | undefined
}>): ReactElement | null {
    const playerVotedFor = usePlayerVotedFor();

    if (props.player.buttons.vote) {
        return <button 
            onClick={()=>GAME_MANAGER.sendVotePacket(props.player.index)}
        >{translate("menu.playerList.button.vote")}</button>
    } else if (playerVotedFor === props.player.index) {
        return <Button
            highlighted={true}
            onClick={() => GAME_MANAGER.sendVotePacket(null)}
        >{translate("button.clear")}</Button>
    } else {
        return null
    }
}