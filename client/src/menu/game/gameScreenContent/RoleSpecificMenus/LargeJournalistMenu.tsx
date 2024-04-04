import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import "./largeJournalistMenu.css"
import { Button } from "../../../../components/Button"
import Icon from "../../../../components/Icon"

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

        if(
            GAME_MANAGER.state.stateType === "game" && 
            GAME_MANAGER.state.clientState.type === "player" &&
            GAME_MANAGER.state.clientState.roleState?.role === "journalist"
        )
            this.state = {
                syncedPublic: GAME_MANAGER.state.clientState.roleState?.public,
                localJournal: GAME_MANAGER.state.clientState.roleState?.journal,
                syncedJournal: GAME_MANAGER.state.clientState.roleState?.journal,
            };
        this.listener = ()=>{
            if(
                GAME_MANAGER.state.stateType === "game" &&
                GAME_MANAGER.state.clientState.type === "player"
            ){
                if(GAME_MANAGER.state.clientState.roleState?.role === "journalist"){
                    this.setState({
                        syncedJournal: GAME_MANAGER.state.clientState.roleState.journal,
                        syncedPublic: GAME_MANAGER.state.clientState.roleState.public,
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
    handleSend(){
        GAME_MANAGER.sendSendMessagePacket('\n' + this.state.syncedJournal);
    }

    render(){
        return <div className="large-journalist-menu">
            <div>
                {translate("role.journalist.menu.journal")}
                <div>
                    <Button
                        highlighted={this.state.syncedJournal !== this.state.localJournal}
                        onClick={() => {
                            this.handleSave();
                            return true;
                        }}
                    >
                        <Icon>save</Icon>
                    </Button>
                    <Button
                        onClick={() => {
                            this.handleSend();
                            return true;
                        }}
                    >
                        <Icon>send</Icon>
                    </Button>
                </div>
            </div>
            <div>
                {translate("role.journalist.menu.public")}
                <label onClick={()=>this.handlePublicToggle()}>
                    <Icon>{this.state.syncedPublic ? "check" : "close"}</Icon>
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