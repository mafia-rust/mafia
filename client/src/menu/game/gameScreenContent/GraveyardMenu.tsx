import React from "react";
import translate, { styleText } from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { Grave } from "../../../game/grave";
import GameScreen, { ContentMenus } from "../GameScreen";
import "./graveyardMenu.css";
import GameState from "../../../game/gameState.d";
import { translateRoleListEntry } from "../../../game/roleListState.d";

type GraveyardMenuProps = {
}
type GraveyardMenuState = {
    gameState: GameState,
    extendedGraveIndex: number | null
}

export default class GraveyardMenu extends React.Component<GraveyardMenuProps, GraveyardMenuState> {
    listener: () => void;
    constructor(props: GraveyardMenuProps) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            extendedGraveIndex: null
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
        let graveRoleString: string;
        if (grave.role.type === "role") {
            graveRoleString = translate(`role.${grave.role.role}.name`);
        } else {
            graveRoleString = translate(`grave.role.${grave.role.type}`);
        }

        return(<button key={graveIndex} onClick={()=>{this.setState({extendedGraveIndex:graveIndex})}}>
            {this.state.gameState.players[grave.playerIndex]?.toString()} {styleText("("+graveRoleString+")")}
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
            {styleText(this.state.gameState.players[grave.playerIndex]?.toString()+" ("+graveRoleString+")")}<br/>
            {styleText(translate("menu.graveyard.killedBy")+" "+deathCauseString)}<br/>
            <div className="will-area">{styleText(grave.will)}</div>
        </button>);
    }

    renderRoleList(){return<div>
        {this.state.gameState.roleList.map((entry, index)=>{
            return <button key={index}>{translateRoleListEntry(entry)}</button>
        }, this)}
    </div>}

    renderExcludedRoles(){
        return<div className="graveyard-menu-excludedRoles">
            <section>
                {translate("menu.excludedRoles.excludedRoles")}
            </section>
            <div>
                {this.state.gameState.excludedRoles.map((value, i)=>{
                    return <button key={i}>
                        {translateRoleListEntry(value)}
                    </button>
                })}
            </div>
        </div>
    }


    render(){return(<div className="graveyard-menu">
        <button onClick={()=>{
            GameScreen.instance.closeMenu(ContentMenus.GraveyardMenu)
        }}>{styleText(translate("menu.graveyard.title"))}</button>
        <div>
            {this.renderRoleList()}
            {this.renderGraves()}
            {/* //TODO show excluded roles at top */}
        </div>
        {this.renderExcludedRoles()}

        
        


        <div>
            {this.state.extendedGraveIndex!==null?this.renderGraveExtended(this.state.gameState.graves[this.state.extendedGraveIndex]):null}
        </div>
    </div>)}
}