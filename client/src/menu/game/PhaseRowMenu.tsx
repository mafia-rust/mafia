import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import GameState, { Phase } from "../../game/gameState.d";
import GameScreen from "./GameScreen";
import PlayerListMenu from "./gameScreenContent/PlayerListMenu";
import GraveyardMenu from "./gameScreenContent/GraveyardMenu";
import WikiMenu from "./gameScreenContent/WikiMenu";
import WillMenu from "./gameScreenContent/WillMenu";


type PhaseRowMenuProps = {
    phase: Phase | null,
}
type PhaseRowMenuState = {
    gameState: GameState,
}

export default class PhaseRowMenu extends React.Component<PhaseRowMenuProps, PhaseRowMenuState> {
    listener: () => void;
    
    constructor(props: PhaseRowMenuProps) {
        super(props);

        this.state = {
            gameState: GAME_MANAGER.gameState,
        };
        this.listener = () => {
            this.setState({
                gameState: GAME_MANAGER.gameState
            }); // update the component state with the new copy
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
            
            case Phase.Judgement:
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
                            <button style={{gridColumn: 2}} onClick={()=>{GAME_MANAGER.sendJudgementPacket(-1)}}>{translate("verdict.Guilty")}</button>
                            <button style={{gridColumn: 3}} onClick={()=>{GAME_MANAGER.sendJudgementPacket(0)}}>{translate("verdict.Abstain")}</button>
                            <button style={{gridColumn: 4}} onClick={()=>{GAME_MANAGER.sendJudgementPacket(1)}}>{translate("verdict.Innocent")}</button>
                            <div style={{gridColumn: 5}}></div>
                        </div></div>);})()}
                </div>);
            }else{
                return(<div>
                    ERROR NO PLAYER ON TRIAL FOUND IN JUDGEMENT PHASE
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
            <button onClick={()=>{GameScreen.instance.openOrCloseMenu(<WillMenu/>)}}>{translate("menu.will.title")}</button>
            <button onClick={()=>{GameScreen.instance.openOrCloseMenu(<PlayerListMenu/>)}}>{translate("menu.playerList.title")}</button>
            <button onClick={()=>{GameScreen.instance.openOrCloseMenu(<GraveyardMenu/>)}}>{translate("menu.graveyard.title")}</button>
            <button onClick={()=>{GameScreen.instance.openOrCloseMenu(<WikiMenu role={null}/>)}}>{translate("menu.wiki.title")}</button>
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