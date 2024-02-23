import { ReactElement } from "react";
import Wiki from "../../components/Wiki";
import React from "react";
import translate from "../../game/lang";
import "./standaloneWiki.css";

export default function StandaloneWiki(): ReactElement {
    return <div className="standalone-wiki">
        <header>
            <h2>{translate("menu.wiki.title")}</h2>
        </header>
        <div>
            <Wiki/>
        </div>
    </div>
}