import React from "react";
import gameManager from "../index";
import "../index.css";
import "./joinMenu.css";
import * as LoadingScreen from "./LoadingScreen";
import { Main } from "../Main";
import { translate } from "../game/lang";

type JoinMenuState = {
    roomCode: string,
    name: string,
}

export class JoinMenu extends React.Component<any, JoinMenuState> {
    constructor(props: any) {
        super(props);

        this.state = {
            roomCode: "",
            name: Main.instance?.isLoggedIn() ? Main.instance?.getUser()?.getAccountName() : "",
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    setRoomCode(code: string) {
        this.setState({roomCode: code})
    }
    setName(name: string) {
        this.setState({name: name})
    }
    joinGameButton(){
        // erm... >.<
        gameManager.roomCode = this.state.roomCode;
        gameManager.name = this.state.name;

        Main.instance.setContent(LoadingScreen.create(LoadingScreen.Type.Join));

        gameManager.Server.close();
        gameManager.Server.open();

        // Wait for server to open
        setTimeout(gameManager.join_button, 1000);
        setTimeout(()=>{gameManager.setName_button(this.state.name)}, 1000)
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
                            gameManager.roomCode = this.state.roomCode;
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
                            gameManager.name = this.state.name;
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