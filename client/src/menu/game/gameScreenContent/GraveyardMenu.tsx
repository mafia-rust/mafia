import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { Grave } from "../../../game/grave";
import { ContentMenus, ContentTab } from "../GameScreen";
import "./graveyardMenu.css";
import GameState from "../../../game/gameState.d";
import { translateRoleListEntry } from "../../../game/roleListState.d";
import StyledText from "../../../components/StyledText";
import WikiMenu from "./WikiMenu";

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
        return <>
            {this.state.gameState.graves.map((grave, graveIndex)=>{
                return this.renderGrave(grave, graveIndex);
            }, this)}
        </>
    }
    renderGrave(grave: Grave, graveIndex: number){
        let graveRoleString: string;
        if (grave.role.type === "role") {
            graveRoleString = translate(`role.${grave.role.role}.name`);
        } else {
            graveRoleString = translate(`grave.role.${grave.role.type}`);
        }

        return(<button 
            style={{ gridRow: graveIndex + 1, gridColumn:0 }} 
            key={graveIndex} 
            onClick={()=>{this.setState({extendedGraveIndex:graveIndex})}}
        >
            {this.state.gameState.players[grave.playerIndex]?.toString()}
            <StyledText>
                {`(${graveRoleString})`}
            </StyledText>
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
        return(<button className="grave" onClick={()=>{this.setState({extendedGraveIndex:null})}}>
            <StyledText>{`
${diedPhaseString+" "+grave.dayNumber}

${this.state.gameState.players[grave.playerIndex]?.toString()+" ("+graveRoleString+")"}

${translate("menu.graveyard.killedBy")+" "+deathCauseString}
            `}</StyledText>
            {grave.will.length === 0 || <>
                {translate("grave.will")}
                <div className="note-area">
                    <StyledText>
                        {grave.will}
                    </StyledText>
                </div>
            </>}
            {grave.deathNotes.length === 0 || grave.deathNotes.map(note => <>
                {translate("grave.deathNote")}
                <div className="note-area">
                    <StyledText>
                        {note}
                    </StyledText>
                </div>
            </>)}
        </button>);
    }

    renderRoleList(){return<>
        {this.state.gameState.roleList.map((entry, index)=>{
            return <button 
                style={{ gridRow: index + 1, gridColumn:1 }} 
                key={index}
                onClick={() => {
                    if (entry.type === "exact") {
                        WikiMenu.setPage(`role/${entry.role}`);
                    }
                }}
            >
                <StyledText>
                    {translateRoleListEntry(entry) ?? ""}
                </StyledText>
            </button>
        }, this)}
    </>}

    renderExcludedRoles(){
        return<div className="graveyard-menu-excludedRoles">
            <section>
                {translate("menu.excludedRoles.excludedRoles")}
            </section>
            <div>
                {this.state.gameState.excludedRoles.length === 0 
                    ? <StyledText>{translate("none")}</StyledText>
                    : this.state.gameState.excludedRoles.map((value, i)=>{
                    return <button key={i}>
                        <StyledText>
                            {translateRoleListEntry(value) ?? ""}
                        </StyledText>
                    </button>
                })}
            </div>
        </div>
    }


    render(){return(<div className="graveyard-menu">
        <ContentTab close={ContentMenus.GraveyardMenu}>{translate("menu.graveyard.title")}</ContentTab>
            
        <div className="grid">
            {this.renderRoleList()}
            {this.renderGraves()}
        </div>
        {this.renderExcludedRoles()}

        <div>
            {this.state.extendedGraveIndex!==null?this.renderGraveExtended(this.state.gameState.graves[this.state.extendedGraveIndex]):null}
        </div>
    </div>)}
}