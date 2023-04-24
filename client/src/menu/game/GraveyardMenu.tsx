import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import GameState, { Grave, RoleListEntry } from "../../game/gameState.d";

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
        return(<div key={graveIndex}>
            {grave.diedPhase} {grave.dayNumber}<br/>
            {grave.playerIndex+1}:{this.state.gameState.players[grave.playerIndex]?.toString()}<br/>
            {grave.role} killed by {(()=>{
                if(grave.deathCause === "Lynching"){
                    return <div>{"a lynching."}</div>
                }
                return <div>{grave.deathCause?.join() + "."}</div>
            })()}
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
        if(roleListEntry==="Any"){
            return <div key={index}>
                <button>{translate("FactionAlignment.Faction.Random")}</button>
            </div>
        }
        if(roleListEntry.Exact !== undefined){
            let role = roleListEntry.Exact.role;
            return <div key={index}>
                <button>{translate("role."+role+".name")}</button>
            </div>
        }
        if(roleListEntry.FactionAlignment !== undefined){
            let factionAlignment = roleListEntry.FactionAlignment.faction_alignment;
            
            let faction = roleListEntry.FactionAlignment.faction;
            let alignment = factionAlignment.replace(faction, "");

            return <div key={index}>
                <button>{translate("FactionAlignment.Faction."+faction)} {translate("FactionAlignment.Alignment."+alignment)}</button>
            </div>
        }
        if(roleListEntry.Faction !== undefined){
            let faction = roleListEntry.Faction;
            return <div key={index}>
                <button>{translate("FactionAlignment.Faction."+faction)} {translate("FactionAlignment.Alignment.Random")}</button>
            </div>
        }
        console.log("uncaught rolelistentry type: "+roleListEntry);
        return null;
    }
    render(){return(<div>
        {this.state.gameState.players[this.state.gameState.myIndex!]?.toString()}: {this.state.gameState.role}
        {this.state.gameState.graves.map((grave, graveIndex)=>{
            return this.renderGrave(grave, graveIndex);
        }, this)}
        {this.renderRoleList()}
    </div>)}
}