import React, { useEffect } from "react";
import { ReactElement } from "react";
import { StateEventType, StateListener } from "../../game/gameManager.d";
import GAME_MANAGER from "../..";
import ObituaryScreen from "./ObituaryScreen";
import StyledText from "../../components/StyledText";
import translate from "../../game/lang";
import ChatElement from "../../components/ChatMessage";
import { translateRoleOutline } from "../../game/roleListState.d";
import { PlayerIndex } from "../../game/gameState.d";
import { GraveRole } from "../../game/graveState";

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
                    if(GAME_MANAGER.state.stateType === "game"){
                        let votesRequired = GAME_MANAGER.getVotesRequired();
                        if(votesRequired !== null){
                            subtitleText = votesRequired === 1 ? translate("votesRequired.1") : translate("votesRequired", votesRequired);
                        }
                        subtitleText += " "+translate("trialsRemaining", phase.trialsLeft);
                    }
                    break;
                case "testimony":
                case "judgement":
                case "finalWords":
                    if(GAME_MANAGER.state.stateType === "game" && phase.playerOnTrial !== null){
                        subtitleText = translate("phase."+phase.type+".subtitle", GAME_MANAGER.getPlayerNames()[phase.playerOnTrial+1].toString());
                    }
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

    return <div className="spectator-chat chat-menu-colors">
        <h1><StyledText>{
            props.subtitle??translate("wiki.article.standard.chat.title")
        }</StyledText></h1>
        <ul>
            {
                messages.map((message, index) => {
                    return <ChatElement key={index} message={message}/>
                })
            }
        </ul>
    </div>
}


function Playerlist(): ReactElement {
    const [players, setPlayers] = React.useState(()=>{
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.players : []
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
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setPlayers]);

    const livingPlayers = players.filter(player => player.alive);
    const deadPlayers = players.filter(player => !player.alive);

    return <div className="spectator-player-list player-list-menu-colors">
        <h1><StyledText>{translate("wiki.article.standard.playerList.title")}</StyledText></h1>
        <section>
            <StyledText>{translate("menu.playerList.button.living")}</StyledText>
            <ul>
                {livingPlayers.map(player => {
                    return <div key={player.name}><StyledText>
                        {player.toString()}
                    </StyledText></div>
                })}
            </ul>
        </section>
        <section>
            <StyledText>{translate("dead")}</StyledText>
            <ul>
                {deadPlayers.map(player => {

                    let graveRole = getGraveRole(player.index);
                    let graveRoleString = "";
                    if(graveRole && graveRole.type === "role"){
                        graveRoleString = "("+translate("role."+graveRole.role+".name")+")";
                    }else if(graveRole && graveRole.type === "cleaned"){
                        graveRoleString = "("+translate("cleaned")+")";
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