import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import "./largeJournalistMenu.css"

type LargeJournalistMenuProps = {
}
type LargeJournalistMenuState = {
    syncedPublic: boolean,
    localJournal: string,
    syncedJournal: string,
}
export default class LargeJournalistMenu extends React.Component<LargeJournalistMenuProps, LargeJournalistMenuState> {
    listener: () => void;
    constructor(props: LargeJournalistMenuState) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.roleState?.role === "journalist")
            this.state = {
                syncedPublic: GAME_MANAGER.state.roleState?.public,
                localJournal: GAME_MANAGER.state.roleState?.journal,
                syncedJournal: GAME_MANAGER.state.roleState?.journal,
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game"){
                if(GAME_MANAGER.state.roleState?.role === "journalist"){
                    this.setState({
                        syncedJournal: GAME_MANAGER.state.roleState.journal,
                        syncedPublic: GAME_MANAGER.state.roleState.public,
                    })
                }
            }
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    handlePublicToggle(){
        GAME_MANAGER.sendSetJournalistJournalPublic(
            !this.state.syncedPublic
        );
    }
    handleSave(){
        GAME_MANAGER.sendSetJournalistJournal(
            this.state.localJournal,
        );
    }

    render(){
        return <div className="large-journalist-menu">
            <div>
                {translate("role.journalist.menu.journal")}
                <button
                    className={"material-icons-round " + (this.state.syncedJournal !== this.state.localJournal ? "highlighted" : "")}
                    onClick={() => this.handleSave()}
                >
                    save
                </button>
            </div>
            <div>
                {translate("role.journalist.menu.public")}
                <label className="material-icons-round" onClick={()=>this.handlePublicToggle()}>
                    {this.state.syncedPublic ? "check" : "close"}
                </label>
            </div>
            <textarea
                value={this.state.localJournal}
                onChange={(e) => {
                    this.setState({ localJournal: e.target.value });
                }}
                onKeyDown={(e) => {
                    if (e.ctrlKey) {
                        if (e.key === 's') {
                            e.preventDefault();
                            this.handleSave();
                        } else if (e.key === "Enter") {
                            this.handleSave();
                        }
                    }
                }}>
            </textarea>
        </div>
    }
}