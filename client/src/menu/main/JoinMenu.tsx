import React from "react";
import GAME_MANAGER from "../../index";
import "../../index.css";
import "./joinMenu.css";
import Anchor from "../Anchor";
import * as LoadingScreen from "../LoadingScreen";
import translate from "../../game/lang";

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
    async joinGameButton(){
        // erm... >.<
        GAME_MANAGER.roomCode = this.state.roomCode;
        GAME_MANAGER.name = this.state.name;

        Anchor.setContent(LoadingScreen.create(LoadingScreen.Type.Join));

        GAME_MANAGER.Server.close();
        await GAME_MANAGER.Server.open();
        
        await GAME_MANAGER.join_button();
        
        GAME_MANAGER.setName_button(this.state.name);
    }
    render(){return(<div style={{display: "flex", flexDirection: "column"}}>
        <header className="jm-header">
            <h1 className="jm-header-text">
                {translate("menu.join.title")}
            </h1>
        </header>
        <form className="input-column">
            <div>
                <label htmlFor="roomcode">{translate("menu.join.field.room_code")}</label>
                <input name="roomcode" type="text" value={this.state.roomCode} 
                    onChange={(e)=>{this.setRoomCode(e.target.value)}}
                    onKeyUp={(e)=>{
                        if(e.key === 'Enter') {
                            GAME_MANAGER.roomCode = this.state.roomCode;
                            this.joinGameButton();
                        }
                    }}
                />
            </div>
            <div>
                <label htmlFor="name">{translate("menu.join.field.name")}</label>
                <input name="name" type="text" value={this.state.name} 
                    onChange={(e)=>{this.setName(e.target.value)}}
                    onKeyUp={(e)=>{
                        if(e.key === 'Enter') {
                            GAME_MANAGER.name = this.state.name;
                            this.joinGameButton();
                        }
                    }}
                />
            </div>
            <button className="jm-button" onClick={()=>{this.joinGameButton()}}>
                {translate("menu.join.button.join")}
            </button>
        </form>
    </div>)}
}