import { ReactElement } from "react"
import React from "react"
import { Role, RoleState } from "../../../../../game/roleState.d"
import TwoRoleOutlineOptionSelectionMenu from "../AbilitySelectionTypes/TwoRoleOutlineOptionSelectionMenu"
import GAME_MANAGER from "../../../../.."
import { abilityIdToString, TwoRoleOutlineOptionSelection } from "../../../../../game/abilityInput"
import { usePlayerState } from "../../../../../components/useHooks"
import ListMap from "../../../../../ListMap"

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
    const onInput = (selection: TwoRoleOutlineOptionSelection) => {
        GAME_MANAGER.sendAbilityInput({
            id: {
                type: "role",
                role: "auditor",
                id: 0
            },
            selection: {
                type: "twoRoleOutlineOption",
                selection: selection
            }
        });
    }

    const availableAbilitySelection = usePlayerState(
        playerState => playerState.availableAbilitySelection,
        ["yourSavedAbilityInput", "yourAvailableAbilityInput"]
    )!;
    const selectedAbilitySelection = usePlayerState(
        playerState => playerState.abilitySelection,
        ["yourSavedAbilityInput", "yourAvailableAbilityInput"]
    )!;

    const selectedAbilitySelectionMap = new ListMap(selectedAbilitySelection, (k1, k2) => abilityIdToString(k1) === abilityIdToString(k2));
    let selection = selectedAbilitySelectionMap.get({type: "role", role: "auditor", id: 0});

    const availableAbilitySelectionMap = new ListMap(availableAbilitySelection, (k1, k2) => abilityIdToString(k1) === abilityIdToString(k2));
    let available = availableAbilitySelectionMap.get({type: "role", role: "auditor", id: 0});

    let newSelection;
    if(selection !== undefined && selection?.type === "twoRoleOutlineOption") {
        newSelection = selection.selection;
    } else {
        newSelection = undefined;
    }

    let newAvailable;
    if(available !== undefined && available?.type === "twoRoleOutlineOption") {
        newAvailable = available.selection;
    } else {
        newAvailable = undefined;
    }

    return <TwoRoleOutlineOptionSelectionMenu
        previouslyGivenResults={props.roleState.previouslyGivenResults}
        selection={newSelection}
        available={newAvailable}
        onChoose={onInput}
    />
}