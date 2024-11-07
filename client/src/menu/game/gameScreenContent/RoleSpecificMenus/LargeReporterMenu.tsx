import React from "react"
import GAME_MANAGER from "../../../.."
import translate from "../../../../game/lang"
import "./largeReporterMenu.css"
import Icon from "../../../../components/Icon"
import { TextDropdownArea } from "../../../../components/TextAreaDropdown"

type LargeReporterMenuProps = {
}
type LargeReporterMenuState = {
    syncedPublic: boolean,
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

    render(){
        return <div className="large-reporter-menu">
            <div>
                {translate("role.reporter.menu.public")}
                <label onClick={()=>this.handlePublicToggle()}>
                    <Icon>{this.state.syncedPublic ? "check" : "close"}</Icon>
                </label>
            </div>
            <TextDropdownArea
                open={true}
                titleString={translate("role.reporter.menu.report")}
                savedText={this.state.syncedReport}
                onSave={(text)=>{
                    GAME_MANAGER.sendSetReporterReport(
                        text,
                    );
                }}
                cantPost={false}
            />
        </div>
    }
}