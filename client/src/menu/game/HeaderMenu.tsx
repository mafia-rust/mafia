import React from "react";
import translate, { styleText } from "../../game/lang";
import GAME_MANAGER from "../../index";
import GameState, { Phase, Verdict } from "../../game/gameState.d";
import GameScreen, { ContentMenus as GameScreenContentMenus } from "./GameScreen";
import "./headerMenu.css";


type HeaderMenuProps = {
    phase: Phase | null,
}
type HeaderMenuState = {
    gameState: GameState,
}

export default class HeaderMenu extends React.Component<HeaderMenuProps, HeaderMenuState> {
    listener: () => void;
    
    constructor(props: HeaderMenuProps) {
        super(props);

        this.state = {
            gameState: GAME_MANAGER.gameState,
        };
        this.listener = () => {
            this.setState({
                gameState: GAME_MANAGER.gameState,
            });
        };
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    renderPhaseSpecific(){
        switch(this.state.gameState.phase){
            case "judgement":
            if(this.state.gameState.playerOnTrial !== null){
                return(<div className="judgement-specific">
                    <div>
                    {styleText(this.state.gameState.players[this.state.gameState.playerOnTrial!]?.toString())}
                    {(()=>{
                        if (this.state.gameState.playerOnTrial === this.state.gameState.myIndex) {
                            return <div className="judgement-info">{translate("judgement.cannotVote.onTrial")}</div>;
                        } else if (!this.state.gameState.players[this.state.gameState.myIndex!].alive){
                            return <div className="judgement-info">{translate("judgement.cannotVote.dead")}</div>;
                        } else {
                            return(<div className="judgement-info">
                                {this.renderVerdictButton("guilty")}
                                {this.renderVerdictButton("abstain")}
                                {this.renderVerdictButton("innocent")}
                            </div>);
                        }
                    })()}
                    </div>
                </div>);
            }else{
                //TODO lang or fix
                return(<div> 
                    ERROR NO PLAYER ON TRIAL FOUND IN JUDGEMENT PHASE TODO 
                </div>);
            }
            
            default:
            return null;
        }
    }

    renderVerdictButton(verdict: Verdict) {
        return <button
            className={this.state.gameState.judgement === verdict ? "highlighted" : undefined}
            onClick={()=>{GAME_MANAGER.sendJudgementPacket(verdict)}}
        >
            {styleText(translate("verdict." + verdict))}
        </button>
    }
    
    renderMenuButtons(){
        return <div className="menu-buttons">
            {(()=>
                GameScreen.instance.menusOpen().includes(GameScreenContentMenus.WillMenu)?null:
                    <button onClick={()=>{
                        GameScreen.instance.openMenu(GameScreenContentMenus.WillMenu)
                    }}>{translate("menu.will.title")}</button>
            )()}
            {(()=>
                GameScreen.instance.menusOpen().includes(GameScreenContentMenus.PlayerListMenu)?null:
                    <button onClick={()=>{
                        GameScreen.instance.openMenu(GameScreenContentMenus.PlayerListMenu)
                    
                    }}>{translate("menu.playerList.title")}</button>
            )()}
            {(()=>
                GameScreen.instance.menusOpen().includes(GameScreenContentMenus.GraveyardMenu)?null:
                    <button onClick={()=>{
                        GameScreen.instance.openMenu(GameScreenContentMenus.GraveyardMenu)
                    
                    }}>{translate("menu.graveyard.title")}</button>
            )()}
            {(()=>
                GameScreen.instance.menusOpen().includes(GameScreenContentMenus.WikiMenu)?null:
                    <button onClick={()=>{
                        GameScreen.instance.openMenu(GameScreenContentMenus.WikiMenu)
                    
                    }}>{translate("menu.wiki.title")}</button>
            )()}
        </div>
    }
    renderPhase(){
        if(this.state.gameState.phase){
            return(<div>
                {translate("phase."+this.state.gameState.phase)} {this.state.gameState.dayNumber}⏳{this.state.gameState.secondsLeft}
            </div>);
        }
        return null;
    }

    render(){return(<div className="header-menu">
        {this.renderPhase()}
        {(()=>{
            if(this.state.gameState.myIndex !== null){
                return styleText(this.state.gameState.players[this.state.gameState.myIndex].toString() +
                 " (" + translate("role."+this.state.gameState.role+".name") + ")");
            }
        })()}
        {this.renderPhaseSpecific()}
        {this.renderMenuButtons()}
    </div>)}
}