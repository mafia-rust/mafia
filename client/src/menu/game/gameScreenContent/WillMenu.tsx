import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameScreen, { ContentMenus } from "../GameScreen";
import "./willMenu.css"
import GameState from "../../../game/gameState.d";
import { StateListener } from "../../../game/gameManager.d";

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
        return (<div className="textarea-section">
            {translate("menu.will.will")}
            <button 
                onClick={()=>{GAME_MANAGER.sendSaveWillPacket(this.state.willFeild)}}
                style={{borderColor: this.state.gameState.will === this.state.willFeild ? undefined : "yellow"}}
            >{translate("menu.will.save")}</button>
            <button onClick={()=>{GAME_MANAGER.sendSendMessagePacket('\n' + this.state.gameState.will)}}>{translate("menu.will.post")}</button>
            <textarea
                // onKeyPress={(e) => {
                //     if(e.code === "Enter") {
                //         GAME_MANAGER.sendSaveWillPacket(this.state.willFeild)
                //     }
                // }}
                value={this.state.willFeild}
                onChange={(e)=>{this.setState({willFeild : e.target.value});}}>
            </textarea>
        </div>)
    }
    renderNotesInput(){
        return (<div className="textarea-section">
            {translate("menu.will.notes")}
            <button 
                onClick={()=>{GAME_MANAGER.sendSaveNotesPacket(this.state.notesFeild)}}
                style={{borderColor: this.state.gameState.notes === this.state.notesFeild ? undefined : "yellow"}}
            >{translate("menu.will.save")}</button>
            <button onClick={()=>{GAME_MANAGER.sendSendMessagePacket(this.state.gameState.notes)}}>{translate("menu.will.post")}</button>
            <textarea
                // onKeyPress={(e) => {
                //     if(e.code === "Enter") {
                //         GAME_MANAGER.sendSaveNotesPacket(this.state.notesFeild)
                //     }
                // }}
                value={this.state.notesFeild}
                onChange={(e)=>{this.setState({notesFeild : e.target.value});}}>
            </textarea>
        </div>)
    }
    render() {return (<div className="will-menu">
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.WillMenu)}}>{translate("menu.will.title")}</button>
        <section>
            {this.renderWillInput()}
            {this.renderNotesInput()}
        </section>
    </div>);}
}