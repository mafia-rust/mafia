import React, { ReactElement } from "react"
import "./largeDoomsayerMenu.css"
import translate, { translateChecked } from "../../../../../game/lang";
import GAME_MANAGER from "../../../../..";
import { RoleState } from "../../../../../game/roleState.d";
import PlayerOptionDropdown from "../../../../../components/PlayerOptionDropdown";
import { PlayerIndex } from "../../../../../game/gameState.d";
import Select from "../../../../../components/Select";
import StyledText from "../../../../../components/StyledText";


export type Doomsayer = {
    type: "doomsayer",
    guesses: DoomsayerGuesses
}

export const DOOMSAYER_GUESSES = [
    "nonTown",
    "jailor", "villager",  
    "doctor",  "bodyguard",  "cop", "bouncer", "engineer", "armorsmith", "steward",
    "vigilante",  "veteran", "marksman", "deputy", "rabblerouser",
    "escort",  "medium",  "retributionist", "reporter", "mayor",  "transporter", "porter", "polymath"
];

export type DoomsayerGuess = typeof DOOMSAYER_GUESSES[number];
export function doomsayerGuessTranslate(doomsayerGuess: DoomsayerGuess): string{
    let outString = translateChecked("role."+doomsayerGuess+".name");
    if(outString===null){
        outString = translate(doomsayerGuess);
    }
    return outString;
}

type DoomsayerGuesses = [
    [number, DoomsayerGuess],
    [number, DoomsayerGuess],
    [number, DoomsayerGuess]
];
export function DoomsayerMenu(props: {
    roleState: RoleState & {type: "doomsayer"}
}): ReactElement {
    return <div className="large-doomsayer-menu">
        <DoomsayerGuessPicker index={0} roleState={props.roleState}/>
        <DoomsayerGuessPicker index={1} roleState={props.roleState}/>
        <DoomsayerGuessPicker index={2} roleState={props.roleState}/>
    </div>
}
function DoomsayerGuessPicker(props: {index: number, roleState: RoleState & {type: "doomsayer"}}): ReactElement {
    function sendGuess(guess: DoomsayerGuesses){
        GAME_MANAGER.sendSetDoomsayerGuess(guess);
    }
    
    // const doomsayerGuessOptions = DOOMSAYER_GUESSES.map((guessString, i)=>
        
    // );

    const doomsayerGuessOptionsMap = new Map();
    for(const guess of DOOMSAYER_GUESSES){
        doomsayerGuessOptionsMap.set(guess, <div key={guess}><StyledText noLinks={true}>{doomsayerGuessTranslate(guess)}</StyledText></div>)
    }

    return <div>
        <PlayerOptionDropdown
            value={props.roleState.guesses[props.index][0]}
            canChooseNone={false}
            onChange={(player: PlayerIndex | null) => {
                let newGuess: DoomsayerGuesses = [...props.roleState.guesses];
                newGuess[props.index][0] = player as PlayerIndex;
                sendGuess(newGuess);
            }}
        />
        <Select
            value={props.roleState.guesses[props.index][1]}
            onChange={(e)=>{
                let newGuess: DoomsayerGuesses = [...props.roleState.guesses];
                newGuess[props.index][1] = e as DoomsayerGuess;
                sendGuess(newGuess);
            }}
            optionsNoSearch={doomsayerGuessOptionsMap}
        />
    </div>
}