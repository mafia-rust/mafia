import React from "react"
import GameState from "../../../../game/gameState.d"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import "./largeMayorMenu.css"

type LargeMayorMenuProps = {
}
type LargeMayorMenuState = {
    gameState: GameState,
    syncedPublic: boolean,
    localJournal: string,
    syncedJournal: string,
}
export default class LargeMayorMenu extends React.Component<LargeMayorMenuProps, LargeMayorMenuState> {
    listener: () => void;
    constructor(props: LargeMayorMenuState) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.roleState?.role === "mayor")
            this.state = {
                gameState : GAME_MANAGER.state,
                syncedPublic: GAME_MANAGER.state.roleState?.public,
                localJournal: GAME_MANAGER.state.roleState?.journal,
                syncedJournal: GAME_MANAGER.state.roleState?.journal,
            };
        this.listener = ()=>{
            if(GAME_MANAGER.state.stateType === "game"){
                this.setState({
                    gameState: GAME_MANAGER.state
                })
                if(GAME_MANAGER.state.roleState?.role === "mayor"){
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
        GAME_MANAGER.sendSetMayorsJournalPublic(
            !this.state.syncedPublic
        );
    }
    handleSave(){
        GAME_MANAGER.sendSetMayorsJournal(
            this.state.localJournal,
        );
    }

    render(){
        return <div className="large-mayor-menu">
            <div>
                {translate("role.mayor.menu.journal")}
                <button
                    className={"material-icons-round " + (this.state.syncedJournal !== this.state.localJournal ? "highlighted" : "")}
                    onClick={() => this.handleSave()}
                >
                    save
                </button>
            </div>
            <div>
                {translate("role.mayor.menu.public")}
                <label className="material-icons-round" onClick={()=>this.handlePublicToggle()}>
                    {this.state.syncedPublic ? "check" : "close"}
                </label>
            </div>
            <textarea
                value={this.state.localJournal}
                onChange={(e) => {
                    let fields = this.state.localJournal;
                    fields = e.target.value;
                    this.setState({ localJournal: fields });
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