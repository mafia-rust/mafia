import { ReactElement } from "react";
import WikiSearch from "../../components/Wiki";
import React from "react";
import translate from "../../game/lang";
import "./standaloneWiki.css";

export default function StandaloneWiki(): ReactElement {
    return <div className="hero">
        <div className="standalone-wiki">
            <header>
                <h2>{translate("menu.wiki.title")}</h2>
            </header>
            
            <WikiSearch/>
        </div>
    </div>
}