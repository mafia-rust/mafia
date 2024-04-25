import React, { useEffect } from "react";
import { ReactElement } from "react";
import { StateEventType, StateListener } from "../../game/gameManager.d";
import GAME_MANAGER from "../..";
import ObituaryScreen from "./ObituaryScreen";
import StyledText from "../../components/StyledText";
import translate from "../../game/lang";
import ChatElement from "../../components/ChatMessage";
import { translateRoleOutline } from "../../game/roleListState.d";
import { PhaseState, Player, PlayerIndex } from "../../game/gameState.d";
import { GraveRole } from "../../game/graveState";
import { getTranslatedSubtitle } from "./SpectatorGameScreen";

export default function SpectatorBody(): ReactElement {

    const [phase, setPhase] = React.useState(()=>{
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.phaseState : {type:"briefing" as "briefing"}
    });
    useEffect(() => {
        const listener: StateListener = (type?: StateEventType) => {
            if(GAME_MANAGER.state.stateType !== "game") return;

            switch (type) {
                case "phase":
                    setPhase(GAME_MANAGER.state.phaseState);
                    break;
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setPhase]);

    switch(phase.type) {
        case "briefing":
        case "night":
        case "discussion":
        case "nomination":
        case "testimony":
        case "judgement":
        case "finalWords":
        case "dusk":

            let subtitleText = undefined;
            switch (phase.type) {
                case "nomination":
                case "testimony":
                case "judgement":
                case "finalWords":
                    subtitleText = getTranslatedSubtitle();
                    break;
            }

            return (
                <div className="spectator-body">
                    <Chat subtitle={subtitleText}/>
                    <RoleList/>
                    <Playerlist/>
                </div>
            );
            
        case "obituary":
            return (
                <ObituaryScreen/>
            );
    }    
}
function RoleList(): ReactElement {

    const [outlines, setOutlines] = React.useState(()=> {
        return GAME_MANAGER.state.stateType==="game"? GAME_MANAGER.state.roleList : []
    });

    useEffect(() => {
        const listener = (type?: StateEventType) => {
            if(GAME_MANAGER.state.stateType !== "game") return;
            switch (type) {
                case "roleList":
                    setOutlines(GAME_MANAGER.state.roleList);
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setOutlines]);


    return <div className="spectator-role-list graveyard-menu-colors">
        <h1><StyledText>{translate("wiki.article.standard.outlineList.title:var.2")}</StyledText></h1>
        <ul>
            {
                outlines.map((role, index) => {
                    return <div key={index}><StyledText>{translateRoleOutline(role)}</StyledText></div>
                })
            }
        </ul>
    </div>
}


function Chat(props: {
    subtitle?: string
}): ReactElement {

    const [messages, setMessages] = React.useState(()=> {
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.chatMessages : []
    });

    useEffect(() => {
        const listener = (type?: StateEventType) => {
            if(GAME_MANAGER.state.stateType !== "game") return;
            switch (type) {
                case "addChatMessages":
                    setMessages(GAME_MANAGER.state.chatMessages);
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setMessages]);

    const reversedMessages = messages.slice().reverse();

    return <div className="spectator-chat chat-menu-colors">
        <h1><StyledText>{
            props.subtitle??translate("wiki.article.standard.chat.title")
        }</StyledText></h1>
        <ul>
            {
                reversedMessages.map((message, index) => {
                    return <ChatElement key={index} message={message}/>
                })
            }
        </ul>
    </div>
}


function SpectatorPlayerList(props: {
    players: Player[],
    phase?: PhaseState
}): ReactElement {
    const livingPlayers = props.players.filter(player => player.alive);
    
    let guiltiedPlayers = props.players.filter(player => player.alive && playerLastVerdict(player)==="guilty");
    let innocentedPlayers = props.players.filter(player => player.alive && playerLastVerdict(player)==="innocent");
    let abstainedPlayers = props.players.filter(player => player.alive && playerLastVerdict(player)==="abstain");
    if(!wasTrialToday()){
        guiltiedPlayers = [];
        innocentedPlayers = [];
        abstainedPlayers = [];
    }


    const otherPlayers = livingPlayers.filter(player => 
        !guiltiedPlayers.includes(player) && !innocentedPlayers.includes(player) && !abstainedPlayers.includes(player)
    );


            
    return <>
        {guiltiedPlayers.length>0 && <section>
            <StyledText>{translate("verdict.guilty")}</StyledText>
            <ul>
                {guiltiedPlayers.map(player => {return <LivingPlayer key={player.name} player={player} phaseState={props.phase}/>})}
            </ul>
        </section>}
        {innocentedPlayers.length>0 && <section>
            <StyledText>{translate("verdict.innocent")}</StyledText>
            <ul>
                {innocentedPlayers.map(player => {return <LivingPlayer key={player.name} player={player} phaseState={props.phase}/>})}
            </ul>
        </section>}
        {abstainedPlayers.length>0&&<section>
            <StyledText>{translate("verdict.abstain")}</StyledText>
            <ul>
                {abstainedPlayers.map(player => {return <LivingPlayer key={player.name} player={player} phaseState={props.phase}/>})}
            </ul>
        </section>}
        <section>
            <StyledText>{translate("menu.playerList.button.living")}</StyledText>
            <ul>
                {otherPlayers.map(player => {return <LivingPlayer key={player.name} player={player} phaseState={props.phase}/>})}
            </ul>
        </section>
    </>
}

function wasTrialToday(): boolean {
    if(GAME_MANAGER.state.stateType !== "game") return false;
    for(let i = GAME_MANAGER.state.chatMessages.length-1; i >= 0; i--){
        let msg = GAME_MANAGER.state.chatMessages[i];
        if(
            msg.variant.type === "phaseChange" &&
            msg.variant.phase.type === "judgement" &&
            msg.variant.dayNumber === GAME_MANAGER.state.dayNumber
        ){
            return true;
        }
    }
    return false;
}
function playerLastVerdict(player: Player): "guilty" | "innocent" | "abstain" | null {
    if (GAME_MANAGER.state.stateType !== "game") {return null;}

    //iterate backwards untill we find the last judgement verdict OR the judegement phase start
    for(let i = GAME_MANAGER.state.chatMessages.length-1; i >= 0; i--){
        let msg = GAME_MANAGER.state.chatMessages[i];

        if(msg.variant.type === "phaseChange" && msg.variant.phase.type === "judgement"){
            return null;
        }

        if(
            msg.variant.type === "judgementVerdict" && 
            msg.variant.voterPlayerIndex === player.index
        ){
            return msg.variant.verdict;
        }
    }

    return null;
}
function votedForThisPlayersTrialToday(player: Player, playerOnTrial: PlayerIndex): boolean {
    if (GAME_MANAGER.state.stateType !== "game") {return false;}

    //iterate backwards untill we find the last judgement verdict OR the judegement phase start
    for(let i = GAME_MANAGER.state.chatMessages.length-1; i >= 0; i--){
        let msg = GAME_MANAGER.state.chatMessages[i];

        if(msg.variant.type === "phaseChange" && msg.variant.phase.type === "nomination"){
            return false;
        }

        if(
            msg.variant.type === "voted" && 
            msg.variant.voter === player.index &&
            msg.variant.votee === playerOnTrial
        ){
            return true;
        }
    }

    return false;
}


function LivingPlayer(props: {
    player: Player,
    phaseState?: PhaseState
}): ReactElement {

    switch (props.phaseState?.type) {
        case "nomination":
            return <div className="player" key={props.player.name}>
                <StyledText>{props.player.toString()}</StyledText>
                <div className="vote-bar"><div style={{
                    width: (100*props.player.numVoted/(GAME_MANAGER.getVotesRequired()??7))+"%",
                    minWidth: "1.05rem",
                    paddingRight: "0.1rem",
                    textAlign: "end",

                    height: "100%",
                    backgroundColor: "orange",
                    marginLeft: "auto",

                    color: "black",
                    fontWeight: "bold",
                }}>{props.player.numVoted}</div></div>
            </div>
        case "testimony":
        case "judgement":
        case "finalWords":
            if(props.phaseState.playerOnTrial === props.player.index){
                return <div className="player" key={props.player.name}><StyledText>
                    {props.player.toString()} ({translate("onTrial")})
                </StyledText></div>
            }else if(votedForThisPlayersTrialToday(props.player, props.phaseState.playerOnTrial)){
                return <div className="player" key={props.player.name}><StyledText>
                    {props.player.toString()} ({translate("voted")})
                </StyledText></div>

            }
            break;
    }

    return <div className="player" key={props.player.name}><StyledText>
        {props.player.toString()}
    </StyledText></div>
}

function Playerlist(): ReactElement {
    const [players, setPlayers] = React.useState(()=>{
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.players : []
    });

    const [phase, setPhase] = React.useState(()=>{
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.phaseState : {type:"briefing" as "briefing"}
    });

    useEffect(() => {
        const listener: StateListener = (type?: StateEventType) => {
            if(GAME_MANAGER.state.stateType !== "game") return;
            switch (type) {
                case "playerAlive":
                case "playerVotes":
                case "playersHost":
                case "gamePlayers":
                    setPlayers(GAME_MANAGER.state.players);
                    break;
                case "phase":
                    setPhase(GAME_MANAGER.state.phaseState);
                    break;
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setPlayers]);

    const deadPlayers = players.filter(player => !player.alive);

    return <div className="spectator-player-list player-list-menu-colors">
        <h1><StyledText>{translate("wiki.article.standard.playerList.title")}</StyledText></h1>
        <SpectatorPlayerList players={players} phase={phase}/>
        <section>
            <StyledText>{translate("dead")}</StyledText>
            <ul>
                {deadPlayers.map(player => {

                    let graveRole = getGraveRole(player.index);
                    let graveRoleString = "";
                    if(graveRole && graveRole.type === "role"){
                        graveRoleString = "("+translate("role."+graveRole.role+".name")+")";
                    }else if(graveRole && graveRole.type === "cremated"){
                        graveRoleString = "("+translate("cremated")+")";
                    }


                    return <div key={player.name}><StyledText>
                        {player.toString()} {graveRoleString}
                    </StyledText></div>
                })}
            </ul>
        </section>
    </div>
}


function getGraveRole(player: PlayerIndex): GraveRole | null{
    if(GAME_MANAGER.state.stateType !== "game") return null;
    for(let grave of GAME_MANAGER.state.graves){
        if(grave.playerIndex === player){
            return grave.role;
        }
    }
    return null;
}