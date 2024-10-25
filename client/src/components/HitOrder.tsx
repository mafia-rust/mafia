import React from "react";
import { useGameState, usePlayerState } from "./useHooks";
import GAME_MANAGER from "..";
import translate from "../game/lang";
import StyledText from "./StyledText";
import PlayerDropdown from "./PlayerDropdown";

export default function HitOrder(props: Readonly<{
    hitOrderOpen: boolean,
    setHitOrderOpen: (open: boolean)=>void,
}>) {
    const enabledModifiers = useGameState(
        gameState => gameState.enabledModifiers,
        ["enabledModifiers"]
    )!;
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase", "playerOnTrial"]
    )!;
    const players = useGameState(
        gameState => gameState.players,
        ["gamePlayers", "yourButtons", "playerAlive", "yourPlayerTags", "yourRoleLabels", "playerVotes"]
    )!;
    const hitOrderVote = usePlayerState(
        playerState => playerState.hitOrderVote,
        ["yourHitOrderVote"]
    )!;
    const dayNumber = useGameState(
        gameState => gameState.dayNumber,
        ["phase"]
    )!;
    
    const insiderGroups = usePlayerState(
        playerState => playerState.insiderGroups,
        ["yourInsiderGroups"]
    )!;

    if(
        !enabledModifiers.includes("mafiaHitOrders") ||
        phaseState.type !== "night" ||
        dayNumber === 1 ||
        !insiderGroups.includes("mafia")
    )
        return null;
    else
        return (<details className="role-specific-colors small-role-specific-menu" open={props.hitOrderOpen}>
            <summary
                onClick={(e)=>{
                    e.preventDefault();
                    props.setHitOrderOpen(!props.hitOrderOpen);
                }}
            >{translate("hitOrder")}</summary>
            <div>
                <StyledText>{translate("hitOrder.description")}</StyledText>
                <div>
                <PlayerDropdown 
                    value={hitOrderVote===undefined?null:hitOrderVote}
                    onChange={(player)=>{
                        GAME_MANAGER.sendHitOrderVotePacket(player)
                    }}
                    choosablePlayers={
                        players.filter((player)=>player.alive).map((player)=>player.index)
                    }
                    canChooseNone={true}
                /></div>
            </div>
        </details>);
}