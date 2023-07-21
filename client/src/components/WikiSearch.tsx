import React from "react";
import ROLES from "./../resources/roles.json";
import translate, { langText } from "../game/lang";
import "./wikiSearch.css";
import { Role } from "../game/roleState.d";
import { FactionAlignment, getRoleListEntryFromFactionAlignment, translateRoleListEntry } from "../game/roleListState.d";
import StyledText from "../components/StyledText";
import { HistoryQueue } from "../history";
import { regEscape } from "..";

type WikiSearchState = ({
    type: "search"
} | {
    type: "page",
    page: WikiPage,
}) & {
    searchQuery: string,
}
type WikiPage = {
    type: "role", role: Role
} | {
    type: "article", article: Article
}
export const ARTICLES = ["help", "roles_and_teams", "phases_and_timeline", "controls", "wills_and_notes", "visit", "all_language"] as const;
export type Article = typeof ARTICLES[number];

export default class WikiSearch extends React.Component<{}, WikiSearchState> {
    history: HistoryQueue<WikiSearchState> = new HistoryQueue(10);
    constructor(props: {}) {
        super(props);

        this.state = {
            type: "search",
            searchQuery: "",
        };
    }
    componentDidMount() {
    }
    componentWillUnmount() {
    }
    getGeneralPageFromSearch(search: string): Article[] {
        return ARTICLES.filter(article => 
            RegExp(regEscape(search), 'i').test(translate("menu.wiki.entries."+article+".title")) 
            || RegExp(regEscape(search), 'i').test(translate("menu.wiki.entries."+article+".text"))
        )
    }
    renderGeneralWikiPage(article: string){
        if(article==="all_language"){
            return <>{langText}</>;
        }
        return <StyledText className="wiki-content-body">{`
# ${translate("menu.wiki.entries."+article+".title")}
<br/>

${translate("menu.wiki.entries."+article+".text")}
        `}</StyledText>
    }
    getRolesFromSearch(search: string): Role[] {
        return Object.keys(ROLES).filter(role => 
            RegExp(regEscape(search), 'i').test(translate("role."+role+".name")) 
            || RegExp(regEscape(search), 'i').test(translate("role."+role+".abilities"))
            || RegExp(regEscape(search), 'i').test(translate("role."+role+".attributes"))
        ) as Role[]
    }
    renderRoleWikiPage(role: Role){
        let defenseString = "";
        switch(ROLES[role].defense){
            case 0:
                defenseString = translate("none");
                break;
            case 1:
                defenseString = translate("basic");
                break;
            case 2:
                defenseString = translate("powerful");
                break;
            case 3:
                defenseString = translate("invincible");
                break;
        }

        const roleData = ROLES[role as keyof typeof ROLES];

        return <StyledText className="wiki-content-body">{`
# ${translate("role."+role+".name")}
### ${translateRoleListEntry(getRoleListEntryFromFactionAlignment(roleData.factionAlignment as FactionAlignment))}
<br/>

### ${translate("menu.wiki.abilities")}
${translate("role."+role+".abilities")}
### ${translate("menu.wiki.attributes")}
${translate("role."+role+".attributes")}

<br/>

### ${roleData.maxCount === null ? '' : translate("menu.wiki.maxCount", roleData.maxCount)}
### ${translate("menu.wiki.suspicious", ROLES[role as keyof typeof ROLES].suspicious?"suspicious":"innocent")}
### ${translate("menu.wiki.defense", defenseString)}`}
        </StyledText>
    }

    renderOpenPageButton(page: WikiPage) {
        let key, text;
        if (page.type === "role") {
            key = page.role;
            text = translate("role."+page.role+".name");
        } else {
            key = page.article;
            text = translate("menu.wiki.entries."+page.article+".title");
        }
        
        return <button key={key} 
            onClick={()=>{
                this.history.push(this.state);
                this.setState({
                    type: "page",
                    searchQuery: this.state.searchQuery,
                    page
                });
            }}
        >
            <StyledText>{text}</StyledText>
        </button>
    }
    
    renderWikiPageOrSearch(){
        if(this.state.type === "page"){
            if (this.state.page.type === "role") {
                return this.renderRoleWikiPage(this.state.page.role);
            } else {
                return this.renderGeneralWikiPage(this.state.page.article);
            }
        } else {
            return this.getRolesFromSearch(this.state.searchQuery)
                .map(role => this.renderOpenPageButton({ type: "role", role }))
                .concat(
                    this.getGeneralPageFromSearch(this.state.searchQuery)
                        .map(article => this.renderOpenPageButton({ type: "article", article }))
                );
        }
    }

    eitherContains(string1: string, string2: string): boolean{
        return string1.toLowerCase().includes(string2.toLowerCase()) ||
        string2.toLowerCase().includes(string1.toLowerCase())
    }

    renderSearchBar() {
        return <div className="wiki-search-bar">
            {this.history.length() !== 0 ? 
                <button
                    className="material-icons-round"
                    onClick={() => {this.setState(this.history.pop()!)}}
                    aria-label={translate("menu.wiki.search.back")}
                >
                    arrow_back
                </button> 
            : null}
            <input type="text" value={this.state.searchQuery}
                onChange={(e)=>{
                    if (this.state.type === "page" || this.history.length() === 0) {
                        this.history.push(this.state);
                    }
                    this.setState({type: "search", searchQuery: e.target.value})
                }}
                placeholder={translate("menu.wiki.search.placeholder")}
            />
            {this.state.searchQuery && <button 
                tabIndex={-1}
                className="material-icons-round clear"
                onClick={() => {
                    this.history.push(this.state)
                    this.setState({ type: "search", searchQuery: "" });
                }}
                aria-label={translate("menu.wiki.search.clear")}
            >
                backspace
            </button>}
        </div>
    }
    
    render() {
        return (<div className="wiki-search">
        {this.renderSearchBar()}
        <div className="wiki-results">
            {this.renderWikiPageOrSearch()}
        </div>
    </div>);}
}
