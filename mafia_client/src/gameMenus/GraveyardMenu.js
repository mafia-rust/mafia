import React from "react";
import gameManager from "../index.js";

export class GraveyardMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
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
    renderGrave(grave, index){
        return(<div>
            {JSON.stringify(grave)}
        </div>)
    }
    renderExtendedGrave(){
        //this is supposed to be for rendering will when a button is pressed to extend it
    }
    render(){return(<div>
        {this.state.gameState.graves.map((grave, index)=>{
            return this.renderGrave(grave, index);
            
        }, this)}
    </div>)}
}