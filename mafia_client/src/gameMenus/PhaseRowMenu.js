import React from "react";
import { getPlayerString, translate } from "../game/lang";
import gameManager from "../index";

export class PhaseRowMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState: gameManager.gameState,
        };
        this.listener = () => {
            this.setState({
                gameState: gameManager.gameState
            }); // update the component state with the new copy
        };
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }
    renderPhaseSpecific(){
        switch(this.state.gameState.phase){
            
            case"Judgement":
            //TODO make buttons light up if they are clicked
            return(<div>
                {getPlayerString(this.state.gameState.playerOnTrial)}
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
                        <button style={{gridColumn: 2}} onClick={()=>{gameManager.judgement_button(-1)}}>{translate("verdict.Guilty")}</button>
                        <button style={{gridColumn: 3}} onClick={()=>{gameManager.judgement_button(0)}}>{translate("verdict.Abstain")}</button>
                        <button style={{gridColumn: 4}} onClick={()=>{gameManager.judgement_button(1)}}>{translate("verdict.Innocent")}</button>
                        <div style={{gridColumn: 5}}></div>
                    </div></div>);})()}
            </div>);
            default:
            return null;
        }
    }
    render(){return(<div>
        <br/>
        {translate("phase."+this.state.gameState.phase)} {this.state.gameState.dayNumber}<br/>
        {this.state.gameState.secondsLeft}<br/>
        {this.renderPhaseSpecific()}<br/>
    </div>)}
}