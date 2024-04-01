import { ReactElement, useEffect } from "react";
import GraveComponent from "../../components/grave";
import React from "react";
import GAME_MANAGER from "../..";
import { StateEventType } from "../../game/gameManager.d";
import translate from "../../game/lang";

export default function ObituaryScreen(props: {}): ReactElement {

    const [graves, setGraves] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.graves : []
    );
    const [dayNumber, setDayNumber] = React.useState(
        GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.dayNumber : 0
    );

    useEffect(() => {
        const listener = (type?: StateEventType) => {
            if(GAME_MANAGER.state.stateType !== "game") return;
            switch(type){
                case "addGrave":
                    if(GAME_MANAGER.state.graves !== null)
                        setGraves(GAME_MANAGER.state.graves);
                    break;
                case "phase":
                    setDayNumber(GAME_MANAGER.state.dayNumber);
                    break;
            }
        };

        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [setGraves]);


    let newGraves = [];
    for(let i = 0; i < graves.length; i++){
        if(graves[i] === null) continue;
        if(graves[i].diedPhase === "night" && graves[i].dayNumber + 1 === dayNumber)
            newGraves.push(
                <GraveComponent key={i} grave={graves[i]} playerNames={GAME_MANAGER.getPlayerNames()}/>
            );
    }

    if(newGraves.length === 0)
        return (
            <div className="obituary-spectator-body">
                <h1>{translate("nobodyDiedLastNight")}</h1>
            </div>
        );
    return (
        <div className="obituary-spectator-body">
            {newGraves}
        </div>
    );
}