import React, { ReactElement } from "react";
import GAME_MANAGER from "..";
import translate, { translateAny } from "../game/lang";
import StyledText from "./StyledText";
import "./roleSpecific.css";
import { useGameState, usePlayerState } from "./useHooks";

export default function SelectionInformation(): ReactElement | null {
    const phaseType = useGameState(gameState => gameState.phaseState.type, ["phase"])!;
    const targets = usePlayerState(playerState => playerState.targets, ["yourSelection"])!;
    const players = useGameState(gameState => gameState.players, ["yourButtons", "gamePlayers"])!;
    const role = usePlayerState(playerState => playerState.roleState?.type, ["yourRoleState"])!;

    const shouldShow = 
        phaseType === "night" 
        && (
            players.some(player => Object.values(player.buttons).includes(true))
            || targets.length !== 0
        );

    if (shouldShow) {
        let selectionText;

        if (targets.length === 0) {
            selectionText = translateAny([`role.${role}.youAreSelecting.null`, "youAreSelecting.null"])
        } else {
            const targetNames = targets.map(playerIndex => players[playerIndex].toString());

            if (role === "framer" && targets.length === 2) {
                selectionText = translateAny(
                    [`role.${role}.youAreSelecting.2`, "youAreSelecting"],
                    ...targetNames
                )
            } else if (role === "marksman") {
                selectionText = translate("role.marksman.youAreSelecting", targetNames.join(", "));
            }else {
                selectionText = translateAny(
                    [`role.${role}.youAreSelecting`, "youAreSelecting"],
                    ...targetNames,
                    ...[translate("nobody"), translate("nobody"), translate("nobody")] // a little insurance
                )
            }
        }
        
        return <div className="role-information">
            <div>
                <StyledText>{selectionText}</StyledText>
            </div>
            {!!targets.length && <button className="button gm-button" onClick={()=>{
                GAME_MANAGER.sendTargetPacket([]);
            }}>{translate("button.clear")}</button>}
        </div>;
    }
    return null;
}