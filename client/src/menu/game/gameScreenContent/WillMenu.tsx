import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import ForgerMenu from "./ForgerMenu";
import GameScreen, { ContentMenus } from "../GameScreen";
import "./willMenu.css"
import GameState from "../../../game/gameState.d";
import { StateListener } from "../../../game/net/gameManager.d";

interface WillMenuState {
    gameState : GameState,
    willFeild: string,
}

export default class WillMenu extends React.Component<{}, WillMenuState> {
    listener: StateListener
    constructor(props: {}) {
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
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.WillMenu)}}>{translate("menu.will.title")}</button>
            
        <div className= "will-menu textarea">
            <br/>
            <textarea 
                className="textarea-text"
                onKeyPress={(e) => {
                    if(e.code === "Enter") {
                        GAME_MANAGER.sendSaveWillPacket(this.state.willFeild)
                    }
                }}
                value={this.state.willFeild}
                onChange={(e)=>{this.setState({willFeild : e.target.value});}}>
            </textarea>
            <br/>
            <button className="gm-button" onClick={()=>{GAME_MANAGER.sendSaveWillPacket(this.state.willFeild)}}>{translate("menu.will.save")}</button>
            <button className="gm-button" onClick={()=>{GAME_MANAGER.sendSendMessagePacket(this.state.gameState.will)}}>{translate("menu.will.post")}</button>
        </div>

        <div>
            <ForgerMenu/>
        </div>
    </div>);}
}