import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import GameState, { Phase, Verdict } from "../../game/gameState.d";
import GameScreen, { ContentMenus as GameScreenContentMenus } from "./GameScreen";


type PhaseRowMenuProps = {
    phase: Phase | null,
}
type PhaseRowMenuState = {
    gameState: GameState,

    willMenuOpen: boolean,
    playerListMenuOpen: boolean,
    graveyardMenuOpen: boolean,
    wikiMenuOpen: boolean,
}

export default class PhaseRowMenu extends React.Component<PhaseRowMenuProps, PhaseRowMenuState> {
    listener: () => void;
    
    constructor(props: PhaseRowMenuProps) {
        super(props);

        this.state = {
            gameState: GAME_MANAGER.gameState,

            willMenuOpen: true,
            playerListMenuOpen: true,
            graveyardMenuOpen: true,
            wikiMenuOpen: true,
        };
        this.listener = () => {
            this.setState({
                gameState: GAME_MANAGER.gameState,

                willMenuOpen: GAME_MANAGER.willMenuOpen,
                playerListMenuOpen: GAME_MANAGER.playerListMenuOpen,
                graveyardMenuOpen: GAME_MANAGER.graveyardMenuOpen,
                wikiMenuOpen: GAME_MANAGER.wikiMenuOpen,
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
    render(){return(<div>
        <br/>
        {this.renderPhaseName()}
        {this.state.gameState.secondsLeft}<br/>
        {this.renderPhaseSpecific()}<br/>
        {this.renderMenuButtons()}
    </div>)}
    renderMenuButtons(){
        return <div>
            {(()=>
                this.state.willMenuOpen?null:
                    <button onClick={()=>{
                        GameScreen.instance.openMenu(GameScreenContentMenus.WillMenu)
                    }}>{translate("menu.will.title")}</button>
            )()}
            {(()=>
                this.state.playerListMenuOpen?null:
                    <button onClick={()=>{
                        GameScreen.instance.openMenu(GameScreenContentMenus.PlayerListMenu)
                    
                    }}>{translate("menu.playerList.title")}</button>
            )()}
            {(()=>
                this.state.graveyardMenuOpen?null:
                    <button onClick={()=>{
                        GameScreen.instance.openMenu(GameScreenContentMenus.GraveyardMenu)
                    
                    }}>{translate("menu.graveyard.title")}</button>
            )()}
            {(()=>
                this.state.wikiMenuOpen?null:
                    <button onClick={()=>{
                        GameScreen.instance.openMenu(GameScreenContentMenus.WikiMenu)
                    
                    }}>{translate("menu.wiki.title")}</button>
            )()}
        </div>
    }
    renderPhaseName(){
        if(this.state.gameState.phase){
            return(<div>
                {translate("phase."+this.state.gameState.phase)} {this.state.gameState.dayNumber}<br/> 
            </div>);
        }
        return null;
    }
}