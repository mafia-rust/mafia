import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameState, { Grave, RoleListEntry } from "../../../game/gameState.d";

interface GraveyardMenuState {
    gameState: GameState,
}

export default class GraveyardMenu extends React.Component<any, GraveyardMenuState> {
    listener: () => void;
    constructor(props: any) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
        };
        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState
            })
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    renderGrave(grave: Grave, graveIndex: number){
        let deathCauseString: string;
        if(grave.deathCause.type === "lynching"){
            deathCauseString = "a lynching.";
        } else  {
            deathCauseString = grave.deathCause.killers.map((killer)=>{
                return killer.type === "role" ? killer.role : killer.type;
            }).join() + ".";
        }

        let graveRoleString: string;
        if (grave.role.type === "role") {
            graveRoleString = grave.role.role;
        } else {
            graveRoleString = grave.role.type;
        }

        return(<div key={graveIndex}>
            {grave.diedPhase.toString()} {grave.dayNumber}<br/>
            {this.state.gameState.players[grave.playerIndex]?.toString()}<br/>
            {graveRoleString} killed by {deathCauseString}
        </div>)
    }

    renderRoleList(){return<div>
        {
            this.state.gameState.roleList.map((entry, index)=>{
                return this.renderRoleListEntry(entry, index)
            }, this)
        }
    </div>}
    renderRoleListEntry(roleListEntry: RoleListEntry, index: number){
        if(roleListEntry.type === "any"){
            return <div key={index}>
                <button>{translate("FactionAlignment.Faction.Random")}</button>
            </div>
        } else if(roleListEntry.type === "exact"){
            let role = roleListEntry.role;
            return <div key={index}>
                <button>{translate("role."+role+".name")}</button>
            </div>
        } else if(roleListEntry.type === "factionAlignment"){
            let factionAlignment = roleListEntry.factionAlignment;
            
            let faction = roleListEntry.faction;
            let alignment = factionAlignment.replace(faction, "");

            return <div key={index}>
                <button>{translate("FactionAlignment.Faction."+faction)} {translate("FactionAlignment.Alignment."+alignment)}</button>
            </div>
        } else {
            let faction = roleListEntry.faction;
            return <div key={index}>
                <button>{translate("FactionAlignment.Faction."+faction)} {translate("FactionAlignment.Alignment.Random")}</button>
            </div>
        }
    }
    render(){return(<div>
        {this.state.gameState.players[this.state.gameState.myIndex!]?.toString()}: {this.state.gameState.role}
        {this.state.gameState.graves.map((grave, graveIndex)=>{
            return this.renderGrave(grave, graveIndex);
        }, this)}
        {this.renderRoleList()}
    </div>)}
}