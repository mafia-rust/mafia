import React, { ReactElement } from "react";
import GAME_MANAGER from "../../index";
import "../../index.css";
import GameState from "../../game/gameState.d";

const ROLES: ReadonlyMap<string, any> = new Map(Object.entries(require("../../resources/roles.json")));

interface RolePaneState {
    roleList: RoleListEntry[]
}

export default class LobbyRolePane extends React.Component<any, RolePaneState> {
    listener: (type: any) => void;

    constructor(props: any){
        super(props);

        this.state = {
            roleList: []
        }

        this.listener = (type: any) => {
            this.setState({
                roleList: [...GAME_MANAGER.gameState.roleList],
            });
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render(){return(<div>
        Role List
        {this.renderList()}
    </div>);}

    renderList(){
        return <div>
            {this.state.roleList.map((roleListEntry, index) => {
                return <RolePicker
                    roleListEntry={roleListEntry}
                    index={index}
                />
            })}
        </div>
    }

    onChangeRolePicker(index: number, value: RoleListEntry){
        let roleList = [...this.state.roleList];
        roleList[index] = value;

        GAME_MANAGER.sendSetRoleListPacket(this.state.roleList);
    }
}

interface RolePickerProps {
    roleListEntry: RoleListEntry,
    index: number
}

interface RolePickerState {
    roleListEntry: RoleListEntry
}

class RolePicker extends React.Component<RolePickerProps, RolePickerState> {
    constructor(props: RolePickerProps) {
        super(props);

        console.log("Role picker " + props.index + ": " + props.roleListEntry);

        this.state = {
            roleListEntry: props.roleListEntry,
        };
    }

    allRoles(faction: string, alignment: string) {
        let roles = [];

        for (let role in ROLES) {
            if (ROLES.get(role).faction !== faction) continue;
            if (ROLES.get(role).alignment !== alignment && alignment !== "Random") continue;

            roles.push(role);
        }

        return roles;
    }

    render() {
        let selectors: JSX.Element[];
        if (this.state.roleListEntry === "Any"){
            selectors = [
                <select value="Any"> {
                    allFactions().map((faction: string) => {
                        return <option> {faction} </option>
                    })
                } </select>
            ];
        } else if (this.state.roleListEntry.Faction !== undefined) {
            selectors = [
                <select value={getFaction(this.state.roleListEntry)}> {
                    allFactions().map((faction: string) => {
                        return <option> {faction} </option>
                    })
                } </select>
            ]
        } else if (this.state.roleListEntry.FactionAlignment !== undefined) {
            let faction = getFaction(this.state.roleListEntry);
            selectors = [
                <select value={faction}> {
                    allFactions().map((faction: string) => {
                        return <option> {faction} </option>
                    })
                } </select>,
                <select value={getAlignment(this.state.roleListEntry)}> {
                    allAlignments(faction).map((faction: string) => {
                        return <option> {faction} </option>
                    })
                } </select>
            ]
        } else {
            let faction = getFaction(this.state.roleListEntry);
            let alignment = getAlignment(this.state.roleListEntry);
            selectors = [
                <select value={faction}> {
                    allFactions().map((faction: string) => {
                        return <option> {faction} </option>
                    })
                } </select>,
                <select value={alignment}> {
                    allAlignments(faction).map((alignment: string) => {
                        return <option> {alignment} </option>
                    })
                } </select>,
                <select value={this.state.roleListEntry.Exact?.role}> {
                    this.allRoles(faction, alignment).map((faction: string) => {
                        return <option> {faction} </option>
                    })
                } </select>
            ]
        }

        return <div>
            {selectors}
        </div>
    }
}

export type RoleListEntry = "Any" | {
    "Exact"?: {
        role: string,
        faction_alignment: string,
        faction: string,
    },
    "FactionAlignment"?: {
        faction_alignment: string,
        faction: string,
    },
    "Faction"?: {
        faction: string,
    }
};

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
    } else if (roleListEntry.Faction === undefined) {
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
    for (let role in ROLES) {
        if (!factions.includes(ROLES.get(role).faction)) {
            factions.push(ROLES.get(role).faction);
        }
    }
    return factions;
}

function allAlignments(faction: string) {
    let alignments: string[] = [];
    let roles: string[] = [];

    for (let role in ROLES) {
        if (ROLES.get(role).faction !== faction) continue;

        if (!alignments.includes(ROLES.get(role).alignment)) {
            alignments.push(ROLES.get(role).alignment);
        }
    }

    return alignments.concat(roles);
}