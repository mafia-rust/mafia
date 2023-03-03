import React from "react";
import gameManager from "../index.js";

export class PhaseRowMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,

            listener : {func : ()=>{
                this.setState({
                    gameState: gameManager.gameState
                })
            }},
        };
                
    }

    componentDidMount() {
        gameManager.addStateListner(this.state.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListner(this.state.listener);
    }
    renderPhaseSpecific(){
        switch(this.state.gameState.phase){
            case"Voting":
            return(<div>
                <button onClick={()=>{gameManager.vote_button(null);}}>Reset Vote</button>
            </div>);
            case"Judgement":
            return(<div>
                {this.state.gameState.playerOnTrial}:{this.state.gameState.players[this.state.gameState.playerOnTrial].name}
                <div
                    style={{
                        display:"grid",                        
                        gridAutoColumns: "1fr",
                    }}
                >
                    <button style={{gridColumn: 2}} onClick={()=>{gameManager.judgement_button(-1)}}>Guilty</button>
                    <button style={{gridColumn: 3}} onClick={()=>{gameManager.judgement_button(0)}}>Abstain</button>
                    <button style={{gridColumn: 4}} onClick={()=>{gameManager.judgement_button(1)}}>Innocent</button>
                    <div style={{gridColumn: 5}}></div>
                </div>
            </div>);
            case"Night":
            return(<div>
                <button onClick={()=>{gameManager.target_button([]);}}>Reset Targets</button>
            </div>);
        }
    }
    render(){return(<div>
        <br/>
        {this.state.gameState.phase} {this.state.gameState.dayNumber}<br/>
        {this.state.gameState.secondsLeft}<br/>
        {this.renderPhaseSpecific()}<br/>
    </div>)}
}