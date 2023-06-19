import React from "react";
import translate, { styleText } from "../../game/lang";
import GAME_MANAGER from "../../index";
import { PhaseState, Verdict } from "../../game/gameState.d";
import GameScreen, { ContentMenus as GameScreenContentMenus } from "./GameScreen";
import "./headerMenu.css";
import { StateListener } from "../../game/gameManager.d";


type HeaderMenuProps = {
}
type HeaderMenuState = {
    phaseState: PhaseState,
    dayNumber: number,
    secondsLeft: number,
}

export default class HeaderMenu extends React.Component<HeaderMenuProps, HeaderMenuState> {
    phaseListener: StateListener;
    secondsListener: StateListener;
    
    constructor(props: HeaderMenuProps) {
        super(props);

        this.state = {
            phaseState: GAME_MANAGER.gameState.phaseState!,
            dayNumber: GAME_MANAGER.gameState.dayNumber,
            secondsLeft: GAME_MANAGER.gameState.secondsLeft,
        };
        this.phaseListener = () => {
            this.setState({
                phaseState: GAME_MANAGER.gameState.phaseState!,
                dayNumber: GAME_MANAGER.gameState.dayNumber,
                secondsLeft: GAME_MANAGER.gameState.secondsLeft
            });
        };
        this.secondsListener = () => {
            this.setState({
                secondsLeft: GAME_MANAGER.gameState.secondsLeft
            })
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener("phaseState", this.phaseListener);
        GAME_MANAGER.addStateListener("tick", this.secondsListener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener("phaseState", this.phaseListener);
        GAME_MANAGER.removeStateListener("tick", this.secondsListener);
    }
    renderPhaseSpecific(){
        switch(this.state.phaseState?.phase){
            case "judgement":
                return(<div className="judgement-specific">
                    <div>
                    {styleText(GAME_MANAGER.gameState.players[this.state.phaseState.playerOnTrial]?.toString())}
                    {(()=>{
                        if (this.state.phaseState.playerOnTrial === GAME_MANAGER.gameState.myIndex) {
                            return <div className="judgement-info">{translate("judgement.cannotVote.onTrial")}</div>;
                        } else if (!GAME_MANAGER.gameState.players[GAME_MANAGER.gameState.myIndex!].alive){
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
            default:
                return null;
        }
    }

    renderVerdictButton(verdict: Verdict) {
        return <button
            className={GAME_MANAGER.gameState.judgement === verdict ? "highlighted" : undefined}
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
        if(this.state.phaseState.phase){
            return(<div>
                {translate("phase."+this.state.phaseState.phase)} {this.state.dayNumber}⏳{this.state.secondsLeft}
            </div>);
        }
        return null;
    }

    render(){return(<div className="header-menu">
        {this.renderPhase()}
        {(()=>{
            if(GAME_MANAGER.gameState.myIndex !== null){
                return styleText(GAME_MANAGER.gameState.players[GAME_MANAGER.gameState.myIndex].toString() +
                " (" + translate("role."+GAME_MANAGER.gameState.roleState.role+".name") + ")");
            }
        })()}
        {this.renderPhaseSpecific()}
        {this.renderMenuButtons()}
    </div>)}
}