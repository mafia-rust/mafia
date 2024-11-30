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

    const savedAbilities = usePlayerState(
        playerState => playerState.savedAbilities,
        ["yourSavedAbilities"]
    )!;
    const savedAbilitiesMap = new ListMap(savedAbilities, (k1, k2) => abilityIdToString(k1) === abilityIdToString(k2));

    let singleAbilitySave = savedAbilitiesMap.get({type: "role", role: "auditor", id: 0});

    let newSelection;
    let newAvailable;
    if(
        singleAbilitySave !== null &&
        singleAbilitySave.selection.type === "twoRoleOutlineOption" &&
        singleAbilitySave.availableAbilityData.available.type === "twoRoleOutlineOption"
    ){
        newSelection = singleAbilitySave.selection.selection;
        newAvailable = singleAbilitySave.availableAbilityData.available.selection;
    } else {
        newSelection = undefined;
        newAvailable = undefined;
    }

    return <TwoRoleOutlineOptionSelectionMenu
        previouslyGivenResults={props.roleState.previouslyGivenResults}
        selection={newSelection}
        available={newAvailable}
        onChoose={onInput}
    />
}