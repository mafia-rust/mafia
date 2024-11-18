import { ReactElement } from "react"
import React from "react"
import { Role } from "../../../../../game/roleState.d"
import { usePlayerState } from "../../../../../components/useHooks"
import TwoRoleOutlineOptionSelectionMenu from "../AbilitySelectionTypes/TwoRoleOutlineOptionSelectionMenu"
import GAME_MANAGER from "../../../../.."
import { TwoRoleOutlineOptionSelection } from "../../../../../game/abilityInput"

export type AuditorResult = {
    type: "two",
    roles: [Role, Role]
} | {
    type: "one",
    role: Role
}

export default function AuditorMenu(props: {}): ReactElement {
    const previouslyGivenResults = usePlayerState(
        (playerState, gameState)=>{
            if(playerState.roleState?.type === "auditor"){
                return playerState.roleState.previouslyGivenResults;
            }
            return [];
        },
        ["yourRoleState"]
    )!;
    const chosenOutlines = usePlayerState(
        (playerState, gameState)=>{
            if(playerState.roleState?.type === "auditor"){
                return playerState.roleState.chosenOutline;
            }
            return null;
        },
        ["yourRoleState"]
    )!;

    const onInput = (chosenOutlines: TwoRoleOutlineOptionSelection) => {
        const input = {
            type: "auditor" as const,
            selection: chosenOutlines
        };
        GAME_MANAGER.sendAbilityInput(input);
    }
    
    return <TwoRoleOutlineOptionSelectionMenu
        previouslyGivenResults={previouslyGivenResults}
        chosenOutlines={chosenOutlines}
        onChoose={onInput}
    />
}