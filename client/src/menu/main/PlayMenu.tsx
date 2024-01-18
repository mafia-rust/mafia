import React from "react";
import translate from "../../game/lang";
import Anchor from "../Anchor";
import GAME_MANAGER from "../..";
import LoadingScreen from "../LoadingScreen";
import "./playMenu.css";


type PlayMenuProps = {
}
type PlayMenuState = {
    selectedRoomCode: string | null,
    selectedPlayerId: string | null,
}

export default class PlayMenu extends React.Component<PlayMenuProps, PlayMenuState> {
    listener: ()=>void;
    constructor(props: PlayMenuProps) {
        super(props);

        this.state = {
            selectedRoomCode: null,
            selectedPlayerId: null,
        };
        this.listener = () => {
            this.forceUpdate();
        }
        let reconnectData = GAME_MANAGER.loadReconnectData();
        if(reconnectData) {
            Anchor.pushRejoin(reconnectData.roomCode, reconnectData.playerId);
        }
        this.refreshButton();
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    private async joinGameButton(roomCode?: string) {
        if(!roomCode){
            Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.roomDoesntExist"));
            return;
        };
        Anchor.setContent(<LoadingScreen type="join"/>);
        GAME_MANAGER.sendJoinPacket(roomCode);
    }
    private rejoinGameButton(roomCode?: string, playerId?: string) {
        if(!roomCode || !playerId){
            Anchor.pushError(translate("notification.rejectJoin"), translate("notification.rejectJoin.roomDoesntExist"));
            return;
        };
        GAME_MANAGER.sendRejoinPacket(roomCode, parseInt(playerId, 10));
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
                    {translate("menu.play.title")}
                </h1>
            </header>
            
            
                <table>
                    <thead>
                        <tr>
                            <th>{translate("menu.play.field.roomCode")}</th>
                            <th style={{width:"100%"}}></th>
                        </tr>
                    </thead>
                    <tbody>
                    {
                        GAME_MANAGER.state.stateType === "outsideLobby" &&
                        GAME_MANAGER.state.roomCodes.map((roomCode, i)=>{
                            return <tr>
                                <td><button key={i} onClick={()=>{this.joinGameButton(roomCode??"")}}>{roomCode}</button></td>
                                <td></td>
                            </tr>
                        })
                    }
                    </tbody>
                    
                </table>
            
            <div>
                <section>       
                    <label>{translate("menu.play.field.roomCode")}</label>
                    <input type="text" value={this.state.selectedRoomCode??""} 
                        onChange={(e)=>{this.setState({selectedRoomCode: e.target.value})}}
                        onKeyUp={(e)=>{
                            if(e.key === 'Enter') {
                                this.joinGameButton(this.state.selectedRoomCode??"");
                            }
                        }}
                    />
                    <button onClick={()=>{this.joinGameButton(this.state.selectedRoomCode??"")}}>
                        {translate("menu.play.button.join")}
                    </button>
                </section>
                <section>
                    <label>{translate("menu.play.field.playerId")}</label>
                    
                    <input type="text" value={this.state.selectedPlayerId??""} 
                        onChange={(e)=>{this.setState({selectedPlayerId: e.target.value})}}
                        onKeyUp={(e)=>{
                            if(e.key === 'Enter') {
                                this.rejoinGameButton(this.state.selectedRoomCode??"", this.state.selectedPlayerId??"");
                            }
                        }}
                    />
                    <button onClick={()=>{this.rejoinGameButton(this.state.selectedRoomCode??"", this.state.selectedPlayerId??"")}}>
                        {translate("menu.play.button.rejoin")}
                    </button>
                </section>

                <button onClick={()=>{this.hostGameButton()}}>
                    {translate("menu.play.button.host")}
                </button>
                <button onClick={()=>{this.refreshButton()}}>
                    {translate("menu.play.button.refresh")}
                </button>
            </div>
        </div>
    }
}