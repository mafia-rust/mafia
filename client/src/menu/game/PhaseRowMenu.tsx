import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import GameState, { Phase } from "../../game/gameState.d";

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
                    {GAME_MANAGER.getPlayer(this.state.gameState.playerOnTrial!)?.toString()}
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
                            <button style={{gridColumn: 2}} onClick={()=>{GAME_MANAGER.judgement_button(-1)}}>{translate("verdict.Guilty")}</button>
                            <button style={{gridColumn: 3}} onClick={()=>{GAME_MANAGER.judgement_button(0)}}>{translate("verdict.Abstain")}</button>
                            <button style={{gridColumn: 4}} onClick={()=>{GAME_MANAGER.judgement_button(1)}}>{translate("verdict.Innocent")}</button>
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
    //Will Menu, Playerlist Menu, Rolelist/Graveyard menu, Wiki Menu
    renderMenuButtons(){
        return <div>
            {/* <button onClick={()=>{GAME_MANAGER.menu_button("Will")}}>{translate("menu.will")}</button>
            <button onClick={()=>{GAME_MANAGER.menu_button("Playerlist")}}>{translate("menu.playerlist")}</button>
            <button onClick={()=>{GAME_MANAGER.menu_button("Rolelist")}}>{translate("menu.rolelist")}</button>
            <button onClick={()=>{GAME_MANAGER.menu_button("Wiki")}}>{translate("menu.wiki")}</button> */}
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