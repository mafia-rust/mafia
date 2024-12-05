import { ReactElement } from "react";
import { 
    TwoPlayerOptionSelection, 
    TwoRoleOptionSelection, 
    ControllerID,
    AbilitySelection,
    translateControllerID,
    AvailableAbilitySelection,
    TwoRoleOutlineOptionSelection,
    RoleOptionSelection,
    SavedController,
    controllerIdToLink,
    singleAbilityJsonData,
    StringSelection,
    ThreePlayerOptionSelection
} from "../../../../game/abilityInput";
import React from "react";
import { usePlayerState } from "../../../../components/useHooks";
import { Button } from "../../../../components/Button";
import OnePlayerOptionSelectionMenu from "./AbilitySelectionTypes/OnePlayerOptionSelectionMenu";
import TwoRoleOutlineOptionSelectionMenu from "./AbilitySelectionTypes/TwoRoleOutlineOptionSelectionMenu";
import GAME_MANAGER from "../../../..";
import TwoRoleOptionSelectionMenu from "./AbilitySelectionTypes/TwoRoleOptionSelectionMenu";
import TwoPlayerOptionSelectionMenu from "./AbilitySelectionTypes/TwoPlayerOptionSelectionMenu";
import StyledText from "../../../../components/StyledText";
import CheckBox from "../../../../components/CheckBox";
import KiraSelectionMenu, { KiraSelection } from "./AbilitySelectionTypes/KiraSelectionMenu";
import RoleOptionSelectionMenu from "./AbilitySelectionTypes/RoleOptionSelectionMenu";
import "./genericAbilityMenu.css";
import DetailsSummary from "../../../../components/DetailsSummary";
import translate from "../../../../game/lang";
import StringSelectionMenu from "./AbilitySelectionTypes/StringSelectionMenu";
import ListMap from "../../../../ListMap";
import { Role } from "../../../../game/roleState.d";
import { PlayerIndex } from "../../../../game/gameState.d";
import Icon from "../../../../components/Icon";
import ThreePlayerOptionSelectionMenu from "./AbilitySelectionTypes/ThreePlayerOptionSelectionMenu";

type GroupName = `${PlayerIndex}/${Role}` | "syndicateGunItem" | ControllerID["type"];

type ControllerGroupsMap = ListMap<
    GroupName, 
    ListMap<ControllerID, SavedController>
>;

function getGroupNameFromControllerID(id: ControllerID): GroupName {
    switch (id.type){
        case "role":
            return id.player+"/"+id.role as `${PlayerIndex}/${Role}`
        case "syndicateGunItemGive":
        case "syndicateGunItemShoot":
            return "syndicateGunItem";
        default:
            return id.type;
    }
}

function translateGroupName(id: ControllerID): string {
    switch (id.type){
        case "role":
            return translate("role."+id.role+".name");
        case "syndicateGunItemGive":
        case "syndicateGunItemShoot":
            return translate("syndicateGunItem");
        default:
            return id.type;
    }
}

export default function GenericAbilityMenu(): ReactElement {
    const savedAbilities = usePlayerState(
        playerState => playerState.savedControllers,
        ["yourAllowedControllers"]
    )!;

    let controllerGroupsMap: ControllerGroupsMap = new ListMap();
    //build this map ^
    for(let [controllerID, controller] of savedAbilities) {
        let groupName = getGroupNameFromControllerID(controllerID);
        
        let controllers = controllerGroupsMap.get(groupName);
        if(controllers === null){
            controllers = new ListMap([], (k1, k2)=>controllerIdToLink(k1)===controllerIdToLink(k2));
        }

        controllers.insert(controllerID, controller);
        controllerGroupsMap.insert(groupName, controllers);
    }

    return <>
        {controllerGroupsMap.entries().map(([group, controllers], i)=>{

            let firstController = controllers.entries()[0]
            if(firstController !== undefined && controllers.entries().length === 1){
                return <SingleAbilityMenu
                    key={i}
                    abilityId={firstController[0]}
                    saveData={firstController[1]}
                />
            }else{
                return <MultipleControllersMenu
                    key={i}
                    groupName={group}
                    controllers={controllers}
                />
            }
        })}
    </>
}

function MultipleControllersMenu(props: Readonly<{
    groupName: string,
    controllers: ListMap<ControllerID, SavedController>
}>): ReactElement {

    const disabled = !props.controllers.values().some((controller)=>!controller.availableAbilityData.grayedOut)
    const nightIcon = !props.controllers.keys().some(
        (id)=>!singleAbilityJsonData(controllerIdToLink(id))?.midnight
    );


    let anyControllerId = props.controllers.keys()[0]
    let groupName = "";
    if(anyControllerId !== undefined){
        groupName = translateGroupName(anyControllerId)
    }else{
        return <></>;
    }

    return <DetailsSummary
        className="generic-ability-menu"
        summary={
            <div className="generic-ability-menu-tab-summary">
                <StyledText>{groupName}</StyledText>
                {nightIcon?<span>{translate("night.icon")}</span>:null}
            </div>
        }
        defaultOpen={true}
        disabled={disabled}
    >
        {props.controllers.entries().map(([id, saveData], i) => {
            return <SingleAbilityMenu
                key={i}
                abilityId={id}
                saveData={saveData}
                includeDropdown={false}
            />
        })}
    </DetailsSummary>
}

