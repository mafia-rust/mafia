import React from "react";
import translate from "../../game/lang";
import Anchor from "../Anchor";
import GAME_MANAGER from "../..";
import LoadingScreen from "../LoadingScreen";


type PlayMenuProps = {
}
type PlayMenuState = {
    selectedRoomCode: string | null,
}

export default class PlayMenu extends React.Component<PlayMenuProps, PlayMenuState> {
    constructor(props: PlayMenuProps) {
        super(props);

        this.state = {
            selectedRoomCode: null,
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    private async joinGameButton() {
        if(this.state.selectedRoomCode !== null){
            Anchor.setContent(<LoadingScreen type="join"/>);
            GAME_MANAGER.tryJoinGame(this.state.selectedRoomCode);
        }
    }
    
    private async hostGameButton() {
        Anchor.setContent(<LoadingScreen type="host"/>);
        GAME_MANAGER.sendHostPacket();
    }

    render() {
        return <div className="playMenu">
            <header>
                <h1>
                    {translate("menu.join.title")}
                </h1>
            </header>

            <div> 
                <section>
                    <label>{translate("menu.join.field.roomCode")}</label>
                    <input type="text" value={this.state.selectedRoomCode??""} 
                        onChange={(e)=>{this.setState({selectedRoomCode: e.target.value})}}
                        onKeyUp={(e)=>{
                            if(e.key === 'Enter') {
                                this.joinGameButton();
                            }
                        }}
                    />
                </section>
            </div>
            
            <button onClick={()=>{this.joinGameButton()}}>
                {translate("menu.start.button.join")}
            </button>
            <button onClick={()=>{this.hostGameButton()}}>
                {translate("menu.start.button.host")}
            </button>
        </div>
    }
}