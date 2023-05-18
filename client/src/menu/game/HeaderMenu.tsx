import React from "react";
import translate from "../../game/lang";
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
            //TODO make buttons light up if they are clicked
            if(this.state.gameState.playerOnTrial !== null){
                return(<div>
                    {
                        this.state.gameState.players[this.state.gameState.playerOnTrial!]?.toString()
                    }
                    {(()=>{
                    if(this.state.gameState.playerOnTrial !== this.state.gameState.myIndex)
                        return(<div>
                            {translate("verdict."+this.state.gameState.judgement)}
                        <div
                            style={{
                                display:"grid",
                                gridAutoColumns: "1fr",
                            }}
                        >
                            <button style={{gridColumn: 2}} onClick={()=>{GAME_MANAGER.sendJudgementPacket(Verdict.Guilty)}}>{translate("verdict.Guilty")}</button>
                            <button style={{gridColumn: 3}} onClick={()=>{GAME_MANAGER.sendJudgementPacket(Verdict.Abstain)}}>{translate("verdict.Abstain")}</button>
                            <button style={{gridColumn: 4}} onClick={()=>{GAME_MANAGER.sendJudgementPacket(Verdict.Innocent)}}>{translate("verdict.Innocent")}</button>
                            <div style={{gridColumn: 5}}></div>
                        </div></div>);})()}
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
                return this.state.gameState.players[this.state.gameState.myIndex].toString() + " (" + this.state.gameState.players[this.state.gameState.myIndex].roleLabel + ")"
            }
        })()}
        {this.renderPhaseSpecific()}
        {this.renderMenuButtons()}
    </div>)}
}