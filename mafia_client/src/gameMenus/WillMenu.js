import React from "react";
import gameManager from "../index";
import "./willMenu.css"
export class WillMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,
            willFeild: gameManager.gameState.will,
        };
        this.listener = ()=>{
            this.setState({
                gameState: gameManager.gameState,
            })
        };  
    }
    componentDidMount() {
        gameManager.addStateListener(this.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListener(this.listener);
    }
    render(){return(<div class= "will-menu textarea">
        Will
        <br/>
        <textarea class="textarea-text"
        onKeyPress={(e) => {
            if(e.code === "Enter") {
                gameManager.saveWill_button(this.state.willFeild)
            }
        }}
        value={this.state.willFeild}
        onChange={(e)=>{this.setState({willFeild : e.target.value});}}>
        </textarea><br/>
        <button className="gm-button" onClick={()=>{gameManager.saveWill_button(this.state.willFeild)}}>Save</button>
        <button className="gm-button" onClick={()=>{gameManager.sendMessage_button(this.state.gameState.will)}}>Post</button>
    </div>)}
}