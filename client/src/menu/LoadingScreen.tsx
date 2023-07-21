import React from "react";
import "../index.css"
import "./loadingScreen.css"
import translate from "../game/lang";

export type Type = "default" | "host" | "join"

export function create(value: Type = "default") {
    return <div className="loading">
        <h1>{translate("menu.loading." + value)}</h1>
        <div className="loading-dots"></div>
    </div>;
}
