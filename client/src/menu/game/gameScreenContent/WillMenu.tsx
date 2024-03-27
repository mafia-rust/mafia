import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import { ContentMenu, ContentTab } from "../GameScreen";
import "./willMenu.css"
import { StateListener } from "../../../game/gameManager.d";


type FieldType = "will" | "notes" | "deathNote";

type WillMenuState = {
    will: string,
    notes: string,
    deathNote: string,

    cantPost: boolean,
}

export default class WillMenu extends React.Component<{}, WillMenuState> {
    listener: StateListener
    constructor(props: {}) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game")
            this.state = {
                will: GAME_MANAGER.state.will,
                notes: GAME_MANAGER.state.notes,
                deathNote: GAME_MANAGER.state.deathNote,
                cantPost: GAME_MANAGER.state.sendChatGroups.length === 0,
            };
        
        this.listener = (type) => {
            if(GAME_MANAGER.state.stateType === "game"){
                switch(type){
                    case "yourWill":
                        this.setState({
                            will: GAME_MANAGER.state.will,
                        });
                        break;
                    case "yourNotes":
                        this.setState({
                            notes: GAME_MANAGER.state.notes,
                        });
                        break;
                    case "yourDeathNote":
                        this.setState({
                            deathNote: GAME_MANAGER.state.deathNote,
                        });
                        break;
                    case "yourSendChatGroups":
                        this.setState({
                            cantPost: GAME_MANAGER.state.sendChatGroups.length === 0,
                        });
                        break;
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
    send(type: FieldType){
        this.save(type);
        if (GAME_MANAGER.state.stateType === "game"){
            switch(type){
                case "will":
                    GAME_MANAGER.sendSendMessagePacket('\n' + this.state.will);
                    break;
                case "notes":
                    GAME_MANAGER.sendSendMessagePacket('\n' + this.state.notes);
                    break;
                case "deathNote":
                    GAME_MANAGER.sendSendMessagePacket('\n' + this.state.deathNote);
                    break;
            }   
        }
    }
    save(type: FieldType) {
        switch(type){
            case "will":
                GAME_MANAGER.sendSaveWillPacket(this.state.will);
                break;
            case "notes":
                GAME_MANAGER.sendSaveNotesPacket(this.state.notes);
                break;
            case "deathNote":
                GAME_MANAGER.sendSaveDeathNotePacket(this.state.deathNote);
                break;
        }
    }
    renderInput(type: FieldType) {
        const unsaved = GAME_MANAGER.state.stateType === "game" && (
            (type === "will" && GAME_MANAGER.state.will !== this.state.will) ||
            (type === "notes" && GAME_MANAGER.state.notes !== this.state.notes) ||
            (type === "deathNote" && GAME_MANAGER.state.deathNote !== this.state.deathNote)
        );


        return (<details open={type !== "deathNote"}>
            <summary>
                {translate("menu.will." + type)}
                <div>
                    {unsaved ? "Unsaved" : "Saved"}
                    <button
                        className={"material-icons-round " + (unsaved ? "highlighted" : "")}
                        onClick={() => this.save(type)}
                        aria-label={translate("menu.will.save")}
                    >
                        save
                    </button>
                    <button
                        disabled={this.state.cantPost}
                        className="material-icons-round"
                        onClick={() => this.send(type)}
                        aria-label={translate("menu.will.post")}
                    >
                        send
                    </button>
                </div>
            </summary>
            
            <div>
                <textarea
                    value={(()=>{
                        switch(type){
                            case "will":
                                return this.state.will;
                            case "notes":
                                return this.state.notes;
                            case "deathNote":
                                return this.state.deathNote;
                        }
                    })()}
                    onChange={(e) => {
                        switch(type){
                            case "will":
                                this.setState({
                                    will: e.target.value
                                });
                                break;
                            case "notes":
                                this.setState({
                                    notes: e.target.value
                                });
                                break;
                            case "deathNote":
                                this.setState({
                                    deathNote: e.target.value
                                });
                                break;
                        }
                    }}
                    onKeyDown={(e) => {
                        if (e.ctrlKey) {
                            if (e.key === 's') {
                                e.preventDefault();
                                this.save(type);
                            } else if (e.key === "Enter") {
                                this.send(type);
                            }
                        }
                    }}>
                </textarea>
            </div>
        </details>)
    }
    render() {return (<div className="will-menu will-menu-colors">
        <ContentTab close={ContentMenu.WillMenu} helpMenu={"standard/alibi"}>{translate("menu.will.title")}</ContentTab>
        <section>
            {this.renderInput("will")}
            {this.renderInput("notes")}
            {this.renderInput("deathNote")}
        </section>
    </div>);}
}