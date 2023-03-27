import React from "react";
import "@/index.css"
import translate from "@game/lang";

export const enum Type {
    Host = "host",
    Join = "join",
    Default = "default"
}
//uses the index.css files' loader class to create loading dots after 
//the text passed from langon
export function create(value: Type = Type.Default) {
    return <div className="header" style={{height: "100%"}}>
    <h1 className="header-text">
        {translate("menu.loading." + value)}
        <br></br>
        <br></br>
        <span className="loading-dots"></span>
    </h1>
</div>}
