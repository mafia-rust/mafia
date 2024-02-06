import { ReactElement } from "react";
import WikiSearch from "../../components/WikiSearch";
import React from "react";
import translate from "../../game/lang";
import "./standaloneWiki.css";
import { WikiArticleLink } from "../../components/WikiArticle";

export default function StandaloneWiki(props: { page?: WikiArticleLink }): ReactElement {
    return <div className="hero">
        <div className="standalone-wiki">
            <header>
                <h2>{translate("menu.wiki.title")}</h2>
            </header>
            
            <WikiSearch page={props.page}/>
        </div>
    </div>
}