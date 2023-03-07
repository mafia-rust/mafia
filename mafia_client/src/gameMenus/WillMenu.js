import React from "react";
import gameManager from "../index.js";

export class WillMenu extends React.Component {
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
    render(){return(<div style={{height: "100%", overflowX:"hidden"}}>
        <button>Will</button><br/>
        <textarea
        // onKeyPress={
        //     (e) => {
        //         if(e.code === "Enter") {
        //             (props.onEnter ? props.onEnter : ()=>{})();
        //         }
        //         (props.onKeyPress ? props.onKeyPress : ()=>{})(e);
        //     }
        // } 
        value={this.state.notePadValue}
        onChange={(e)=>{this.setState({saved:false, notePadValue : e.target.value});}} 
        style={{minWidth:"98%", minHeight:"80%", textAlign:"left"}} ></textarea><br/>
        <button>Save</button><button>Post</button>
    </div>)}
}