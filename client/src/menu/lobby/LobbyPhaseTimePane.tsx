import React from "react";
import { PhaseTimes } from "../../game/gameState.d";
import GAME_MANAGER from "../../index";
import "./lobbyMenu.css";
import { StateListener } from "../../game/gameManager.d";
import PhaseTimesSelector from "../../components/PhaseTimeSelector";


type PhaseTimePaneState = {
    phaseTimes: PhaseTimes,
    host: boolean
}

export default class LobbyPhaseTimePane extends React.Component<{}, PhaseTimePaneState> {
    listener: StateListener;
    constructor(props: {}) {
        super(props);

        let phaseTimes = {
            morning: 15,
            discussion: 46,
            voting: 30,
            testimony: 24,
            judgement: 20,
            evening: 10,
            night: 37
        };
        if(GAME_MANAGER.state.stateType === "lobby"){
            phaseTimes = GAME_MANAGER.state.phaseTimes;
        }

        this.state = {
            phaseTimes: phaseTimes,
            host: GAME_MANAGER.getMyHost() ?? false
        };

        this.listener = (type)=>{
            if(GAME_MANAGER.state.stateType === "lobby" && (type==="phaseTime" || type==="phaseTimes"))
                this.setState({
                    phaseTimes: GAME_MANAGER.state.phaseTimes
                });
            else if (GAME_MANAGER.state.stateType === "lobby" && type === "playersHost") {
                this.setState({ host: GAME_MANAGER.getMyHost() ?? false });
            }
        }
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }

    render() {return<PhaseTimesSelector disabled={!this.state.host} phaseTimes={this.state.phaseTimes} onChange={(phaseTimes)=>{
        GAME_MANAGER.sendSetPhaseTimesPacket(phaseTimes);
    }}/>}
}
