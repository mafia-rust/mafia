import React from "react";
import ROLES from "./../resources/roles.json";
import translate from "../game/lang";
import "./wikiSearch.css";
import { Role } from "../game/roleState.d";
import { FactionAlignment, getRoleListEntryFromFactionAlignment, translateRoleListEntry } from "../game/roleListState.d";
import StyledText from "../components/StyledText";
import { HistoryQueue } from "../history";

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
export const ARTICLES = ["help", "roles_and_teams", "phases_and_timeline", "controls", "wills_and_notes", "visit"] as const;
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
        let out: Article[] = [];
        for(let i in ARTICLES){
            let article = ARTICLES[i];
            if(
                this.eitherContains(translate("menu.wiki.entries."+article+".title"), search)
            ){
                if(!out.includes(article as Article)){
                    out.push(article as Article);
                }
            }
        }
        for(let i in ARTICLES){
            let article = ARTICLES[i];
            if(
                this.eitherContains(translate("menu.wiki.entries."+article+".text"), search)
            ){
                if(!out.includes(article as Article)){
                    out.push(article as Article);
                }
            }
        }
        return out
    }
    renderGeneralWikiPage(article: string){
        return <StyledText className="wiki-content-body">{`
# ${translate("menu.wiki.entries."+article+".title")}
<br/>

${translate("menu.wiki.entries."+article+".text")}
        `}</StyledText>
    }
    getRolesFromSearch(search: string): Role[] {
        search = search.toLowerCase();
        let out: Role[] = [];
        for(let role in ROLES){
            if(
                this.eitherContains(translate("role."+role+".name"), search)
            ){
                if(!out.includes(role as Role)){
                    out.push(role as Role);
                }
            }
        }
        for(let role in ROLES){
            if(
                this.eitherContains(translate("role."+role+".abilities"), search)
            ){
                if(!out.includes(role as Role)){
                    out.push(role as Role);
                }
            }
        }
        for(let role in ROLES){
            if(
                this.eitherContains(translate("role."+role+".attributes"), search)
            ){
                if(!out.includes(role as Role)){
                    out.push(role as Role);
                }
            }
        }
        return out;
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
    
    renderWikiPageOrSearch(){
        if(this.state.type === "page"){
            if(this.state.page.type === "role"){
                return this.renderRoleWikiPage(this.state.page.role);
            }else if(this.state.page.type === "article"){
                return this.renderGeneralWikiPage(this.state.page.article);
            }
        }else{

            return this.getRolesFromSearch(this.state.searchQuery).map((role)=>{
                return <button key={role} 
                    onClick={()=>{
                        this.history.push(this.state);
                        this.setState({
                            type: "page",
                            searchQuery: this.state.searchQuery,
                            page: { type: "role", role }
                        });
                    }}
                >
                    <StyledText>
                        {translate("role."+role+".name")}
                    </StyledText>
                </button>
            }).concat(this.getGeneralPageFromSearch(this.state.searchQuery).map((article)=>{
                return <button key={article} 
                    onClick={()=>{
                        this.history.push(this.state);
                        this.setState({
                            type: "page",
                            searchQuery: this.state.searchQuery,
                            page: { type: "article", article }
                        });
                    }}
                >
                    <StyledText>
                        {translate("menu.wiki.entries."+article+".title")}
                    </StyledText>
                </button>
            }));
        }
    }

    eitherContains(string1: string, string2: string): boolean{
        return string1.toLowerCase().includes(string2.toLowerCase()) ||
        string2.toLowerCase().includes(string1.toLowerCase())
    }
    
    render() {
        return (<div className="wiki-search">
        <div className="wiki-search-bar">
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
                className="material-icons-round clear"
                onClick={() => {
                    this.history.push(this.state)
                    this.setState({ type: "search", searchQuery: "" });
                }}
            >
                backspace
            </button>}
        </div>
        <div className="wiki-results">
            {this.renderWikiPageOrSearch()}
        </div>
    </div>);}
}
