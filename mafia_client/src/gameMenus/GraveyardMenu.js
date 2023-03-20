import React from "react";
import { getPlayerString } from "../game/lang.js";
import gameManager from "../index.js";

export class GraveyardMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
            expandedGraves: [],    //list of graveIndexs of what graves should be showing its will 
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
            {grave.playerIndex+1}:{this.state.gameState.players[grave.playerIndex]}<br/>
            {grave.role} killed by {(()=>{
                if(grave.death_cause === "Lynching"){
                    return <div>{"a lynching."}</div>
                }
                return <div>{grave.death_cause.Killers.killers.join(", ") + "."}</div>
            })()}
            <button onClick={()=>{
                if(this.state.expandedGraves.includes(graveIndex)){
                    this.state.expandedGraves.splice(this.state.expandedGraves.indexOf(graveIndex));
                }else{
                    this.state.expandedGraves.push(graveIndex);
                }  
            }}>Expand</button>
            {(()=>{if(this.state.expandedGraves.includes(graveIndex))return this.renderExtendedGrave(grave, graveIndex)})()}
        </div>)
    }
    renderExtendedGrave(grave, graveIndex){
        return(<div>{grave.will}</div>);
    }

    renderRoleList(){
        return<div>

        </div>
    }
    renderRoleListEntry(){
        return <div>
            <button></button>
        </div>
    }
    render(){return(<div>
        {getPlayerString(gameManager.gameState.myIndex)}: {this.state.gameState.role}
        {/* {this.state.gameState.graves.map((grave, graveIndex)=>{
            return this.renderGrave(grave, graveIndex);
        }, this)} */}
    </div>)}
}