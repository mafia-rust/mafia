import React from "react";
import translate, { styleText } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameState, { RoleListEntry } from "../../../game/gameState.d";
import { Grave } from "../../../game/grave";
import GameScreen, { ContentMenus } from "../GameScreen";

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

    renderGraves(){
        return <div>
            {this.state.gameState.graves.map((grave, graveIndex)=>{
                return this.renderGrave(grave, graveIndex);
            }, this)}
        </div>
    }
    renderGrave(grave: Grave, graveIndex: number){
        // let deathCauseString: string;
        // if(grave.deathCause.type === "lynching"){
        //     deathCauseString = "a lynching.";
        // } else  {
        //     deathCauseString = grave.deathCause.killers.map((killer)=>{
        //         return killer.type === "role" ? killer.role : killer.type;
        //     }).join() + ".";
        // }

        let graveRoleString: string;
        if (grave.role.type === "role") {
            graveRoleString = grave.role.role;
        } else {
            graveRoleString = grave.role.type;
        }

        // return(<div key={graveIndex}>
        //     {grave.diedPhase.toString()} {grave.dayNumber}<br/>
        //     {this.state.gameState.players[grave.playerIndex]?.toString()}<br/>
        //     {"("+graveRoleString+")"} killed by {deathCauseString}
        // </div>)
        return(<button key={graveIndex}>
            {this.state.gameState.players[grave.playerIndex]?.toString()}<br/>
            {styleText("("+graveRoleString+")")}
        </button>);
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
                <button>{styleText(translate("FactionAlignment.Faction.Random"))}</button>
            </div>
        } else if(roleListEntry.type === "exact"){
            let role = roleListEntry.role;
            return <div key={index}>
                <button>{styleText(translate("role."+role+".name"))}</button>
            </div>
        } else if(roleListEntry.type === "factionAlignment"){
            let factionAlignment = roleListEntry.factionAlignment;
            
            let faction = roleListEntry.faction;
            let alignment = factionAlignment.replace(faction, "");

            return <div key={index}>
                <button>{styleText(translate("FactionAlignment.Faction."+faction))}<br/>{styleText(translate("FactionAlignment.Alignment."+alignment))}</button>
            </div>
        } else {
            let faction = roleListEntry.faction;
            return <div key={index}>
                <button>{styleText(translate("FactionAlignment.Faction."+faction))}<br/>{styleText(translate("FactionAlignment.Alignment.Random"))}</button>
            </div>
        }
    }
    render(){return(<div>
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.GraveyardMenu)}}>{styleText(translate("menu.graveyard.title"))}</button>
        <br/>
        <div style={{display: "flex"}}>
            {this.renderRoleList()}
            {this.renderGraves()}
        </div>        
    </div>)}
}