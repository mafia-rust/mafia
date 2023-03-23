import React from "react";
import { getPlayerString, translate } from "../game/lang";
import gameManager from "../index";

export class GraveyardMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
            expandedGraves: [],    //list of graveIndexs of what graves should be showing its will 
        };
        this.listener = ()=>{
            this.setState({
                gameState: gameManager.gameState
            })
        };  
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }

    renderGrave(grave, graveIndex){
        return(<div key={graveIndex}>
            {grave.diedPhase} {grave.dayNumber}<br/>
            {grave.playerIndex+1}:{this.state.gameState.players[grave.playerIndex]}<br/>
            {grave.role} killed by {(()=>{
                if(grave.death_cause === "Lynching"){
                    return <div>{"a lynching."}</div>
                }
                return <div>{grave.death_cause.Killers.killers.join(", ") + "."}</div>
            })()}
            <button onClick={()=>{
                if(this.state.expandedGraves.includes(graveIndex)){
                    this.state.expandedGraves.splice(this.state.expandedGraves.indexOf(graveIndex));
                }else{
                    this.state.expandedGraves.push(graveIndex);
                }  
            }}>Expand</button>
            {(()=>{if(this.state.expandedGraves.includes(graveIndex))return this.renderExtendedGrave(grave, graveIndex)})()}
        </div>)
    }
    renderExtendedGrave(grave, graveIndex){
        return(<div>{grave.will}</div>);
    }

    renderRoleList(){
        return<div>
            {
                this.state.gameState.roleList.map((entry, index)=>{
                    return this.renderRoleListEntry(entry, index)
                }, this)
            }
        </div>
    }
    renderRoleListEntry(roleListEntry, index){
        if(roleListEntry==="Any"){
            return <div key={index}>
                <button>{translate("FactionAlignment.Faction.Random")}</button>
            </div>
        }
        if(roleListEntry.Exact !== undefined){
            let role = roleListEntry.Exact.Role;
            return <div key={index}>
                <button>{translate("role."+role+".name")}</button>
            </div>
        }
        if(roleListEntry.FactionAlignment !== undefined){
            let factionAlignment = roleListEntry.FactionAlignment;
            
            //get faction and alignment strings seperated
            let faction = null;
            let alignment = null;
            
            
            if(factionAlignment.includes("Town")) faction = "Town";
            if(factionAlignment.includes("Mafia")) faction = "Mafia";
            if(factionAlignment.includes("Neutral")) faction = "Neutral";
            if(factionAlignment.includes("Coven")) faction = "Coven";

            if(factionAlignment.includes("Killing")) alignment = "Killing";
            if(factionAlignment.includes("Investigative")) alignment = "Investigative";
            if(factionAlignment.includes("Protective")) alignment = "Protective";
            if(factionAlignment.includes("Support")) alignment = "Support";
            if(factionAlignment.includes("Deception")) alignment = "Deception";
            if(factionAlignment.includes("Evil")) alignment = "Evil";
            if(factionAlignment.includes("Chaos")) alignment = "Chaos";

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
        {getPlayerString(gameManager.gameState.myIndex)}: {this.state.gameState.role}
        {/* {this.state.gameState.graves.map((grave, graveIndex)=>{
            return this.renderGrave(grave, graveIndex);
        }, this)} */}
        {this.renderRoleList()}
    </div>)}
}