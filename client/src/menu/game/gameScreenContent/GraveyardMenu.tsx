import React from "react";
import translate, { styleText } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameState, { RoleListEntry, getAlignmentStringFromFactionAlignment, getFactionFromFactionAlignment } from "../../../game/gameState.d";
import { Grave } from "../../../game/grave";
import GameScreen, { ContentMenus } from "../GameScreen";
import "./graveyardMenu.css";

type GraveyardMenuProps = {
}
type GraveyardMenuState = {
    gameState: GameState,
}

export default class GraveyardMenu extends React.Component<GraveyardMenuProps, GraveyardMenuState> {
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
            graveRoleString = translate(`role.${grave.role.role}.name`);
        } else {
            graveRoleString = translate(`grave.role.${grave.role.type}`);
        }

        // return(<div key={graveIndex}>
        //     {grave.diedPhase.toString()} {grave.dayNumber}<br/>
        //     {this.state.gameState.players[grave.playerIndex]?.toString()}<br/>
        //     {"("+graveRoleString+")"} killed by {deathCauseString}
        // </div>)
        return(<button key={graveIndex}>
            {this.state.gameState.players[grave.playerIndex]?.toString()} {styleText("("+graveRoleString+")")}
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
        switch(roleListEntry.type){
            case "any":
                return <button key={index}>{styleText(translate("any"))}</button>
            case "faction":
                return <button key={index}>
                    {styleText(translate("faction."+roleListEntry.faction.toString()))} {styleText(translate("any"))}
                </button>
            case "factionAlignment":
                return <button key={index}>
                    {styleText(translate("faction."+getFactionFromFactionAlignment(roleListEntry.factionAlignment)))} {styleText(translate("alignment."+getAlignmentStringFromFactionAlignment(roleListEntry.factionAlignment)))}
                </button>
            case "exact":
                return <button key={index}>{styleText(translate("role."+roleListEntry.role+".name"))}</button>
        }
    }
    render(){return(<div className="graveyard-menu">
        <button onClick={()=>{
            GameScreen.instance.closeMenu(ContentMenus.GraveyardMenu)
        }}>{styleText(translate("menu.graveyard.title"))}</button>
        <div>
            {this.renderRoleList()}
            {this.renderGraves()}
        </div>        
    </div>)}
}