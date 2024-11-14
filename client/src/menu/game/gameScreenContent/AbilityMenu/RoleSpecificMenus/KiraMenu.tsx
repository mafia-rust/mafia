import React, { ReactElement, useEffect, useState } from "react"
import "./kiraMenu.css";
import GAME_MANAGER from "../../../../..";
import translate, { translateChecked } from "../../../../../game/lang";
import StyledText from "../../../../../components/StyledText";
import Select, { SelectOptionsSearch } from "../../../../../components/Select";
import { PlayerIndex } from "../../../../../game/gameState.d";
import { RoleState } from "../../../../../game/roleState.d";

export const KIRA_GUESSES = [
    "none",
    "nonTown",
    "jailor", "villager",  
    "detective", "lookout", "spy", "tracker", "philosopher", "psychic", "auditor", "snoop", "gossip", "tallyClerk",
    "doctor",  "bodyguard",  "cop", "bouncer", "engineer", "armorsmith", "steward",
    "vigilante",  "veteran", "marksman", "deputy", "rabblerouser",
    "escort",  "medium",  "retributionist", "reporter", "mayor",  "transporter",
];
type KiraGuessRecord = Partial<Record<PlayerIndex, KiraGuess>>;
export type KiraInput = [PlayerIndex, KiraGuess][];

export type KiraGuess = typeof KIRA_GUESSES[number];
export function kiraGuessTranslate(kiraGuess: KiraGuess): string{
    let outString = translateChecked("role."+kiraGuess+".name");
    if(outString===null){
        outString = translate(kiraGuess);
    }
    return outString;
}

export type KiraGuessResult = "correct" | "notInGame" | "wrongSpot";

export default function KiraMenu(props: Readonly<{
    roleState: RoleState & {type: "kira"},
}>): ReactElement {


    function sendSetKiraGuess(guesses: KiraGuessRecord){
        let guessesOut: KiraInput = [];
        for(let [player, guess] of Object.entries(guesses)){
            guessesOut.push([Number.parseInt(player), guess?? "none"]);
        }

        GAME_MANAGER.sendAbilityInput({
            type: "kira" as const,
            selection: guessesOut
        });
    }



    let guessPickers: ReactElement[] = [];
    let keys = Object.keys(props.roleState.guesses).map((k)=>{return Number.parseInt(k)}).sort();
    for(let playerIndex of keys){
        guessPickers.push(
            <KiraGuessPicker 
                key={playerIndex} 
                playerIndex={playerIndex} 
                guess={props.roleState.guesses[playerIndex]?? "none"} 
                onChange={(guess: KiraGuess) => {
                    let newGuesses: KiraGuessRecord = {...props.roleState.guesses};
                    newGuesses[playerIndex] = guess;
                    sendSetKiraGuess(newGuesses);
                }}
            />
        );
    }

    return <div className="large-kira-menu">
        {guessPickers}
    </div>
}

function KiraGuessPicker(props: {
    playerIndex: PlayerIndex,
    guess: KiraGuess,
    onChange: (guess: KiraGuess) => void
}): ReactElement {

    const [players, setPlayers] = useState(() => {
        if(GAME_MANAGER.state.stateType === "game")
            return GAME_MANAGER.state.players;
        return [];
    });

    useEffect(()=>{
        const listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game")
                setPlayers(GAME_MANAGER.state.players);
        };

        listener();
        GAME_MANAGER.addStateListener(listener);
        return ()=>GAME_MANAGER.removeStateListener(listener);
    }, [setPlayers]);

    const guessOptions: SelectOptionsSearch<KiraGuess> = new Map();
    for(let guess of KIRA_GUESSES){
        guessOptions.set(guess, [
            <StyledText noLinks={true}>{kiraGuessTranslate(guess)}</StyledText>,
            kiraGuessTranslate(guess)
        ]);
    }

    return <div className="kira-guess-picker">
        <StyledText>{players[props.playerIndex].toString()}</StyledText>
        <Select
            value={props.guess}
            onChange={(e) => props.onChange(e)}
            optionsSearch={guessOptions}
        />
    </div>
}