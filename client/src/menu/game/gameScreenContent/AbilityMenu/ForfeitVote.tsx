import { ReactElement } from "react";
import { useGameState, usePlayerState } from "../../../../components/useHooks";
import { Button } from "../../../../components/Button";
import StyledText from "../../../../components/StyledText";
import translate from "../../../../game/lang";
import GAME_MANAGER from "../../../..";
import React from "react";

export default function ForfeitVote(): ReactElement {
    const forfeitVote = usePlayerState(
        playerState => playerState.forfeitVote,
        ["yourForfeitVote"]
    )
    const phaseState = useGameState(
        gameState => gameState.phaseState,
        ["phase"]
    )!
    const myIndex = usePlayerState(
        playerState => playerState.myIndex,
        ["yourPlayerIndex"]
    )
    const imAlive = useGameState(
        gameState => gameState.players[myIndex!].alive,
        ["gamePlayers", "playerAlive"]
    )
    
    return <>{(
        myIndex !== undefined &&
        phaseState.type === "discussion" &&
        imAlive
    ) ? <Button
        className={forfeitVote ? "highlighted" : ""}
        onClick={()=>{
            const input = {
                type: "forfeitVote" as const,
                selection: !forfeitVote
            }
            GAME_MANAGER.sendAbilityInput(input);
        }}
    >
        <StyledText noLinks={true}>{translate("forfeitVote")}</StyledText>
    </Button> : null}</>
}