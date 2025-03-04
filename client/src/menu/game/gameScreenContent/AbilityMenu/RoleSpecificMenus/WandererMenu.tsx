import { ReactElement } from "react"
import React from "react"
import { Role, RoleState } from "../../../../../game/roleState.d"
import RoleOutlineOptionSelectionMenu from "../AbilitySelectionTypes/RoleOutlineOptionSelectionMenu"
import GAME_MANAGER from "../../../../.."
import { controllerIdToLink, RoleOutlineOptionSelection } from "../../../../../game/abilityInput"
import { usePlayerState } from "../../../../../components/useHooks"
import ListMap from "../../../../../ListMap"

export type AuditorResult = {
    type: "two",
    roles: [Role, Role]
} | {
    type: "one",
    role: Role
}

export default function WandererMenu(): ReactElement {
    
    const myPlayerIndex = usePlayerState(
        state=>state.myIndex,
        ["yourPlayerIndex"]
    )!;

    const onInput = (selection: RoleOutlineOptionSelection) => {
        GAME_MANAGER.sendAbilityInput({
            id: {
                type: "role",
                role: "wanderer",
                player: myPlayerIndex,
                id: 0
            },
            selection: {
                type: "roleOutlineOption",
                selection: selection
            }
        });
    }
    

    const savedAbilities = usePlayerState(
        playerState => playerState.savedControllers,
        ["yourAllowedControllers"]
    )!;
    
    const savedAbilitiesMap = new ListMap(savedAbilities, (k1, k2) => controllerIdToLink(k1) === controllerIdToLink(k2));

    let singleAbilitySave = savedAbilitiesMap.get({
        type: "role",
        role: "auditor",
        player: myPlayerIndex,
        id: 0
    });

    let newSelection;
    let newAvailable;
    if(
        singleAbilitySave !== null &&
        singleAbilitySave.selection.type === "roleOutlineOption" &&
        singleAbilitySave.availableAbilityData.available.type === "roleOutlineOption"
    ){
        newSelection = singleAbilitySave.selection.selection;
        newAvailable = singleAbilitySave.availableAbilityData.available.selection;
    } else {
        newSelection = undefined;
        newAvailable = undefined;
    }

    return <RoleOutlineOptionSelectionMenu
        previouslyGivenResults={undefined}
        selection={newSelection}
        available={newAvailable}
        onChoose={onInput}
    />
}