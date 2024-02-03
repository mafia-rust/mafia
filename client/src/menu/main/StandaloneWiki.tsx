import { ReactElement } from "react";
import WikiSearch, { WikiPage } from "../../components/WikiSearch";
import React from "react";
import translate from "../../game/lang";
import "./standaloneWiki.css";

export default function StandaloneWiki(props: { page?: WikiPage }): ReactElement {
    return <div className="hero">
        <div className="standalone-wiki">
            <header>
                <h2>{translate("menu.wiki.title")}</h2>
            </header>
            
            <WikiSearch page={props.page}/>
        </div>
    </div>
}