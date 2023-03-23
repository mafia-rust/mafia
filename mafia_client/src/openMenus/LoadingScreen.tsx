import React from "react";
import "../index.css"
import { translate } from "../game/lang.js";

export const enum Type {
    Host = "host",
    Join = "join",
    Default = "default"
}
//uses the index.css files' loader class to create loading dots after 
//the text "Loading" or "Joining" or "Hosting"
export function create(value: Type = Type.Default) {
    return <div className="header" style={{height: "100%"}}>
    <h1 className="header-text">
        {translate("menu.loading." + value)}
        <br></br>
        <br></br>
        <span className="loading-dots"></span>
    </h1>
</div>}
