import React from "react";
import translate from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./playerListMenu.css"
import "./gameScreen.css"
import ChatMenu from "./ChatMenu";

export default class PlayerListMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : GAME_MANAGER.gameState,
            hideDead: false,
        };
        this.listener = ()=>{
            this.setState({
                gameState: GAME_MANAGER.gameState
            })
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    renderPhaseSpecific(){
        switch(this.state.gameState.phase){
            case"Voting":
                let votedString = "";
                if(this.state.gameState.voted!=null){
                    votedString = this.state.gameState.players[this.state.gameState.voted].name;
                    return(<div>
                        <div>{votedString}</div>
                        <button className="button gm-button" onClick={()=>{
                            GAME_MANAGER.sendVotePacket(null);
                        }}>Reset Vote</button>
                    </div>);
                }
                return null;
            case"Night":
                let targetStringList = [];
                for(let i = 0; i < this.state.gameState.targets.length; i++){
                    targetStringList.push(GAME_MANAGER.getPlayer(this.state.gameState.targets[i]).toString());
                }

                if(targetStringList.length>0){
                    return(<div>
                        <div>{targetStringList.join(", ")+"."}</div>
                        <button className="button gm-button" onClick={()=>{
                            GAME_MANAGER.sendTargetPacket([]);
                        }}>Reset Targets</button>
                    </div>);
                }
                return null;
            default:
                return null;
        }
    }

    renderPlayer(player, playerIndex){
        
        let canWhisper = this.state.gameState.phase !== "Night" && GAME_MANAGER.gameState.phase !== null && this.state.gameState.myIndex !== playerIndex;

        // let buttonCount = player.buttons.dayTarget + player.buttons.target + player.buttons.vote + canWhisper;

        return(<div key={playerIndex}>
            {GAME_MANAGER.getPlayer(playerIndex).toString()}<br/>

            <div>
                {((player)=>{if(player.buttons.target){return(<button className="button gm-button" onClick={()=>{
                        GAME_MANAGER.sendTargetPacket([...GAME_MANAGER.gameState.targets, playerIndex]);
                    }}
                >{translate("button.Target")}</button>)}})(player)}
                {((player)=>{if(player.buttons.vote){return(<button className="button gm-button" onClick={()=>{GAME_MANAGER.sendVotePacket(playerIndex)}}
                >{translate("button.Vote")}</button>)}})(player)}
                {((player)=>{if(player.buttons.dayTarget){return(<button className="button gm-button" onClick={()=>{GAME_MANAGER.sendDayTargetPacket(playerIndex)}}
                >{translate("button.DayTarget")}</button>)}})(player)}
                {((player)=>{if(canWhisper){return(<button className="button gm-button" onClick={()=>{
                    ChatMenu.prependWhisper(playerIndex)
                }}>{translate("button.Whisper")}</button>)}})(player)}

                <div></div>
            </div>

            
        </div>)
    }
    renderPlayers(players){return<div>
        {
            players.map((player, playerIndex)=>{
                if(!this.state.hideDead || player.alive){
                    return this.renderPlayer(player, playerIndex);
                }
                return null;
            }, this)
        }
    </div>}

    render(){return(<div>
        
        {translate("menu.playerList.title")}

        <button className="button gm-button" onClick={()=>{
            this.setState({
                hideDead: !this.state.hideDead
            })
        }}>LANG TODO hide dead</button>
        <br/>
        <br/>
        {this.renderPhaseSpecific()}
        <br/>
        {this.renderPlayers(this.state.gameState.players)}
    </div>)}
}
