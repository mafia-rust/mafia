import { ReactElement } from "react";
import { 
    AvailableOnePlayerOptionSelection, 
    OnePlayerOptionSelection, 
    
    AvailableTwoPlayerOptionSelection, 
    TwoPlayerOptionSelection, 
    
    AvailableTwoRoleOptionSelection, 
    TwoRoleOptionSelection, 
    
    AvailableTwoRoleOutlineOptionSelection, 
    TwoRoleOutlineOptionSelection,
    AbilityID,
    AbilitySelection
} from "../../../../game/abilityInput";
import React from "react";
import { usePlayerState } from "../../../../components/useHooks";
import { Button } from "../../../../components/Button";
import OnePlayerOptionSelectionMenu from "./AbilitySelectionTypes/OnePlayerOptionSelectionMenu";
import TwoRoleOutlineOptionSelectionMenu from "./AbilitySelectionTypes/TwoRoleOutlineOptionSelectionMenu";
import GAME_MANAGER from "../../../..";
import ListMap from "../../../../ListMap";
import translate from "../../../../game/lang";
import TwoRoleOptionSelectionMenu from "./AbilitySelectionTypes/TwoRoleOptionSelectionMenu";
import TwoPlayerOptionSelectionMenu from "./AbilitySelectionTypes/TwoPlayerOptionSelectionMenu";


export default function AbilityMenu(): ReactElement {

    const availableAbilitySelection = usePlayerState(
        playerState => playerState.availableAbilitySelection,
        ["yourAvailableAbilityInput"]
    )!;
    const selectedAbilitySelection = usePlayerState(
        playerState => playerState.abilitySelection,
        ["yourSavedAbilityInput"]
    )!;

    const role = usePlayerState(
        playerState => playerState.roleState.type,
        ["yourRoleState"]
    )!;

    const selectedAbilitySelectionTypeMap = new ListMap(selectedAbilitySelection);

    return <>
        {availableAbilitySelection.map(([id, availableSelectionType], i) => {
            const selectedAbilitySelectionType = selectedAbilitySelectionTypeMap.get(id);

            switch(availableSelectionType.type) {
                case "unit":
                    return <Button key={i}>
                        {translate("ability.abilityId."+translate("role."+role+".name")+"."+id+".name")}
                    </Button>
                case "onePlayerOption":{
                    
                    let selectedPlayer;
                    if(selectedAbilitySelectionType === null || selectedAbilitySelectionType.type !== "onePlayerOption"){
                        selectedPlayer = null;
                    }else{
                        selectedPlayer = selectedAbilitySelectionType.selection;
                    }
                    
                    return <OnePlayerOptionSelectionMenu
                        key={i}
                        availablePlayers={availableSelectionType.selection}
                        selectedPlayer={selectedPlayer}
                        onChoose={(player) => {
                            GAME_MANAGER.sendAbilityInput({
                                id, 
                                selection: {
                                    type: "onePlayerOption",
                                    selection: player
                                }
                            });
                        }}
                    />;
                }
                case "twoPlayerOption":{
                    let input: TwoPlayerOptionSelection;
                    if(
                        selectedAbilitySelectionType === null ||
                        selectedAbilitySelectionType.type !== "twoPlayerOption"
                    ){
                        input = [null, null];
                    }else{
                        input = selectedAbilitySelectionType.selection;
                    }

                    return <TwoPlayerOptionSelectionMenu
                        key={i}
                        selection={input}
                        availableSelection={availableSelectionType.selection}
                        onChoose={(selection) => {
                            GAME_MANAGER.sendAbilityInput({
                                id, 
                                selection: {
                                    type: "twoPlayerOption",
                                    selection
                                }
                            });
                        }}
                    />;
                }
                case "twoRoleOption":{

                    let input: TwoRoleOptionSelection;
                    if(
                        selectedAbilitySelectionType === null ||
                        selectedAbilitySelectionType.type !== "twoRoleOption"
                    ){
                        input = [null, null];
                    }else{
                        input = selectedAbilitySelectionType.selection;
                    }

                    return <TwoRoleOptionSelectionMenu
                        key={i}
                        input={input}
                        availableSelection={availableSelectionType.selection}
                        onChoose={(selection) => {
                            GAME_MANAGER.sendAbilityInput({
                                id,
                                selection: {
                                    type: "twoRoleOption",
                                    selection: selection
                                }
                            });
                        }}
                    />;
                }
                case "twoRoleOutlineOption":{
                    return <TwoRoleOutlineOptionSelectionMenu
                        key={i}
                        onChoose={(selection) => {
                            GAME_MANAGER.sendAbilityInput({
                                id,
                                selection: {
                                    type: "twoRoleOutlineOption",
                                    selection: selection
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


