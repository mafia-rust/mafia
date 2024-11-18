import { ReactElement } from "react";
import { 
    AvailableOnePlayerOptionSelection, 
    OnePlayerOptionSelection, 
    
    AvailableTwoPlayerOptionSelection, 
    TwoPlayerOptionSelection, 
    
    AvailableTwoRoleOptionSelection, 
    TwoRoleOptionSelection, 
    
    AvailableTwoRoleOutlineOptionSelection, 
    TwoRoleOutlineOptionSelection
} from "../../../../game/abilityInput";
import React from "react";
import { usePlayerState } from "../../../../components/useHooks";
import { Button } from "../../../../components/Button";
import OnePlayerOptionSelectionMenu from "./AbilitySelectionTypes/OnePlayerOptionSelectionMenu";
import TwoRoleOutlineOptionSelectionMenu from "./AbilitySelectionTypes/TwoRoleOutlineOptionSelectionMenu";
import GAME_MANAGER from "../../../..";
import ListMap, { ListMapData } from "../../../../ListMap";
import translate from "../../../../game/lang";
import TwoRoleOptionSelectionMenu from "./AbilitySelectionTypes/TwoRoleOptionSelectionMenu";
import TwoPlayerOptionSelectionMenu from "./AbilitySelectionTypes/TwoPlayerOptionSelectionMenu";


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
    type: "twoPlayerOptionSelection",
    selection: AvailableTwoPlayerOptionSelection,
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
    type: "twoPlayerOptionSelection",
    selection: TwoPlayerOptionSelection
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

    const role = usePlayerState(
        playerState => playerState.roleState.type,
        ["yourRoleState"]
    )!;

    return <>
        {availableGenericAbilitySelection.input.map(([id, availableSelectionType]) => {
            
            const selectedGenericAbilitySelectionTypeMap = new ListMap(selectedGenericAbilitySelection.input);
            const selectedGenericAbilitySelectionType = selectedGenericAbilitySelectionTypeMap.get(id);

            switch(availableSelectionType.type) {
                case "unitSelection":
                    return <Button key={id}>
                        {translate("genericAbility.abilityId."+translate("role."+role+".name")+"."+id+".name")}
                    </Button>
                case "onePlayerOptionSelection":{
                    
                    let selectedPlayer;
                    if(selectedGenericAbilitySelectionType === null || selectedGenericAbilitySelectionType.type !== "onePlayerOptionSelection"){
                        selectedPlayer = null;
                    }else{
                        selectedPlayer = selectedGenericAbilitySelectionType.selection;
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
                    />;
                }
                case "twoPlayerOptionSelection":{
                    let input: TwoPlayerOptionSelection;
                    if(
                        selectedGenericAbilitySelectionType === null ||
                        selectedGenericAbilitySelectionType.type !== "twoPlayerOptionSelection"
                    ){
                        input = [null, null];
                    }else{
                        input = selectedGenericAbilitySelectionType.selection;
                    }

                    return <TwoPlayerOptionSelectionMenu
                        key={id}
                        selection={input}
                        availableSelection={availableSelectionType.selection}
                        onChoose={(selection) => {
                            const mapInput: ListMap<GenericAbilityID, GenericAbilitySelectionType> = new ListMap();
                            mapInput.set(id, {
                                type: "twoPlayerOptionSelection",
                                selection: selection
                            });

                            GAME_MANAGER.sendAbilityInput({
                                type: "genericAbility",
                                selection: {
                                    input: mapInput.entries()
                                }
                            });
                        }}
                    />;
                }
                case "twoRoleOptionSelection":{

                    let input: TwoRoleOptionSelection;
                    if(
                        selectedGenericAbilitySelectionType === null ||
                        selectedGenericAbilitySelectionType.type !== "twoRoleOptionSelection"
                    ){
                        input = [null, null];
                    }else{
                        input = selectedGenericAbilitySelectionType.selection;
                    }

                    return <TwoRoleOptionSelectionMenu
                        key={id}
                        input={input}
                        availableSelection={availableSelectionType.selection}
                        onChoose={(selection) => {
                            const mapInput: ListMap<GenericAbilityID, GenericAbilitySelectionType> = new ListMap();
                            mapInput.set(id, {
                                type: "twoRoleOptionSelection",
                                selection: selection
                            });

                            GAME_MANAGER.sendAbilityInput({
                                type: "genericAbility",
                                selection: {
                                    input: mapInput.entries()
                                }
                            });
                        }}
                    />;
                }
                case "twoRoleOutlineOptionSelection":{
                    return <TwoRoleOutlineOptionSelectionMenu
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
                }
                default:
                    return null;
            }
        })
    }</>
}


