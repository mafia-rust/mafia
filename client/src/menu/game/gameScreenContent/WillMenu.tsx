import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameScreen, { ContentMenus } from "../GameScreen";
import "./willMenu.css"
import GameState from "../../../game/gameState.d";
import { StateListener } from "../../../game/net/gameManager.d";

interface WillMenuState {
    gameState : GameState,
    willFeild: string,
    notesFeild: string
}

export default class WillMenu extends React.Component<{}, WillMenuState> {
    listener: StateListener
    constructor(props: {}) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            willFeild: GAME_MANAGER.gameState.will,
            notesFeild: GAME_MANAGER.gameState.notes,
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
    renderWillInput(){
        return (<div className= "will-menu textarea">
            <br/>
            {translate("menu.will.will")}
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
            <button className="gm-button" onClick={()=>{GAME_MANAGER.sendSaveWillPacket(this.state.willFeild)}}>{translate("menu.will.save")}</button>
            <button className="gm-button" onClick={()=>{GAME_MANAGER.sendSendMessagePacket(this.state.gameState.will)}}>{translate("menu.will.post")}</button>
        </div>)
    }
    renderNotesInput(){
        return (<div className= "will-menu textarea">
            <br/>
            {translate("menu.will.notes")}
            <textarea 
                className="textarea-text"
                onKeyPress={(e) => {
                    if(e.code === "Enter") {
                        GAME_MANAGER.sendSaveNotesPacket(this.state.notesFeild)
                    }
                }}
                value={this.state.notesFeild}
                onChange={(e)=>{this.setState({notesFeild : e.target.value});}}>
            </textarea>
            <button className="gm-button" onClick={()=>{GAME_MANAGER.sendSaveNotesPacket(this.state.notesFeild)}}>{translate("menu.will.save")}</button>
            <button className="gm-button" onClick={()=>{GAME_MANAGER.sendSendMessagePacket(this.state.gameState.notes)}}>{translate("menu.will.post")}</button>
        </div>)
    }
    render() {return (<div>
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.WillMenu)}}>{translate("menu.will.title")}</button>
        <br/>
        {this.renderWillInput()}
        {this.renderNotesInput()}
    </div>);}
}