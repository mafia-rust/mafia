import React, { ReactElement, useEffect, useState } from "react"
import "./largeKiraMenu.css";
import GAME_MANAGER from "../../../../..";
import translate, { translateChecked } from "../../../../../game/lang";
import StyledText from "../../../../../components/StyledText";
import Select, { SelectOptionsSearch } from "../../../../../components/Select";
import { PlayerIndex } from "../../../../../game/gameState.d";

export const KIRA_GUESSES = [
    "none",
    "nonTown",
    "jailor", "villager",  
    "detective", "lookout", "spy", "tracker", "philosopher", "psychic", "auditor", "snoop", "gossip", "tallyClerk",
    "doctor",  "bodyguard",  "cop", "bouncer", "engineer", "armorsmith", "steward",
    "vigilante",  "veteran", "marksman", "deputy", "rabblerouser",
    "escort",  "medium",  "retributionist", "reporter", "mayor",  "transporter",
];

export type KiraGuess = typeof KIRA_GUESSES[number];
export function kiraGuessTranslate(kiraGuess: KiraGuess): string{
    let outString = translateChecked("role."+kiraGuess+".name");
    if(outString===null){
        outString = translate(kiraGuess);
    }
    return outString;
}

export type KiraGuessResult = "correct" | "notInGame" | "wrongSpot";

export default function LargeKiraMenu(props: {}): ReactElement {
    
    const [localKiraGuesses, setLocalKiraGuesses] = useState(() => {
        if( GAME_MANAGER.state.stateType === "game" && 
            GAME_MANAGER.state.clientState.type === "player" && 
            GAME_MANAGER.state.clientState.roleState?.type === "kira"
        )
            return GAME_MANAGER.state.clientState.roleState.guesses;
        return {};
    });

    useEffect(()=>{
        const listener = ()=> {
            if( GAME_MANAGER.state.stateType === "game" && 
                GAME_MANAGER.state.clientState.type === "player" && 
                GAME_MANAGER.state.clientState.roleState?.type === "kira"
            )
                setLocalKiraGuesses(GAME_MANAGER.state.clientState.roleState.guesses);
        };

        listener();
        GAME_MANAGER.addStateListener(listener);
        return ()=>GAME_MANAGER.removeStateListener(listener);
    }, [setLocalKiraGuesses]);

    let guessPickers: ReactElement[] = [];
    let keys = Object.keys(localKiraGuesses).map((k)=>{return Number.parseInt(k)}).sort();
    for(let playerIndex of keys){
        guessPickers.push(
            <KiraGuessPicker 
                key={playerIndex} 
                playerIndex={playerIndex} 
                guess={localKiraGuesses[playerIndex]} 
                onChange={(guess) => {
                    let newGuesses = {...localKiraGuesses};
                    newGuesses[playerIndex] = guess;
                    GAME_MANAGER.sendSetKiraGuess(newGuesses);
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