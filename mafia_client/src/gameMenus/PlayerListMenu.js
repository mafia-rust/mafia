import React from "react";
import gameManager from "../index.js";

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
        
        let player = this.state.gameState.players[playerIndex];
        let canWhisper = this.state.gameState.phase !== "Night" && gameManager.gameState.phase !== null && this.state.gameState.myIndex !== playerIndex;

        // let buttonCount = player.buttons.dayTarget + player.buttons.target + player.buttons.vote + canWhisper;

        return(<div key={playerIndex}>
            {playerIndex}:{player.name}<br/>

            <div style={{
                display: "grid",

                gridAutoColumns: "1fr",

                width: "100%",

                //gridGap: "5px",
            }}>
                {(()=>{if(player.buttons.target){return(<button style={{
                        gridColumn: 2,
                        // overflowX: "hidden",
                    }}
                    onClick={()=>{
                        gameManager.gameState.targets.push(playerIndex);
                        gameManager.target_button(this.state.gameState.targets);
                    }}
                >Target</button>)}})()}
                {(()=>{if(player.buttons.vote){return(<button style={{
                        gridColumn: 3,                    
                        // overflowX: "hidden",
                    }}
                    onClick={()=>{gameManager.vote_button(playerIndex)}}
                >Vote</button>)}})()}
                {(()=>{if(player.buttons.dayTarget){return(<button style={{
                        gridColumn: 4,                    
                        // overflowX: "hidden",
                    }}
                    onClick={()=>{gameManager.dayTarget_button(playerIndex)}}
                >DayTarget</button>)}})()}
                {(()=>{if(canWhisper){return(<button style={{
                    gridColumn: 5,                    
                    // overflowX "hidden",
                }}>Whisper</button>)}})()}

                <div style={{
                    gridColumn: 6,                    
                    // overflowX "hidden",
                }}></div>
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