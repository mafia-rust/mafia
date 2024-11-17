import { ReactElement } from "react";
import { AvailableOnePlayerOptionSelection, AvailableTwoRoleOptionSelection, AvailableTwoRoleOutlineOptionSelection, OnePlayerOptionSelection, TwoRoleOptionSelection, TwoRoleOutlineOptionSelection } from "../../../../game/abilityInput";
import React from "react";
import { usePlayerState } from "../../../../components/useHooks";
import { Button } from "../../../../components/Button";
import OnePlayerOptionSelectionMenu from "./AbilitySelectionTypes/OnePlayerOptionSelectionMenu";
import TwoPlayerOptionSelectionType from "./AbilitySelectionTypes/TwoPlayerOptionSelectionMenu";
import TwoRoleOutlineOptionInputMenu from "./AbilitySelectionTypes/TwoRoleOutlineOptionMenu";
import GAME_MANAGER from "../../../..";
import ListMap, { ListMapData } from "../../../../ListMap";


export type GenericAbilityID = number;
export type AvailableGenericAbilitySelection = {
    input: ListMapData<GenericAbilityID, AvailableGenericAbilitySelectionType>,
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
    input: ListMapData<GenericAbilityID, GenericAbilitySelectionType>
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
    const selectedGenericAbilitySelection = usePlayerState(
        playerState => playerState.genericAbilitySelection,
        ["genericAbilitySelection", "clearGenericAbilitySelection"]
    )!;

    return <>
        {availableGenericAbilitySelection.input.map(([id, availableSelectionType]) => {
            switch(availableSelectionType.type) {
                case "unitSelection":
                    return <Button
                        key={id}
                    >UNIT SELECT</Button>
                case "onePlayerOptionSelection":

                    const selectedGenericAbilitySelectionType = new ListMap(selectedGenericAbilitySelection.input);
                    let x = selectedGenericAbilitySelectionType.get(id);
                    let selectedPlayer;
                    if(x === null || x.type !== "onePlayerOptionSelection"){
                        selectedPlayer = null;
                    }else{
                        selectedPlayer = x.selection;
                    }

                    return <OnePlayerOptionSelectionMenu
                        key={id}
                        availablePlayers={availableSelectionType.selection}
                        selectedPlayer={selectedPlayer}
                        onChoose={(player) => {
                            
                            const mapInput: ListMap<GenericAbilityID, GenericAbilitySelectionType> = new ListMap();
                            mapInput.set(id, {
                                type: "onePlayerOptionSelection",
                                selection: player
                            });

                            GAME_MANAGER.sendAbilityInput({
                                type: "genericAbility",
                                selection: {
                                    input: mapInput.entries()
                                }
                            });
                        }}
                    />
                case "twoRoleOptionSelection":
                    return <TwoPlayerOptionSelectionType
                        key={id}
                    />
                case "twoRoleOutlineOptionSelection":
                    return <TwoRoleOutlineOptionInputMenu
                        key={id}
                        onChoose={(selection) => {

                            const mapInput: ListMap<GenericAbilityID, GenericAbilitySelectionType> = new ListMap();
                            mapInput.set(id, {
                                type: "twoRoleOutlineOptionSelection",
                                selection: selection
                            });

                            GAME_MANAGER.sendAbilityInput({
                                type: "genericAbility",
                                selection: {
                                    input: mapInput.entries()
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


