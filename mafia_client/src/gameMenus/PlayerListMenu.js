import React from "react";
import gameManager from "../game/gameManager";

export class PlayerListMenu extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            gameState : gameManager.gameState,

            listener : {func : ()=>{
                this.setState({
                    gameState: gameManager.gameState
                })
            }},
        };
                
    }

    componentDidMount() {
        gameManager.addStateListner(this.state.listener);
    }
    componentWillUnmount() {
        gameManager.removeStateListner(this.state.listener);
    }

    renderPlayer(playerIndex){
        
        let player = gameManager.gameState.players[playerIndex];
        let canWhisper = gameManager.gameState.phase !== "Night" && gameManager.gameState.phase !== null;

        let buttonCount = player.buttons.dayTarget + player.buttons.target + player.buttons.vote + canWhisper;

        return(<div key={playerIndex}>
            {playerIndex}:{this.state.gameState.players[playerIndex].name}<br/>

            <div style={{
                display: "grid",

                gridAutoColumns: "1fr",

                width: "100%",

                //gridGap: "5px",
            }}>
                {(()=>{if(player.buttons.target){<button style={{
                    gridColumn: 1,                    
                    // overflowX: "hidden",
                }}>Target</button>}})()}
                {(()=>{if(player.buttons.vote){<button style={{
                    gridColumn: 2,                    
                    // overflowX: "hidden",
                }}>Vote</button>}})()}
                {(()=>{if(player.buttons.dayTarget){<button style={{
                    gridColumn: 3,                    
                    // overflowX: "hidden",
                }}>DayTarget</button>}})()}
                {(()=>{if(player.buttons.canWhisper){<button style={{
                    gridColumn: 4,                    
                    // overflowX: "hidden",
                }}>Whisper</button>}})()}
            </div>

            
        </div>)
    }
    renderPlayers(){return<div>
        {
            this.state.gameState.players.map((player, index)=>{
                return this.renderPlayer(index);
            }, this)
        }
        {/* {this.renderPlayer("Cotton Mather")}
        <br/>
        {this.renderPlayer("Johnathan Williams")}
        <br/>
        {this.renderPlayer("Sammy")}
        <br/> */}
    </div>}

    render(){return(<div>
        {this.renderPlayers()}
    </div>)}
}