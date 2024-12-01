import { ReactElement } from "react"
import React from "react"
import { Role } from "../../../../../game/roleState.d"
import { usePlayerState } from "../../../../../components/useHooks"
import TwoRoleOutlineOptionInputMenu from "../AbilitySelectionTypes/TwoRoleOutlineOptionInputMenu"
import GAME_MANAGER from "../../../../.."
import { TwoRoleOutlineOptionInput } from "../../../../../game/abilityInput"

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

    const onInput = (chosenOutlines: TwoRoleOutlineOptionInput) => {
        const input = {
            type: "auditor" as const,
            input: chosenOutlines
        };
        GAME_MANAGER.sendAbilityInput(input);
    }
    
    return <TwoRoleOutlineOptionInputMenu
        previouslyGivenResults={previouslyGivenResults}
        chosenOutlines={chosenOutlines}
        onChoose={onInput}
    />
}