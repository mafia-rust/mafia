import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./willMenu.css"

export default class WillMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            willFeild: GAME_MANAGER.gameState.will,
        };
        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState,
            })
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    render() {return (<div>
        <div class= "will-menu textarea">
            {translate("menu.willMenu.will")}
            <br/>
            <textarea 
                class="textarea-text"
                onKeyPress={(e) => {
                    if(e.code === "Enter") {
                        gameManager.saveWill_button(this.state.willFeild)
                    }
                }}
                value={this.state.willFeild}
                onChange={(e)=>{this.setState({willFeild : e.target.value});}}>
            </textarea>
            <br/>
            <button className="gm-button" onClick={()=>{gameManager.saveWill_button(this.state.willFeild)}}>{translate("menu.willMenu.save")}</button>
            <button className="gm-button" onClick={()=>{gameManager.sendMessage_button(this.state.gameState.will)}}>{translate("menu.willMenu.post")}</button>
        </div>

        <div>
            <ForgerMenu/>
        </div>
    </div>);}
}