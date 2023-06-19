import React from "react";
import translate, { styleText } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { Grave } from "../../../game/grave";
import GameScreen, { ContentMenus } from "../GameScreen";
import "./graveyardMenu.css";
import { renderRoleListEntry } from "../../../game/gameState.d";

type GraveyardMenuProps = {
}
type GraveyardMenuState = {
    graves: Grave[],
    extendedGraveIndex: number | null
}

export default class GraveyardMenu extends React.Component<GraveyardMenuProps, GraveyardMenuState> {
    listener: () => void;
    constructor(props: GraveyardMenuProps) {
        super(props);

        this.state = {
            graves: GAME_MANAGER.gameState.graves,
            extendedGraveIndex: null
        };
        this.listener = ()=>{
            this.setState({
                graves: GAME_MANAGER.gameState.graves
            })
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener("addGrave", this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener("addGrave", this.listener);
    }

    renderGraves(){
        return <div>
            {this.state.graves.map((grave, graveIndex)=>{
                return this.renderGrave(grave, graveIndex);
            }, this)}
        </div>
    }
    renderGrave(grave: Grave, graveIndex: number){
        let graveRoleString: string;
        if (grave.role.type === "role") {
            graveRoleString = translate(`role.${grave.role.role}.name`);
        } else {
            graveRoleString = translate(`grave.role.${grave.role.type}`);
        }

        return(<button key={graveIndex} onClick={()=>{this.setState({extendedGraveIndex:graveIndex})}}>
            {GAME_MANAGER.gameState.players[grave.playerIndex]?.toString()} {styleText("("+graveRoleString+")")}
        </button>);
    }
    renderGraveExtended(grave: Grave){
        let deathCauseString: string;
        if(grave.deathCause.type === "lynching"){
            deathCauseString = translate("grave.deathCause.lynching");
        } else  {
            deathCauseString = grave.deathCause.killers.map((killer)=>{
                switch(killer.type){
                    case "role":
                        return translate("role."+killer.value+".name");
                    case "faction":
                        return translate("faction."+killer.value);
                    default:
                        return translate(killer.type);
                }
            }).join() + ".";
        }

        let graveRoleString: string;
        if (grave.role.type === "role") {
            graveRoleString = translate(`role.${grave.role.role}.name`);
        } else {
            graveRoleString = translate(`grave.role.${grave.role.type}`);
        }

        let diedPhaseString = grave.diedPhase === "day" ? translate("day") : translate("phase.night");
        return(<button onClick={()=>{this.setState({extendedGraveIndex:null})}}>
            {styleText(diedPhaseString+" "+grave.dayNumber)} <br/>
            {styleText(GAME_MANAGER.gameState.players[grave.playerIndex]?.toString()+" ("+graveRoleString+")")}<br/>
            {styleText(translate("menu.graveyard.killedBy")+" "+deathCauseString)}<br/>
            <div className="will-area">{styleText(grave.will)}</div>
        </button>);
    }

    renderRoleList(){return<div>
        {GAME_MANAGER.gameState.roleList.map((entry, index)=>{
            return <button key={index}>{renderRoleListEntry(entry)}</button>
        }, this)}
    </div>}
    render(){return(<div className="graveyard-menu">
        <button onClick={()=>{
            GameScreen.instance.closeMenu(ContentMenus.GraveyardMenu)
        }}>{styleText(translate("menu.graveyard.title"))}</button>
        <div>
            {this.renderRoleList()}
            {this.renderGraves()}
            {/* //TODO show excluded roles at top */}
        </div>
        <div>
            {this.state.extendedGraveIndex!==null?this.renderGraveExtended(GAME_MANAGER.gameState.graves[this.state.extendedGraveIndex]):null}
        </div>
    </div>)}
}