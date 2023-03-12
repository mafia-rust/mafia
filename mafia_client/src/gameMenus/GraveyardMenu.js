import React from "react";
import gameManager from "../index.js";

export class GraveyardMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
            expandedGraves: null,    //list of graveIndexs of what graves should be showing its will 
        };
        this.listener = ()=>{
            this.setState({
                gameState: gameManager.gameState
            })
        };  
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }
    renderGrave(grave, graveIndex){
        return(<div key={graveIndex}>
            {grave.diedPhase} {grave.dayNumber}<br/>
            {grave.playerIndex+1}:{this.gameState.players[grave.playerIndex]}<br/>
            {grave.role} killed by {(()=>{
                let outString = "";
                for(let i = 0; i < grave.killer.length; i++){
                    outString+=grave.killer;
                }
                return outString;
            })()}
            <button onClick={()=>{
                if(this.state.expandedGraves.contains(graveIndex)){
                    this.state.expandedGraves.splice(this.state.expandedGraves.indexOf(graveIndex));
                }else{
                    this.state.expandedGraves.push(graveIndex);
                }  
            }}>Expand</button>
            {(()=>{if(this.state.expandedGraves.contains(graveIndex))return this.renderExtendedGrave(grave, graveIndex)})()}
        </div>)
    }
    renderExtendedGrave(grave, graveIndex){
        return(<div>{grave.will}</div>);
    }
    render(){return(<div>
        {this.state.gameState.role}
        {this.state.gameState.graves.map((grave, graveIndex)=>{
            return this.renderGrave(grave, graveIndex);
        }, this)}
    </div>)}
}