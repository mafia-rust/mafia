import React from "react";
import GAME_MANAGER from "@/index";
import "@/index.css";
import "@menu/main/joinMenu.css";
import Anchor from "@menu/Anchor";
import * as LoadingScreen from "@menu/LoadingScreen";
import translate from "@game/lang";

interface JoinMenuState {
    roomCode: string,
    name: string,
}

export default class JoinMenu extends React.Component<any, JoinMenuState> {
    constructor(props: any) {
        super(props);

        this.state = {
            roomCode: "",
            name: /* logged in ? username : */ "",
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    private setRoomCode(code: string) {
        this.setState({roomCode: code})
    }
    private setName(name: string) {
        this.setState({name: name})
    }
    joinGameButton(){
        // erm... >.<
        GAME_MANAGER.roomCode = this.state.roomCode;
        GAME_MANAGER.name = this.state.name;

        Anchor.setContent(LoadingScreen.create(LoadingScreen.Type.Join));

        GAME_MANAGER.Server.close();
        GAME_MANAGER.Server.open();

        // Wait for server to open
        setTimeout(GAME_MANAGER.join_button, 1000);
        setTimeout(()=>{GAME_MANAGER.setName_button(this.state.name)}, 1000)
    }
    render(){return(<div style={{display: "flex", flexDirection: "column"}}>
        <div className="header jm-header">
            <h1 className="header-text jm-header-text">
                {translate("menu.join.title")}
            </h1>
        </div>
        <div className="jm-input-column">
            <div className="input-box">
                <h3 className="input-box-label">{translate("menu.join.field.room_code")}</h3>
                <input className="input-field" type="text" value={this.state.roomCode} 
                    onChange={(e)=>{this.setRoomCode(e.target.value)}}
                    onKeyUp={(e)=>{
                        if(e.key === 'Enter') {
                            GAME_MANAGER.roomCode = this.state.roomCode;
                            this.joinGameButton();
                        }
                    }}
                />
            </div>
            <div className="input-box">
                <h3 className="input-box-label">{translate("menu.join.field.name")}</h3>
                <input className="input-field" type="text" value={this.state.name} 
                    onChange={(e)=>{this.setName(e.target.value)}}
                    onKeyUp={(e)=>{
                        if(e.key === 'Enter') {
                            GAME_MANAGER.name = this.state.name;
                            this.joinGameButton();
                        }
                    }}
                />
            </div>
            <button className="button jm-button" onClick={()=>{this.joinGameButton()}}>
                {translate("menu.join.button.join")}
            </button>
        </div>
    </div>)}
}