import { ReactElement, useEffect } from "react";
import "./spectatorGameScreen.css";
import React from "react";
import PhaseStartedScreen from "./PhaseStartedScreen";
import GAME_MANAGER from "../..";
import { StateEventType, StateListener } from "../../game/gameManager.d";
import SpectatorHeader from "./SpectatorHeader";
import SpectatorBody from "./SpectatorBody";
import translate from "../../game/lang";



/*
    briefing - Click the fast forward button to vote to start the game
    dusk (grave as subtitle if possible from final words)
        (Chat, rolelist, playerlist (if possible, sorted by verdict) )
    night - use your ability
        (Playerlist)
    obituary
        (Shows graves)
    discussion
        (Shows chat, playerlist, rolelist)
    nomination - 7 votes required, 3 trials remain
        (Shows chat, rolelist, and playerlist (with vote bars))
    testimony - Sammy is on trial
        (shows chat, rolelist, and playerlist (says if they voted))
    judgement - Sammy is on trial
        (shows chat, rolelist, and playerlist (says if they voted))
    final words - Sammy will be executed 
        (shows chat, rolelist, and playerlist (sorted by verdict))
*/

const DEFAULT_START_PHASE_SCREEN_TIME = 3;

export default function SpectatorGameScreen (props: {}): ReactElement {


    const [phase, setPhase] = React.useState(()=>{
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.phaseState : {type:"briefing" as "briefing"}
    });
    const [timeLeftMs, setTimeLeftMs] = React.useState(()=>{
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.timeLeftMs : 0
    });
    const [fastForward, setFastForward] = React.useState(()=>{
        return GAME_MANAGER.state.stateType==="game" ? GAME_MANAGER.state.fastForward : false
    });

    useEffect(() => {
        const listener: StateListener = (type?: StateEventType) => {

            if(GAME_MANAGER.state.stateType !== "game") return;

            switch (type) {
                case "yourVoteFastForwardPhase":
                    setFastForward(GAME_MANAGER.state.fastForward);
                    break;
                case "phase":
                    setPhase(GAME_MANAGER.state.phaseState);
                    break;
                case "phaseTimeLeft":
                case "tick":
                    setTimeLeftMs(GAME_MANAGER.state.timeLeftMs);
                    break;
            }
        };
        GAME_MANAGER.addStateListener(listener);
        return () => GAME_MANAGER.removeStateListener(listener);
    }, [
        setPhase,
        setTimeLeftMs
    ]);

    let showStartedScreen = true;
    let maxTime = 0;
    if(GAME_MANAGER.state.stateType === "game"){
        maxTime = GAME_MANAGER.state.phaseTimes[phase.type];
        let timePassed = Math.floor(maxTime - timeLeftMs/1000);
        showStartedScreen = timePassed < DEFAULT_START_PHASE_SCREEN_TIME;
    }
    if(phase.type === "briefing") showStartedScreen = true;

    return (
        <div className="spectator-game-screen">
            <button 
                onClick={()=>{
                    GAME_MANAGER.sendVoteFastForwardPhase(true);
                }}
                className={"material-icons-round fast-forward-button" + (fastForward ? " highlighted" : "")}
            >
                double_arrow
            </button>
            {showStartedScreen ? 
                <PhaseStartedScreen/>
            : 
                <>
                    <SpectatorHeader phase={phase} timeLeftMs={timeLeftMs} timeBarPercentage={timeLeftMs/(maxTime*1000)}/>
                    <SpectatorBody/>
                </>
            }
        </div>
    );
    
}


export function getTranslatedSubtitle(): string {
    if(GAME_MANAGER.state.stateType !== "game") return "";
    let subtitleText = "";

    switch (GAME_MANAGER.state.phaseState.type) {
        case "briefing":
        case "night":
        case "discussion":
            subtitleText = translate("phase."+GAME_MANAGER.state.phaseState.type+".subtitle");
            break;
        case "nomination":
            if(GAME_MANAGER.state.stateType === "game"){
                let votesRequired = GAME_MANAGER.getVotesRequired();

                if(votesRequired !== null){
                    subtitleText += votesRequired === 1 ? translate("votesRequired.1") : translate("votesRequired", votesRequired);
                }

                subtitleText += " "+translate("trialsRemaining", GAME_MANAGER.state.phaseState.trialsLeft);
            }
            break;
        case "testimony":
        case "judgement":
        case "finalWords":
            if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.phaseState.playerOnTrial !== null){
                subtitleText = translate("phase."+GAME_MANAGER.state.phaseState.type+".subtitle", GAME_MANAGER.getPlayerNames()[GAME_MANAGER.state.phaseState.playerOnTrial].toString());
            }
            break;
        default:
            break;
    }
    return subtitleText;
}