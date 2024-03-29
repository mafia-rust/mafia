import React from "react";
import { ReactElement } from "react";



export default function PhaseStartedScreen(props: {
    titleText: string,
    subtitleText: string
}): ReactElement {

    return (
        <div className="phase-started-screen">
            <div className="header">
                <h1>{props.titleText}</h1>
            </div>
            <div className="content">
                <p>{props.subtitleText}</p>
            </div>
        </div>
    );
}