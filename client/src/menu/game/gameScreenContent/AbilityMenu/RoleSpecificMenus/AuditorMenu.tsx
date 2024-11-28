import { ReactElement } from "react"
import React from "react"
import { Role, RoleState } from "../../../../../game/roleState.d"
import TwoRoleOutlineOptionSelectionMenu from "../AbilitySelectionTypes/TwoRoleOutlineOptionSelectionMenu"
import GAME_MANAGER from "../../../../.."
import { AbilityInput, AbilitySelection, TwoRoleOutlineOptionSelection } from "../../../../../game/abilityInput"

export type AuditorResult = {
    type: "two",
    roles: [Role, Role]
} | {
    type: "one",
    role: Role
}

export default function AuditorMenu(props: Readonly<{
    roleState: RoleState & {type: "auditor"}
}>): ReactElement {
    const onInput = (chosenOutlines: TwoRoleOutlineOptionSelection) => {
        const selection: AbilitySelection = {
            type: "twoRoleOutlineOption" as const,
            selection: chosenOutlines
        }

        const input: AbilityInput = {
            id: {
                type: "role",
                role: "ojo",
                id: 0
            },
            selection: selection
        }
        GAME_MANAGER.sendAbilityInput(input);
    }
    
    return <TwoRoleOutlineOptionSelectionMenu
        previouslyGivenResults={props.roleState.previouslyGivenResults}
        chosenOutlines={props.roleState.chosenOutline}
        onChoose={onInput}
    />
}