import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate, { translateChecked } from "../../../../game/lang"
import { StateEventType } from "../../../../game/gameManager.d"
import "./largeWizardMenu.css"


export type Wizard = {
    role: "wizard",
    level: number,
    selectedSpell: WizardSpells::Spell

}

export const WIZARD_SPELLS = [
    "none", ""
];

export type WizardSpell = typeof WIZARD_SPELLS[number];

type LargeWizardMenuProps = {
}
type LargeWizardMenuState = {
    gameState: GameState
    localWizardSpell: WizardSpell
}
export default class LargeWizardMenu extends React.Component<LargeWizardMenuProps, LargeWizardMenuState> {
    listener: (type?: StateEventType) => void;
    constructor(props: LargeWizardMenuState) {
        super(props);

        let defaultSpell: WizardSpell
        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.roleState?.role === "wizard"){
            defaultSpell = GAME_MANAGER.state.roleState.guesses;
        }else{
            defaultSpell = "none"
        }
        
        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                gameState : GAME_MANAGER.state,
                localWizardGuesses: defaultGuess
            };
        this.listener = (type)=>{
            if(GAME_MANAGER.state.stateType === "game")
                this.setState({
                    gameState: GAME_MANAGER.state
                });
            if(GAME_MANAGER.state.stateType === "game" && type==="yourRoleState" && GAME_MANAGER.state.roleState?.role === "wizard"){
                this.setState({
                    localWizardGuesses: GAME_MANAGER.state.roleState.guesses
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
        [number, WizardGuess],
        [number, WizardGuess],
        [number, WizardGuess]
    ]){
        this.setState({
            localWizardGuesses: guess
        });
        GAME_MANAGER.sendSetWizardGuess(guess);
    }

    renderSpellPicker(index: number){
        //TODO make so cant guess the same player twice without causing weird problems with HTML

        //Cant guess a player 2x
        //cant guess yourself
        //cant guess a dead player (keep in mind a player could possibly appear dead when their not, so maybe keep it so you can guess dead players?)

        let playerOptions: JSX.Element[] = [];
        for(let i = 0; i < this.state.gameState.players.length; i++){
            playerOptions.push(<option key={i} value={i}>{this.state.gameState.players[i].toString()}</option>);
        }
        let wizardGuessOptions: JSX.Element[] = [];
        for(let i = 0; i < DOOMSAYER_GUESSES.length; i++){
            wizardGuessOptions.push(<option key={i} value={DOOMSAYER_GUESSES[i]}>{wizardGuessTranslate(DOOMSAYER_GUESSES[i])}</option>);
        }
        return <div>
            <select
                value={this.state.localWizardGuesses[index][0]}
                onChange={(e)=>{
                    let newGuess = this.state.localWizardGuesses;
                    newGuess[index][0] = parseInt(e.target.options[e.target.selectedIndex].value);
                    this.sendAndSetGuesses(newGuess);
                }}
            >{playerOptions}</select>
            <select
                value={this.state.localWizardGuesses[index][1]}
                onChange={(e)=>{
                    let newGuess = this.state.localWizardGuesses;
                    newGuess[index][1] = e.target.options[e.target.selectedIndex].value as WizardGuess;
                    this.sendAndSetGuesses(newGuess);
                }}
            >{wizardGuessOptions}</select>
        </div>
    }
    render(){
        return <div className="large-wizard-menu">
            {this.renderGuessPicker(0)}
            {this.renderGuessPicker(1)}
            {this.renderGuessPicker(2)}
        </div>
    }
}