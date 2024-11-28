import { ReactElement, useState } from "react";
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
    AbilitySelection,
    translateAbilityId,
    AvailableAbilitySelection,
    defaultAbilitySelection,
    abilityIdToString
} from "../../../../game/abilityInput";
import React from "react";
import { usePlayerState } from "../../../../components/useHooks";
import { Button } from "../../../../components/Button";
import OnePlayerOptionSelectionMenu from "./AbilitySelectionTypes/OnePlayerOptionSelectionMenu";
import TwoRoleOutlineOptionSelectionMenu from "./AbilitySelectionTypes/TwoRoleOutlineOptionSelectionMenu";
import GAME_MANAGER from "../../../..";
import ListMap from "../../../../ListMap";
import TwoRoleOptionSelectionMenu from "./AbilitySelectionTypes/TwoRoleOptionSelectionMenu";
import TwoPlayerOptionSelectionMenu from "./AbilitySelectionTypes/TwoPlayerOptionSelectionMenu";
import ChatMessage from "../../../../components/ChatMessage";
import StyledText from "../../../../components/StyledText";
import CheckBox from "../../../../components/CheckBox";




export default function GenericAbilityMenu(): ReactElement {

    const availableAbilitySelection = usePlayerState(
        playerState => playerState.availableAbilitySelection,
        ["yourAvailableAbilityInput"]
    )!;
    const selectedAbilitySelection = usePlayerState(
        playerState => playerState.abilitySelection,
        ["yourSavedAbilityInput"]
    )!;

    const selectedAbilitySelectionTypeMap = new ListMap(
        selectedAbilitySelection,
        (k1, k2)=>abilityIdToString(k1) === abilityIdToString(k2)
    );

    return <>
        {availableAbilitySelection.map(([id, availableSelectionType], i) => {
            return <SingleAbilityMenu
                key={i}
                abilityId={id}
                available={availableSelectionType}
                selected={selectedAbilitySelectionTypeMap.get(id)}
            />
        })
    }</>
}


function SingleAbilityMenu(props: Readonly<{
    abilityId: AbilityID,
    key: number,
    available: AvailableAbilitySelection,
    selected: AbilitySelection | null
}>): ReactElement {

    const [open, setOpen] = useState<boolean>(true);

    const myIndex = usePlayerState(
        (playerState, gameState)=>playerState.myIndex
    )!;

    
    return <details key={props.key} className="role-specific-colors small-role-specific-menu" open={open}>
        <summary
            onClick={(e)=>{
                e.preventDefault();
                setOpen(!open);
            }}
        >
            <StyledText>{translateAbilityId(props.abilityId)}</StyledText>
        </summary>

        {props.selected!==null?
            <ChatMessage message={{
                variant: {
                    type: "abilityUsed",
                    player: myIndex,
                    abilityId: props.abilityId,
                    selection: props.selected
                },
                chatGroup: "all"
            }}/>
        :null}
        <SwitchSingleAbilityMenuType
            key={props.key}
            id={props.abilityId}
            available={props.available}
            selected={props.selected}
        />
    </details>
}


function SwitchSingleAbilityMenuType(props: Readonly<{
    key: number,
    id: AbilityID,
    available: AvailableAbilitySelection,
    selected: AbilitySelection | null
}>): ReactElement {

    const {key, id, available} = props;
    let selected: AbilitySelection;

    if (props.selected === null || props.selected.type !== props.available.type) {
        selected = defaultAbilitySelection(props.available);    
    }else{
        selected = props.selected;
    }

    switch(available.type) {
        case "unit":
            return <Button key={key}>
                {translateAbilityId(props.id)}
            </Button>
        case "boolean":{
            let bool;
            if(selected === null || selected.type !== "boolean"){
                bool = false;
            }else{
                bool = selected.selection;
            }
            return <div><CheckBox key={key} checked={bool} onChange={(x)=>{
                GAME_MANAGER.sendAbilityInput({
                    id, 
                    selection: {
                        type: "boolean",
                        selection: x
                    }
                });
            }}/></div>;
        }
        case "onePlayerOption":{
            
            let selectedPlayer;
            if(selected === null || selected.type !== "onePlayerOption"){
                selectedPlayer = null;
            }else{
                selectedPlayer = selected.selection;
            }
            
            return <OnePlayerOptionSelectionMenu
                key={key}
                availablePlayers={available.selection}
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
                props.selected === null ||
                props.selected.type !== "twoPlayerOption"
            ){
                input = [null, null];
            }else{
                input = props.selected.selection;
            }

            return <TwoPlayerOptionSelectionMenu
                key={props.key}
                selection={input}
                availableSelection={available.selection}
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
                props.selected === null ||
                props.selected.type !== "twoRoleOption"
            ){
                input = [null, null];
            }else{
                input = props.selected.selection;
            }

            return <TwoRoleOptionSelectionMenu
                key={props.key}
                input={input}
                availableSelection={available.selection}
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
                key={props.key}
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
            return <></>;
    }
}
