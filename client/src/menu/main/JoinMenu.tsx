import React from "react";
import GAME_MANAGER from "../../index";
import "../../index.css";
import "./joinMenu.css";
import Anchor from "../Anchor";
import * as LoadingScreen from "../LoadingScreen";
import translate from "../../game/lang";

type JoinMenuProps = {
    roomCode: string | null,
}
type JoinMenuState = {
    roomCode: string,
}

export default class JoinMenu extends React.Component<JoinMenuProps, JoinMenuState> {
    constructor(props: JoinMenuProps) {
        super(props);

        this.state = {
            roomCode: this.props.roomCode != null ? this.props.roomCode : "",
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    private setRoomCode(code: string) {
        this.setState({roomCode: code})
    }
    async joinGameButton(){
        GAME_MANAGER.roomCode = this.state.roomCode;

        Anchor.setContent(LoadingScreen.create(LoadingScreen.Type.Join));

        GAME_MANAGER.server.close();
        await GAME_MANAGER.server.open();
        
        await GAME_MANAGER.sendJoinPacket();
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
            </div>
            
            <button onClick={()=>{this.joinGameButton()}}>
                {translate("menu.join.button.join")}
            </button>
        </div>
    )}
}