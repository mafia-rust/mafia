import React from "react";
import "./rolePicker.css";
import { 

} from "../game/gameState.d";
import translate from "../game/lang";
import ROLES from "../resources/roles.json";
import { FACTIONS, Faction, FactionAlignment, RoleOutline, getAlignmentStringFromFactionAlignment, getAllFactionAlignments, getFactionAlignmentFromRoleOutline, getFactionFromFactionAlignment, getFactionFromRoleOutline } from "../game/roleListState.d";
import { Role, getFactionAlignmentFromRole, getFactionFromRole } from "../game/roleState.d";

type RolePickerProps = {
    roleOutline: RoleOutline,
    onChange: (value: RoleOutline) => void,
    disabled?: boolean,
}

export default class RolePicker extends React.Component<RolePickerProps> {
    setAny(){
        this.props.onChange({
            type: "any"
        })
    }
    setFaction(faction: Faction){
        this.props.onChange({
            type: "faction",
            faction: faction
        })
    }
    setFactionAlignment(factionAlignment: FactionAlignment){
        this.props.onChange({
            type: "factionAlignment",
            factionAlignment: factionAlignment
        })
    }
    setExact(role: Role){
        this.props.onChange({
            type: "exact",
            role: role
        })
    }

    setFirstBox(e: { target: { selectedIndex: number; }; }){
        let selected = allFactionsAndAny()[e.target.selectedIndex];

        if(selected === "any"){
            this.setAny();
        } else {
            this.setFaction(selected);
        }
    }
    setSecondBox(e: { target: { selectedIndex: number; }; }){
        let currentFaction = getFactionFromRoleOutline(this.props.roleOutline);
        if(currentFaction === null)
            return;
        
        let selected = allFactionAlignmentsAndAny(currentFaction)[e.target.selectedIndex];

        if(selected === "any"){
            this.setFaction(currentFaction);
        } else {
            this.setFactionAlignment(selected);
        }
    }
    setThirdBox(e: { target: { selectedIndex: number; }; }){
        let currentFactionAlignment = getFactionAlignmentFromRoleOutline(this.props.roleOutline);
        if(currentFactionAlignment === null)
            return;

        let selected = allRolesAndAny(currentFactionAlignment)[e.target.selectedIndex];

        if(selected === "any"){
            this.setFactionAlignment(currentFactionAlignment);
        } else {
            this.setExact(selected);
        }
    }
    
