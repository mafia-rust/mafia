import React from "react";
import "../index.css"
import "./loadingScreen.css"
import translate from "../game/lang";

// LANG TODO: translate
export const enum Type {
    Host = "host",
    Join = "join",
    Default = "default"
}
//uses the index.css files' loader class to create loading dots after 
//the text passed from langon
export function create(value: Type = Type.Default) {
    return <header style={{height: "100%"}}>
    <h1>
        {translate("menu.loading." + value)}
        <span className="loading-dots"></span>
    </h1>
</header>}
