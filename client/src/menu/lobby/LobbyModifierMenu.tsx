import React, { useEffect, useState } from "react";
import { ReactElement } from "react";
import translate from "../../game/lang";
import { MODIFIERS, Modifier } from "../../game/gameState.d";
import GAME_MANAGER from "../..";
import { StateEventType, StateListener } from "../../game/gameManager.d";



export default function LobbyModifierMenu(props: {
    isHost: boolean;
}) : ReactElement {

    const [modifiers, setModifiers] = useState<Modifier[]>(
        GAME_MANAGER.state.stateType === "lobby" ? GAME_MANAGER.state.modifiers : [] as Modifier[]
    );

    useEffect(() => {
        const listener: StateListener = (type?: StateEventType) => {
            if (type === "modifiers" && GAME_MANAGER.state.stateType === "game") {
                setModifiers(GAME_MANAGER.state.modifiers);
            }
        };

        GAME_MANAGER.addStateListener(listener);

        return ()=>{GAME_MANAGER.removeStateListener(listener)};
    }, []);

    let gameStateModifiers = (GAME_MANAGER.state.stateType === "lobby") ? 
        GAME_MANAGER.state.modifiers : undefined;

    useEffect(() => {
        if(GAME_MANAGER.state.stateType === "lobby")
            setModifiers(GAME_MANAGER.state.modifiers);
    }, [gameStateModifiers]);

    const addModifier = () => {
        let newModifier = MODIFIERS.find(modifier => !modifiers.includes(modifier)) as Modifier;
        if (!newModifier){
            return;
        }
        const newModifiers = [...new Set([...modifiers, newModifier])] as Modifier[];
        setModifiers(newModifiers);
        GAME_MANAGER.sendSetModifiersPacket(newModifiers);
    }
    const removeModifier = (index: number) => {
        const newModifiers = modifiers.slice(0, index).concat(modifiers.slice(index + 1));
        setModifiers(newModifiers);
        GAME_MANAGER.sendSetModifiersPacket(newModifiers);
    }
    const changeModifier = (index: number, newModifier: Modifier) => {
        modifiers[index] = newModifier;
        const newModifiers = [...new Set(modifiers)]
        setModifiers(newModifiers);
        GAME_MANAGER.sendSetModifiersPacket(modifiers);
    }

    return <section className="will-menu-colors">
        <h2>{translate("wiki.article.standard.modifiers.title")}</h2>
        <div>
            {modifiers.map((modifier, index) => {
                return <div>
                    <select
                        disabled={!props.isHost}
                        key={index}
                        value={modifier}
                        onChange={(e) => {
                            changeModifier(index, e.target.value as Modifier);
                        }}
                    >
                        {MODIFIERS.map((modifier) => {
                            return <option value={modifier}>{translate(modifier)}</option>;
                        })}
                    </select>
                    <button
                        disabled={!props.isHost}
                        onClick={()=>{
                            removeModifier(index);
                        }}
                    >
                        {translate("sub")}
                    </button>
                </div>;
            })}
            <button
                disabled={!props.isHost}
                onClick={addModifier}
            >
                {translate("add")}
            </button>
        </div>
        
    </section>;
}