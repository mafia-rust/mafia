import React from "react";
import "../index.css"
import "./loadingScreen.css"
import translate from "../game/lang";

export type Type = "default" | "host" | "join"
//uses the index.css files' loader class to create loading dots after 
//the text passed from langon
export function create(value: Type = "default") {
    return <div className="loading">
        <h1>{translate("menu.loading." + value)}</h1>
        <div className="loading-dots"></div>
    </div>;
}
