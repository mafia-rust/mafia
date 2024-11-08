import React, { useMemo } from "react"
import { ReactElement } from "react"
import translate from "../game/lang"
import StyledText from "./StyledText"
import { useGameState, usePlayerState } from "./useHooks"
import "./playerNamePlate.css"

export default function PlayerNamePlate(props: Readonly<{
    playerIndex: number,    //guarantee this index is valid please
}>): ReactElement {

        const phaseState = useGameState(
            (gameState) => gameState.phaseState,
            ["phase"]
        )!;
        const grave = useGameState(
            (gameState) => gameState.graves.find((grave) => grave.player === props.playerIndex),
            ["addGrave"]
        );
        const myRoleState = usePlayerState(
            (playerState) => playerState.roleState,
            ["yourRoleState"]
        );
        const myIndex = usePlayerState(
            (gameState) => gameState.myIndex,
            ["yourPlayerIndex"]
        )!;
        const playerRoleLabel = useGameState(
            (gameState) => gameState.players[props.playerIndex].roleLabel,
            ["gamePlayers", "yourRoleLabels"]
        );
        const playerTags = useGameState(
            (gameState) => gameState.players[props.playerIndex].playerTags,
            ["gamePlayers", "yourPlayerTags"]
        )!;
        const playerAlive = useGameState(
            (gameState) => gameState.players[props.playerIndex].alive,
            ["gamePlayers", "playerAlive"]
        )!;
        const playerName = useGameState(
            (gameState) => gameState.players[props.playerIndex].toString(),
            ["gamePlayers"]
        )!;

        const roleString = useMemo(()=>{
            if(props.playerIndex === myIndex){
                return ("("+translate("role."+myRoleState?.type+".name")+")");
            }
    
            if(playerAlive && playerRoleLabel !== null){
                return ("("+translate("role."+playerRoleLabel+".name")+")");
            }

            if (!playerAlive && grave !== null && grave !== undefined){
                return grave.information.type === "normal"? 
                    "("+translate("role."+grave.information.role+".name")+")" : 
                    "("+translate("obscured")+")";
            }
    
            return "";
        }, [props.playerIndex, myIndex, grave, myRoleState, playerAlive, playerRoleLabel]);

        return <div 
            className="player-name-plate"
            key={props.playerIndex}
        >
            {(() => {
                if (phaseState.type === "testimony" || phaseState.type === "judgement" || phaseState.type === "finalWords") {
                    if (phaseState.playerOnTrial === props.playerIndex) {
                        return <StyledText>{translate("trial.icon")} </StyledText>
                    }
                }
            })()}
            <StyledText>{(playerAlive?"":translate("dead.icon"))} </StyledText>
            <StyledText>{playerName}</StyledText>
            {roleString !== null && <StyledText> {roleString}</StyledText>}
            <StyledText>{playerTags.map((tag)=>{return translate("tag."+tag)})}</StyledText>
        </div>
}