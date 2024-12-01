import React from "react";
import { useGameState, usePlayerState } from "../../../../components/useHooks";
import GAME_MANAGER from "../../../..";
import translate from "../../../../game/lang";
import StyledText from "../../../../components/StyledText";
import PlayerOptionDropdown from "../../../../components/PlayerOptionDropdown";
import DetailsSummary from "../../../../components/DetailsSummary";

export default function Pitchfork(props: Readonly<{
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
        return <div className="role-specific-colors small-role-specific-menu">
            <DetailsSummary
                summary={translate("pitchfork")}
                defaultOpen={true}
            >
                <div>
                    <StyledText>{translate("pitchfork.description")}</StyledText>
                    <div>
                    <PlayerOptionDropdown 
                        value={pitchforkVote===undefined?null:pitchforkVote}
                        onChange={(player)=>{
                            const input = {
                                type:"pitchforkVote" as const,
                                input: player
                            }
                            GAME_MANAGER.sendAbilityInput(input)}
                        }
                        choosablePlayers={players.filter((player)=>player.alive).map((player)=>player.index)}
                        canChooseNone={true}
                    /></div>
                </div>
            </DetailsSummary>
        </div>
}