import { ReactElement } from "react";
import { AvailableOnePlayerOptionSelection, AvailableTwoRoleOptionSelection, AvailableTwoRoleOutlineOptionSelection, OnePlayerOptionSelection, TwoRoleOptionSelection, TwoRoleOutlineOptionSelection } from "../../../../game/abilityInput";
import React from "react";
import { usePlayerState } from "../../../../components/useHooks";
import { Button } from "../../../../components/Button";
import OnePlayerOptionSelectionMenu from "./AbilitySelectionTypes/OnePlayerOptionSelectionMenu";
import TwoPlayerOptionSelectionType from "./AbilitySelectionTypes/TwoPlayerOptionSelectionMenu";
import TwoRoleOutlineOptionInputMenu from "./AbilitySelectionTypes/TwoRoleOutlineOptionMenu";
import GAME_MANAGER from "../../../..";


type GenericAbilityID = number;
export type AvailableGenericAbilitySelection = {
    input: Partial<Record<GenericAbilityID, AvailableGenericAbilitySelectionType>>,
}
export type AvailableGenericAbilitySelectionType = {
    type: "unitSelection",
} | {
    type: "onePlayerOptionSelection",
    selection: AvailableOnePlayerOptionSelection,
} | {
    type: "twoRoleOptionSelection",
    selection: AvailableTwoRoleOptionSelection,
} | {
    type: "twoRoleOutlineOptionSelection",
    selection: AvailableTwoRoleOutlineOptionSelection,
}


export type GenericAbilitySelection = {
    input: [GenericAbilityID, GenericAbilitySelectionType][],
}
export type GenericAbilitySelectionType = {
    type: "unitSelection",
} | {
    type: "onePlayerOptionSelection",
    selection: OnePlayerOptionSelection
} | {
    type: "twoRoleOptionSelection",
    selection: TwoRoleOptionSelection
} | {
    type: "twoRoleOutlineOptionSelection",
    selection: TwoRoleOutlineOptionSelection
}

export default function GenericAbilityMenu(): ReactElement {

    const availableGenericAbilitySelection = usePlayerState(
        playerState => playerState.availableGenericAbilitySelection,
        ["availableGenericAbilitySelection"]
    )!;

    return <>
        Generic MENU
        {JSON.stringify(availableGenericAbilitySelection)}
        {Object.keys(availableGenericAbilitySelection.input).map((id: string) => {
            let idNum = parseInt(id);
            const selectionType = availableGenericAbilitySelection.input[idNum];
            if(selectionType === undefined) {return null}

            switch(selectionType.type) {
                case "unitSelection":
                    return <Button>UNIT SELECT</Button>
                case "onePlayerOptionSelection":

                    
                    /* 
                        {
                            "type":"abilityInput",
                            "abilityInput":{
                                "type":"genericAbility",
                                "selection":{
                                    "input":{
                                        "0":{"type":"onePlayerOptionSelection","selection":0}
                                    }
                                }
                            }
                        }
                    */

                    return <OnePlayerOptionSelectionMenu
                        availablePlayers={selectionType.selection}
                        onChoose={(player) => {
                            
                            const mapInput: [GenericAbilityID, GenericAbilitySelectionType][] = [];
                            mapInput.push([idNum, {
                                type: "onePlayerOptionSelection",
                                selection: player
                            }]);

                            GAME_MANAGER.sendAbilityInput({
                                type: "genericAbility",
                                selection: {
                                    input: mapInput
                                }
                            });
                        }}
                    />
                case "twoRoleOptionSelection":
                    return <TwoPlayerOptionSelectionType/>
                case "twoRoleOutlineOptionSelection":
                    return <TwoRoleOutlineOptionInputMenu
                        onChoose={(selection) => {

                            const mapInput: [GenericAbilityID, GenericAbilitySelectionType][] = [];
                            mapInput.push([idNum, {
                                type: "twoRoleOutlineOptionSelection",
                                selection: selection
                            }]);

                            GAME_MANAGER.sendAbilityInput({
                                type: "genericAbility",
                                selection: {
                                    input: mapInput
                                }
                            });
                        }}
                    />
                default:
                    return null;
            }
        })
    }</>
}


