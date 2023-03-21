import React from "react";
import "../index.css"
import { translate } from "../game/lang.js";

export const enum Type {
    Host = "host",
    Join = "join",
    Default = "default"
}

export function create(value: Type = Type.Default) {
    return <div className="header" style={{height: "100%"}}>
    <h1 className="header-text">
        {translate("menu.loading." + value)}
    </h1>
</div>}