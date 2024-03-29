import { ReactElement } from "react";
import "./spectatorGameScreen.css";
import React from "react";
import PhaseStartedScreen from "./PhaseStartedScreen";

export default function SpectatorGameScreen (props: {}): ReactElement {

    let innerContent = <PhaseStartedScreen titleText="Phase Started" subtitleText="The game has started!"/>;

    return (
        <div className="spectator-game-screen">
            {innerContent}
        </div>
    );
}