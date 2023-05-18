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
            roomCode: this.props.roomCode != null ? this.props.roomCode : "",
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
        GAME_MANAGER.roomCode = this.state.roomCode;

        Anchor.setContent(LoadingScreen.create(LoadingScreen.Type.Join));

        GAME_MANAGER.server.close();
        await GAME_MANAGER.server.open();
        
        await GAME_MANAGER.sendJoinPacket();
        
        if (this.state.name && this.state.name !== " ") {
            GAME_MANAGER.sendSetNamePacket(this.state.name);
        }
    }
    render(){return(
        <div className="jm">
            <header>
                <h1>
                    {translate("menu.join.title")}
                </h1>
            </header>

            <div> 
                <section>
                    <label>{translate("menu.join.field.roomCode")}</label>
                    <input type="text" value={this.state.roomCode} 
                        onChange={(e)=>{this.setRoomCode(e.target.value)}}
                        onKeyUp={(e)=>{
                            if(e.key === 'Enter') {
                                GAME_MANAGER.roomCode = this.state.roomCode;
                                this.joinGameButton();
                            }
                        }}
                    />
                </section>
                <section>
                    <label>{translate("menu.join.field.name")}</label>
                    <input type="text" value={this.state.name} 
                        onChange={(e)=>{this.setName(e.target.value)}}
                        onKeyUp={(e)=>{
                            if(e.key === 'Enter') {
                                this.joinGameButton();
                            }
                        }}
                    />
                </section>
            </div>
            
            <button onClick={()=>{this.joinGameButton()}}>
                {translate("menu.join.button.join")}
            </button>
        </div>
    )}
}