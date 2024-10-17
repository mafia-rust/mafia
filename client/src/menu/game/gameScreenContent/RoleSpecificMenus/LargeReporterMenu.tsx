import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import "./largeReporterMenu.css"
import { Button } from "../../../../components/Button"
import Icon from "../../../../components/Icon"

type LargeReporterMenuProps = {
}
type LargeReporterMenuState = {
    syncedPublic: boolean,
    localReport: string,
    syncedReport: string,
}
export default class LargeReporterMenu extends React.Component<LargeReporterMenuProps, LargeReporterMenuState> {
    listener: () => void;
    constructor(props: LargeReporterMenuState) {
        super(props);

        if(
            GAME_MANAGER.state.stateType === "game" && 
            GAME_MANAGER.state.clientState.type === "player" &&
            GAME_MANAGER.state.clientState.roleState?.type === "reporter"
        )
            this.state = {
                syncedPublic: GAME_MANAGER.state.clientState.roleState?.public,
                localReport: GAME_MANAGER.state.clientState.roleState?.report,
                syncedReport: GAME_MANAGER.state.clientState.roleState?.report,
            };
        this.listener = ()=>{
            if(
                GAME_MANAGER.state.stateType === "game" &&
                GAME_MANAGER.state.clientState.type === "player" &&
                GAME_MANAGER.state.clientState.roleState?.type === "reporter"
            ){
                this.setState({
                    syncedReport: GAME_MANAGER.state.clientState.roleState.report,
                    syncedPublic: GAME_MANAGER.state.clientState.roleState.public,
                })
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
        GAME_MANAGER.sendSetReporterReportPublic(
            !this.state.syncedPublic
        );
    }
    handleSave(){
        GAME_MANAGER.sendSetReporterReport(
            this.state.localReport,
        );
    }
    handleSend(){
        GAME_MANAGER.sendSendMessagePacket('\n' + this.state.syncedReport);
    }

    render(){
        return <div className="large-reporter-menu">
            <div>
                {translate("role.reporter.menu.report")}
                <div>
                    <Button
                        highlighted={this.state.syncedReport !== this.state.localReport}
                        onClick={() => {
                            this.handleSave();
                            return true;
                        }}
                        pressedChildren={() => <Icon>done</Icon>}
                    >
                        <Icon>save</Icon>
                    </Button>
                    <Button
                        onClick={() => {
                            this.handleSend();
                            return true;
                        }}
                        pressedChildren={() => <Icon>done</Icon>}
                    >
                        <Icon>send</Icon>
                    </Button>
                </div>
            </div>
            <div>
                {translate("role.reporter.menu.public")}
                <label onClick={()=>this.handlePublicToggle()}>
                    <Icon>{this.state.syncedPublic ? "check" : "close"}</Icon>
                </label>
            </div>
            <textarea
                value={this.state.localReport}
                onChange={(e) => {
                    this.setState({ localReport: e.target.value });
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