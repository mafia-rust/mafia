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
    listener: ()=>void;
    constructor(props: PlayMenuProps) {
        super(props);

        this.state = {
            selectedRoomCode: null,
        };
        this.listener = () => {
            this.forceUpdate();
        }
        let reconnectData = GAME_MANAGER.loadReconnectData();
        if(reconnectData) {
            Anchor.pushRejoin(reconnectData.roomCode, reconnectData.playerId);
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    private async joinGameButton(roomCode?: string) {
        if(!roomCode){
            Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.invalidRoomCode"));
            return;
        };
        Anchor.setContent(<LoadingScreen type="join"/>);
        GAME_MANAGER.sendJoinPacket(roomCode);
    }
    
    private async hostGameButton() {
        Anchor.setContent(<LoadingScreen type="host"/>);
        GAME_MANAGER.sendHostPacket();
    }

    private refreshButton() {
        GAME_MANAGER.sendLobbyListRequest();
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
                                this.joinGameButton(this.state.selectedRoomCode??"");
                            }
                        }}
                    />
                </section>
            </div>
            
            <button onClick={()=>{this.joinGameButton(this.state.selectedRoomCode??"")}}>
                {translate("menu.start.button.join")}
            </button>
            <button onClick={()=>{this.hostGameButton()}}>
                {translate("menu.start.button.host")}
            </button>
            <button onClick={()=>{this.refreshButton()}}>
                {translate("menu.play.button.refresh")}
            </button>
            <table>
                <thead>
                    <tr>
                        <th>ROOM CODE</th>
                        {/* <th>Players</th>
                        <th>Host</th> */}
                    </tr>
                </thead>
                <tbody>
                    {
                    GAME_MANAGER.state.stateType === "outsideLobby" &&
                    GAME_MANAGER.state.roomCodes.map((roomCode, i)=>{
                        return <tr key={i}>
                            <td><button onClick={()=>{this.joinGameButton(roomCode??"")}}>{roomCode}</button></td>
                            {/* <td>{GAME_MANAGER.state.players.get(roomCode)?.size}</td>
                            <td>{GAME_MANAGER.state.players.get(roomCode)?.getHost()?.name}</td> */}
                        </tr>
                    })}
                </tbody>
            </table>
        </div>
    }
}