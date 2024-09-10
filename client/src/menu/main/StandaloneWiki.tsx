import React, { ReactElement } from "react";
import Wiki from "../../components/Wiki";
import translate from "../../game/lang";
import "./standaloneWiki.css";
import { WikiArticleLink } from "../../components/WikiArticleLink";

export default function StandaloneWiki(props: Readonly<{
    initialWikiPage?: WikiArticleLink
}>): ReactElement {
    return <div className="standalone-wiki wiki-menu-colors">
        <header>
            <h2>{translate("menu.wiki.title")}</h2>
        </header>
        <div>
            <Wiki 
                initialWikiPage={props.initialWikiPage}
                onPageChange={page => {
                    if (page !== null) {
                        window.history.replaceState({}, '', `/wiki/${page}`)
                    } else {
                        window.history.replaceState({}, '', `/wiki`)
                    }
                }}
            />
        </div>
    </div>
}