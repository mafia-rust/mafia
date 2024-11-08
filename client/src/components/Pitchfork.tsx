import React from "react";
import { useGameState, usePlayerState } from "./useHooks";
import GAME_MANAGER from "..";
import translate from "../game/lang";
import StyledText from "./StyledText";
import PlayerOptionDropdown from "./PlayerOptionDropdown";

export default function Pitchfork(props: Readonly<{
    pitchforkVoteOpen: boolean,
    setPitchforkVoteOpen: (open: boolean)=>void,

}>) {
    const enabledRoles = useGameState(
        gameState => gameState.enabledRoles,
        ["enabledRoles"]
    )!;
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!;
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers", "yourButtons", "playerAlive", "yourPlayerTags", "yourRoleLabels", "playerVotes"]
    )!;
    const pitchforkVote = usePlayerState(
        gameState => gameState.pitchforkVote,
        ["yourPitchforkVote"]
    )

    if(
        !enabledRoles.includes("rabblerouser") || 
        phaseState.type === "night" || 
        phaseState.type === "obituary"
    )
        return null;
    else
        return (<details className="role-specific-colors small-role-specific-menu" open={props.pitchforkVoteOpen}>
            <summary
                onClick={(e)=>{
                    e.preventDefault();
                    props.setPitchforkVoteOpen(!props.pitchforkVoteOpen);
                }}
            >{translate("pitchfork")}</summary>
            <div>
                <StyledText>{translate("pitchfork.description")}</StyledText>
                <div>
                <PlayerOptionDropdown 
                    value={pitchforkVote===undefined?null:pitchforkVote}
                    onChange={(player)=>{GAME_MANAGER.sendPitchforkVotePacket(player)}}
                    choosablePlayers={players.filter((player)=>player.alive).map((player)=>player.index)}
                    canChooseNone={true}
                /></div>
            </div>
        </details>);
}