function SingleAbilityMenu(props: Readonly<{
    abilityId: ControllerID,
    key: number,
    saveData: SavedController,
    includeDropdown?: boolean
}>): ReactElement {
    const nightIcon = singleAbilityJsonData(controllerIdToLink(props.abilityId))?.midnight;

    //The chat message makes it more verbose, showing more relevant information
    // as menus get large, it makes it harder to parse. so i keep it out for now
    const inner = <>
        {/* {props.saveData.availableAbilityData.dontSave ? null : 
            <ChatMessage message={{
                variant: {
                    type: "abilityUsed",
                    player: myIndex,
                    abilityId: props.abilityId,
                    selection: props.saveData.selection
                },
                chatGroup: "all"
            }}/>
        } */}
        <SwitchSingleAbilityMenuType
            key={props.key}
            id={props.abilityId}
            available={props.saveData.availableAbilityData.available}
            selected={props.saveData.selection}
        />
    </>

    if(props.includeDropdown===true || props.includeDropdown===undefined){
        return <DetailsSummary
            className="generic-ability-menu"
            summary={
                <div className="generic-ability-menu-tab-summary">
                    <span><StyledText>{translateControllerID(props.abilityId)}</StyledText></span>
                    {nightIcon?<span>{translate("night.icon")}</span>:null}
                </div>
            }
            defaultOpen={true}
            disabled={props.saveData.availableAbilityData.grayedOut}
        >
            {inner}
        </DetailsSummary>
        
    }else{
        return <>
            <div className="generic-ability-menu generic-ability-menu-tab-no-summary">
                <span>
                    {
                        props.saveData.availableAbilityData.grayedOut === true ?
                        <Icon>close</Icon>
                        : null
                    }
                    <StyledText>{translateControllerID(props.abilityId)}</StyledText>
                </span>
                {nightIcon?<span>{translate("night.icon")}</span>:null}
            </div>
            {
                props.saveData.availableAbilityData.grayedOut === false ?
                <>{inner}</>
                : null
            }
        </>
    }
    
}


function SwitchSingleAbilityMenuType(props: Readonly<{
    key: number,
    id: ControllerID,
    available: AvailableAbilitySelection,
    selected: AbilitySelection
}>): ReactElement {

    const {key, id, available} = props;
    let selected: AbilitySelection = props.selected;

    switch(available.type) {
        case "unit":
            return <Button
                key={key}
                onClick={()=>{
                    GAME_MANAGER.sendAbilityInput({
                        id, 
                        selection: {type: "unit"}
                    });
                }}
            >
                {translateControllerID(props.id)}
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
        case "threePlayerOption":{
            let input: ThreePlayerOptionSelection;
            if(
                props.selected === null ||
                props.selected.type !== "threePlayerOption"
            ){
                input = [null, null, null];
            }else{
                input = props.selected.selection;
            }

            return <ThreePlayerOptionSelectionMenu
                key={props.key}
                selection={input}
                availableSelection={available.selection}
                onChoose={(selection) => {
                    GAME_MANAGER.sendAbilityInput({
                        id, 
                        selection: {
                            type: "threePlayerOption",
                            selection
                        }
                    });
                }}
            />;
        }
        case "roleOption":{
            let input: RoleOptionSelection;
            if(
                props.selected === null ||
                props.selected.type !== "roleOption"
            ){
                input = null;
            }else{
                input = props.selected.selection;
            }

            return <RoleOptionSelectionMenu
                selection={input}
                enabledRoles={available.selection}
                onChoose={(selection) => {
                    GAME_MANAGER.sendAbilityInput({
                        id, 
                        selection: {
                            type: "roleOption",
                            selection
                        }
                    });
                }}
            />
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
            let input: TwoRoleOutlineOptionSelection;
            if(
                props.selected === null ||
                props.selected.type !== "twoRoleOutlineOption"
            ){
                input = [null, null];
            }else{
                input = props.selected.selection;
            }

            return <TwoRoleOutlineOptionSelectionMenu
                selection={input}
                available={available.selection}
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
        case "string":{
            let input: StringSelection;
            if(
                props.selected === null ||
                props.selected.type !== "string"
            ){
                input = "";
            }else{
                input = props.selected.selection;
            }

            return <StringSelectionMenu
                id={id}
                selection={input}
                key={props.key}
                onChoose={(selection) => {
                    GAME_MANAGER.sendAbilityInput({
                        id,
                        selection: {
                            type: "string",
                            selection: selection
                        }
                    });
                }}
            />
        }
        case "kira":{
            let input: KiraSelection;
            if(
                props.selected === null ||
                props.selected.type !== "kira"
            ){
                input = [];
            }else{
                input = props.selected.selection;
            }

            return <KiraSelectionMenu
                key={props.key}
                selection={input}
                available={available.selection}
                onChange={(selection)=>{
                    GAME_MANAGER.sendAbilityInput({
                        id,
                        selection: {
                            type: "kira",
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
