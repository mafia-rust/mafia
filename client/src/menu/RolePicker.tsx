import React from "react";
import { RoleListEntry } from "../game/gameState.d";
import { ROLES } from "../game/gameState";

interface RolePickerProps {
    roleListEntry: RoleListEntry,
    onChange: (value: RoleListEntry) => void
}

export default class RolePicker extends React.Component<RolePickerProps> {
    constructor(props: RolePickerProps) {
        super(props);
    }

    render() {
        let selectors: JSX.Element[] = [];
        if (this.props.roleListEntry === "Any"){
            selectors = [
                <select 
                    key="faction" 
                    value="Any"
                    onChange={(e)=>{this.updateRolePicker("faction", e.target.value)}}
                > {
                    allFactions().map((faction: string, key) => {
                        return <option key={key}> {faction} </option>
                    })
                } </select>
            ];
        } else if (this.props.roleListEntry.Faction !== undefined) {
            let faction = getFaction(this.props.roleListEntry);
            selectors = [
                <select
                    key="faction" 
                    value={faction}
                    onChange={(e)=>{this.updateRolePicker("faction", e.target.value)}}
                > {
                    allFactions().map((faction: string, key) => {
                        return <option key={key}> {faction} </option>
                    })
                } </select>,
                <select
                    key="alignment"
                    value={"Random"}
                    onChange={(e)=>{this.updateRolePicker("alignment", e.target.value)}}
                > {
                    allAlignments(faction).map((faction: string, key) => {
                        return <option key={key}> {faction} </option>
                    })
                } </select>
            ]
        } else if (this.props.roleListEntry.FactionAlignment !== undefined) {
            let faction = getFaction(this.props.roleListEntry);
            let alignment = getAlignment(this.props.roleListEntry);
            selectors = [
                <select
                    key="faction" 
                    value={faction}
                    onChange={(e)=>{this.updateRolePicker("faction", e.target.value)}}
                > {
                    allFactions().map((faction: string, key) => {
                        return <option key={key}> {faction} </option>
                    })
                } </select>,
                <select
                    key="alignment"
                    value={alignment}
                    onChange={(e)=>{this.updateRolePicker("alignment", e.target.value)}}
                > {
                    allAlignments(faction).map((faction: string, key) => {
                        return <option key={key}> {faction} </option>
                    })
                } </select>,
                <select
                    key="exact"
                    value={"Random"}
                    onChange={(e)=>{this.updateRolePicker("exact", e.target.value)}}
                > {
                    allRoles(faction, alignment).map((faction: string, key) => {
                        return <option key={key}> {faction} </option>
                    })
                } </select>
            ]
        } else {
            let faction = getFaction(this.props.roleListEntry);
            let alignment = getAlignment(this.props.roleListEntry);
            selectors = [
                <select
                    key="faction"
                    value={faction}
                    onChange={(e)=>{this.updateRolePicker("faction", e.target.value)}}
                > {
                    allFactions().map((faction: string, key) => {
                        return <option key={key}> {faction} </option>
                    })
                } </select>,
                <select
                    key="alignment"
                    value={alignment}
                    onChange={(e)=>{this.updateRolePicker("alignment", e.target.value)}}
                > {
                    allAlignments(faction).map((alignment: string, key) => {
                        return <option key={key}> {alignment} </option>
                    })
                } </select>,
                <select
                    key="exact"
                    value={this.props.roleListEntry.Exact?.role}
                    onChange={(e)=>{this.updateRolePicker("exact", e.target.value)}}
                > {
                    allRoles(faction, alignment).map((faction: string, key) => {
                        return <option key={key}> {faction} </option>
                    })
                } </select>
            ]
        }
        return <div className="lm-role-picker-container">
            {selectors}
        </div>
    }

    updateRolePicker(selector: "faction" | "alignment" | "exact", value: string) {
        let roleListEntry = this.props.roleListEntry;
        switch (selector) {
            case "faction":
                if (value === "Any") {
                    roleListEntry = "Any"
                } else {
                    roleListEntry = {
                        Faction: {
                            faction: value
                        }
                    }
                }
            break;
            case "alignment":
                if (value === "Random") {
                    roleListEntry = {
                        Faction: {
                            faction: getFaction(this.props.roleListEntry)
                        }
                    }
                } else {
                    roleListEntry = {
                        FactionAlignment: {
                            faction: getFaction(this.props.roleListEntry),
                            faction_alignment: getFaction(this.props.roleListEntry) + value
                        }
                    }
                }
            break;
            case "exact":
                if (value === "Random") {
                    roleListEntry = {
                        FactionAlignment: {
                            faction: getFaction(this.props.roleListEntry),
                            faction_alignment: getFaction(this.props.roleListEntry) + getAlignment(this.props.roleListEntry)
                        }
                    }
                } else {
                    roleListEntry = {
                        Exact: {
                            faction: getFaction(this.props.roleListEntry),
                            faction_alignment: getFaction(this.props.roleListEntry) + getAlignment(this.props.roleListEntry),
                            role: value
                        }
                    }
                }
            break;
        }

        this.props.onChange(roleListEntry);
    }
}

function getFaction(roleListEntry: RoleListEntry): string {
    if (roleListEntry === "Any") {
        throw "Couldn't find a faction for Any"
    } else {
        return Object.entries(roleListEntry)[0][1].faction;
    }
}

function getAlignment(roleListEntry: RoleListEntry): string {
    if (roleListEntry === "Any") {
        throw "Couldn't find an alignment for Any"
    } else if (roleListEntry.Faction !== undefined) {
        throw "Couldn't find an alignment for " + roleListEntry.Faction;
    } else {
        let faction_alignment: string = (Object.entries(roleListEntry)[0][1] as any).faction_alignment;
        let faction: string = Object.entries(roleListEntry)[0][1].faction;
        let alignment = faction_alignment.replace(faction, "");
        return alignment;
    }
}

function allFactions(): string[] {
    let factions: string[] = [];
    for (let [_, role] of ROLES) {
        let faction = role.faction;
        if (!factions.includes(faction)) {
            factions.push(faction);
        }
    }
    factions.push("Any");
    return factions;
}

function allAlignments(faction: string): string[] {
    let alignments: string[] = [];

    for (let [_, role] of ROLES) {
        if (role.faction !== faction) continue;

        if (!alignments.includes(role.alignment)) {
            alignments.push(role.alignment);
        }
    }

    alignments.push("Random");
    return alignments;
}

function allRoles(faction: string, alignment: string): string[] {
    let roles = [];

    for (let [name, role] of ROLES) {
        if (role.faction !== faction) continue;
        if (role.alignment !== alignment && alignment !== "Random") continue;

        roles.push(name);
    }

    roles.push("Random");
    return roles;
}