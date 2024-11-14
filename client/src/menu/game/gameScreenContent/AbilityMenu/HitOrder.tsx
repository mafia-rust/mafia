import React from "react";
import { useGameState, usePlayerState } from "../../../../components/useHooks";
import translate from "../../../../game/lang";
import StyledText from "../../../../components/StyledText";
import { Button } from "../../../../components/Button";
import PlayerOptionDropdown from "../../../../components/PlayerOptionDropdown";
import GAME_MANAGER from "../../../..";

export default function HitOrder(props: Readonly<{}>) {
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
        return (<details className="role-specific-colors small-role-specific-menu">
            <summary>{translate("hitOrder")}</summary>
            <div>
                <StyledText>{translate("hitOrder.description")}</StyledText>
                <div>
                    <Button
                        onClick={()=>{
                            const input = {
                                type: "hitOrderMafioso" as const
                            }
                            GAME_MANAGER.sendAbilityInput(input)
                        }}
                    >
                        {translate("switchToMafioso")}
                    </Button>
                    <PlayerOptionDropdown 
                        value={hitOrderVote===undefined?null:hitOrderVote}
                        onChange={(player)=>{
                            const input = {
                                type: "hitOrderVote" as const,
                                selection: player
                            }
                            GAME_MANAGER.sendAbilityInput(input)
                        }}
                        choosablePlayers={
                            players.filter((player)=>player.alive).map((player)=>player.index)
                        }
                        canChooseNone={true}
                    />
                </div>
            </div>
        </details>);
}