    render() {
        let selectors: JSX.Element[] = [];
        
        switch(this.props.roleOutline.type){

            case "any":
                selectors = [
                    <select 
                        disabled={this.props.disabled}
                        key="faction" 
                        value={translate("any")}
                        onChange={(e)=>this.setFirstBox(e)}
                    > {
                        allFactionsAndAny().map((faction: Faction | "any", key) => {
                            if(faction === "any")
                                return <option key={key}>{translate("any")}</option>
                            return <option key={key}>{translate("faction."+faction)}</option>
                        })
                    } </select>
                ];
            break;
            case "faction":
                selectors = [
                    <select 
                        disabled={this.props.disabled}
                        key="faction" 
                        value={translate("faction."+this.props.roleOutline.faction)}
                        onChange={(e)=>this.setFirstBox(e)}
                    > {
                        allFactionsAndAny().map((faction: Faction | "any", key) => {
                            if(faction === "any")
                                return <option key={key}>{translate("any")}</option>
                            return <option key={key}>{translate("faction."+faction)}</option>
                        })
                    } </select>,
                    
                    <select
                        disabled={this.props.disabled}
                        key="alignment"
                        value={translate("any")}
                        onChange={(e)=>this.setSecondBox(e)}
                    > {
                        allFactionAlignmentsAndAny(this.props.roleOutline.faction).map((factionAlignment: FactionAlignment | "any", key) => {
                            if(factionAlignment === "any")
                                return <option key={key}>{translate("any")}</option>
                            return <option key={key}>{translate("alignment."+getAlignmentStringFromFactionAlignment(factionAlignment))}</option>
                        })
                    } </select>
                ]
            break;
            case "factionAlignment":
                selectors = [
                    <select
                        disabled={this.props.disabled}
                        key="faction" 
                        value={translate("faction."+getFactionFromFactionAlignment(this.props.roleOutline.factionAlignment))}
                        onChange={(e)=>this.setFirstBox(e)}
                    > {
                        allFactionsAndAny().map((faction: string, key) => {
                            if(faction === "any")
                                return <option key={key}>{translate("any")}</option>
                            return <option key={key}>{translate("faction."+faction)}</option>
                        })
                    } </select>,

                    <select
                        disabled={this.props.disabled}
                        key="alignment"
                        value={translate("alignment."+getAlignmentStringFromFactionAlignment(this.props.roleOutline.factionAlignment))}
                        onChange={(e)=>this.setSecondBox(e)}
                    > {
                        allFactionAlignmentsAndAny(getFactionFromFactionAlignment(this.props.roleOutline.factionAlignment)).map((factionAlignment: string, key) => {
                            if(factionAlignment === "any")
                                return <option key={key}>{translate("any")}</option>
                            return <option key={key}>{translate("alignment."+getAlignmentStringFromFactionAlignment(factionAlignment as FactionAlignment))}</option>
                        })
                    } </select>,
                    <select
                        disabled={this.props.disabled}
                        key="exact"
                        value={translate("any")}
                        onChange={(e)=>this.setThirdBox(e)}
                    > {
                        allRolesAndAny(this.props.roleOutline.factionAlignment).map((role: string, key) => {
                            if(role === "any")
                                return <option key={key}>{translate("any")}</option>
                            return <option key={key}>{translate(`role.${role}.name`)}</option>
                        })
                    } </select>
                ]
            break;
            case "exact":
                selectors = [
                    <select
                        disabled={this.props.disabled}
                        key="faction" 
                        value={translate("faction."+getFactionFromRole(this.props.roleOutline.role))}
                        onChange={(e)=>this.setFirstBox(e)}
                    > {
                        allFactionsAndAny().map((faction: string, key) => {
                            if(faction === "any")
                                return <option key={key}>{translate("any")}</option>
                            return <option key={key}>{translate("faction."+faction)}</option>
                        })
                    } </select>,

                    <select
                        disabled={this.props.disabled}
                        key="alignment"
                        value={translate("alignment."+getAlignmentStringFromFactionAlignment(getFactionAlignmentFromRole(this.props.roleOutline.role)))}
                        onChange={(e)=>this.setSecondBox(e)}
                    > {
                        allFactionAlignmentsAndAny(getFactionFromRole(this.props.roleOutline.role)).map((factionAlignment: string, key) => {
                            if(factionAlignment === "any")
                                return <option key={key}>{translate("any")}</option>
                            return <option key={key}>{translate("alignment."+getAlignmentStringFromFactionAlignment(factionAlignment as FactionAlignment))}</option>
                        })
                    } </select>,
                    <select
                        disabled={this.props.disabled}
                        key="exact"
                        value={translate(`role.${this.props.roleOutline.role}.name`)}
                        onChange={(e)=>this.setThirdBox(e)}
                    > {
                        allRolesAndAny(getFactionAlignmentFromRole(this.props.roleOutline.role)).map((role: string, key) => {
                            if(role === "any")
                                return <option key={key}>{translate("any")}</option>
                            return <option key={key}>{translate(`role.${role}.name`)}</option>
                        })
                    } </select>
                ]
            break;
        }
        
        return <div className="role-picker">
            {selectors}
        </div>
    }
}

function allFactionsAndAny(): (Faction | "any")[] {
    return ["any" as (Faction | "any")].concat(FACTIONS);
}

function allFactionAlignmentsAndAny(faction: Faction): (FactionAlignment | "any")[] {
    return ["any" as (FactionAlignment | "any")].concat(getAllFactionAlignments(faction.toLowerCase() as Faction));
}

function allRolesAndAny(factionAlignment: FactionAlignment): (Role | "any")[] {
    let roles: (Role | "any")[] = ["any"];

    for(let role of Object.keys(ROLES)){
        if(getFactionAlignmentFromRole(role as Role) === factionAlignment)
            roles.push(role as Role);
    }
    

    return roles;
}