import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenus, ContentTab } from "../GameScreen";
import "./graveyardMenu.css";
import GameState from "../../../game/gameState.d";
import { translateRoleOutline } from "../../../game/roleListState.d";
import StyledText from "../../../components/StyledText";
import WikiSearch from "../../../components/WikiSearch";
import GraveComponent from "../../../components/grave";
import { Grave } from "../../../game/graveState";

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

        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                gameState : GAME_MANAGER.state,
                extendedGraveIndex: null
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game")
                this.setState({
                    gameState: GAME_MANAGER.state
                });
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
            <StyledText noLinks={true}>{this.state.gameState.players[grave.playerIndex]?.toString()}</StyledText>
            <StyledText noLinks={true}>
                {`(${graveRoleString})`}
            </StyledText>
        </button>);
    }
    renderGraveExtended(grave: Grave){
        return(<button className="grave-button" onClick={()=>{this.setState({extendedGraveIndex:null})}}>
            <GraveComponent grave={grave} gameState={this.state.gameState}/>
        </button>);
    }

    renderRoleList(){return<>
        {this.state.gameState.roleList.map((entry, index)=>{
            return <button 
                style={{ gridRow: index + 1, gridColumn:1 }} 
                key={index}
            >
                <StyledText noLinks={false}>
                    {translateRoleOutline(entry) ?? ""}
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
                    : 
                    Array.from(this.state.gameState.excludedRoles.values()).map((entry, i)=>{
                        return <button 
                            key={i}
                        >
                            <StyledText noLinks={false}>
                                {translate("role."+entry+".name") ?? ""}
                            </StyledText>
                        </button>
                    })
                }
            </div>
        </div>
    }


    render(){return(<div className="graveyard-menu graveyard-menu-colors">
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