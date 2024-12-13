import { ReactElement } from "react"
import React from "react"
import { usePlayerState } from "../../../../../components/useHooks";
import GAME_MANAGER from "../../../../..";
import { RoleState } from "../../../../../game/roleState.d";
import TwoRoleOutlineOptionSelectionMenu from "../AbilitySelectionTypes/TwoRoleOutlineOptionSelectionMenu";
import { AbilityInput, AbilitySelection, controllerIdToLink, TwoRoleOutlineOptionSelection } from "../../../../../game/abilityInput";
import ListMap from "../../../../../ListMap";


export default function OjoMenu(
    props: {
        roleState: RoleState & {type: "ojo"}
    }
): ReactElement | null {
    const myPlayerIndex = usePlayerState(
        state=>state.myIndex,
        ["yourPlayerIndex"]
    )!;

    const onInputOutline = (chosenOutlines: TwoRoleOutlineOptionSelection) => {
        const selection: AbilitySelection = {
            type: "twoRoleOutlineOption" as const,
            selection: chosenOutlines
        }

        const input: AbilityInput = {
            id: {
                type: "role",
                player: myPlayerIndex,
                role: "ojo",
                id: 0
            },
            selection: selection
        }
        GAME_MANAGER.sendAbilityInput(input);
    }

    const savedAbilities = usePlayerState(
        playerState => playerState.savedControllers,
        ["yourAllowedControllers"]
    )!;
    
    const savedAbilitiesMap = new ListMap(savedAbilities, (k1, k2) => controllerIdToLink(k1) === controllerIdToLink(k2));

    let singleAbilitySave = savedAbilitiesMap.get({
        type: "role",
        role: "ojo",
        player: myPlayerIndex,
        id: 0
    });

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


    return <>
        <TwoRoleOutlineOptionSelectionMenu
            previouslyGivenResults={props.roleState.previouslyGivenResults}
            selection={newSelection}
            available={newAvailable}
            onChoose={onInputOutline}
        />
    </>
}