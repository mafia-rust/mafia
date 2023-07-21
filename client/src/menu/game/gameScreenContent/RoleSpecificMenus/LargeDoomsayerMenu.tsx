import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate, { translateChecked } from "../../../../game/lang"
import { StateEventType } from "../../../../game/gameManager.d"


export type Doomsayer = {
    role: "doomsayer",
    guesses: [
        [number, DoomsayerGuess],
        [number, DoomsayerGuess],
        [number, DoomsayerGuess]
    ]
}

export const DOOMSAYER_GUESSES = [
    "mafia", "neutral",
    "jailor",  "mayor",  "transporter", 
    "doctor",  "bodyguard",  "crusader",
    "vigilante",  "veteran",  "deputy",
    "escort",  "medium",  "retributionist"
];

export type DoomsayerGuess = typeof DOOMSAYER_GUESSES[number];
export function doomsayerGuessTranslate(doomsayerGuess: DoomsayerGuess): string{
    let outString = translateChecked("role."+doomsayerGuess+".name");
    if(outString===null){
        outString = translate("faction."+doomsayerGuess);
    }
    return outString;
}

type LargeDoomsayerMenuProps = {
}
type LargeDoomsayerMenuState = {
    gameState: GameState
    localDoomsayerGuesses: [
        [number, DoomsayerGuess],
        [number, DoomsayerGuess],
        [number, DoomsayerGuess]
    ]
}
export default class LargeDoomsayerMenu extends React.Component<LargeDoomsayerMenuProps, LargeDoomsayerMenuState> {
    listener: (type?: StateEventType) => void;
    constructor(props: LargeDoomsayerMenuState) {
        super(props);

        let defaultGuess: [
            [number, DoomsayerGuess],
            [number, DoomsayerGuess],
            [number, DoomsayerGuess]
        ];
        if(GAME_MANAGER.gameState.roleState?.role === "doomsayer"){
            defaultGuess = GAME_MANAGER.gameState.roleState.guesses;
        }else{
            defaultGuess = [
                [0, "neutral"],
                [0, "neutral"],
                [0, "neutral"]
            ];
        }
        

        this.state = {
            gameState : GAME_MANAGER.gameState,
            localDoomsayerGuesses: defaultGuess
        };
        this.listener = (type)=>{
            this.setState({
                gameState: GAME_MANAGER.gameState
            });
            if(type==="yourRoleState" && GAME_MANAGER.gameState.roleState?.role === "doomsayer"){
                this.setState({
                    localDoomsayerGuesses: GAME_MANAGER.gameState.roleState.guesses
                });
            }
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    sendAndSetGuesses(guess: [
        [number, DoomsayerGuess],
        [number, DoomsayerGuess],
        [number, DoomsayerGuess]
    ]){
        this.setState({
            localDoomsayerGuesses: guess
        });
        GAME_MANAGER.sendSetDoomsayerGuess(guess);
    }

    renderGuessPicker(index: number){
        //TODO make so cant guess the same player twice without causing weird problems with HTML

        //Cant guess a player 2x
        //cant guess yourself
        //cant guess a dead player (keep in mind a player could possibly appear dead when their not, so maybe keep it so you can guess dead players?)

        let playerOptions: JSX.Element[] = [];
        for(let i = 0; i < this.state.gameState.players.length; i++){
            playerOptions.push(<option key={i}>{this.state.gameState.players[i].toString()}</option>);
        }
        let doomsayerGuessOptions: JSX.Element[] = [];
        for(let i = 0; i < DOOMSAYER_GUESSES.length; i++){
            doomsayerGuessOptions.push(<option key={i}>{doomsayerGuessTranslate(DOOMSAYER_GUESSES[i])}</option>);
        }
        return <div>
            <select
                value={this.state.gameState.players[this.state.localDoomsayerGuesses[index][0]].toString()}
                onChange={(e)=>{
                    let newGuess = this.state.localDoomsayerGuesses;
                    newGuess[index][0] = e.target.selectedIndex;
                    this.sendAndSetGuesses(newGuess);
                }}
            >{playerOptions}</select>
            <select
                value={doomsayerGuessTranslate(this.state.localDoomsayerGuesses[index][1])}
                onChange={(e)=>{
                    let newGuess = this.state.localDoomsayerGuesses;
                    newGuess[index][1] = DOOMSAYER_GUESSES[e.target.selectedIndex];
                    this.sendAndSetGuesses(newGuess);
                }}
            >{doomsayerGuessOptions}</select>
        </div>
    }
    render(){
        return <div>
            {this.renderGuessPicker(0)}
            {this.renderGuessPicker(1)}
            {this.renderGuessPicker(2)}
        </div>
    }
}