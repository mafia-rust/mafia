import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER, { replaceMentions } from "../../../index";
import { ContentMenus, ContentTab } from "../GameScreen";
import "./willMenu.css"
import { StateListener } from "../../../game/gameManager.d";


type FieldType = "will" | "notes" | "deathNote";
type Fields = { [key in FieldType]: string };

type WillMenuState = {
    syncedFields : Fields
    localFields: Fields
}

export default class WillMenu extends React.Component<{}, WillMenuState> {
    listener: StateListener
    constructor(props: {}) {
        super(props);

        if(GAME_MANAGER.state.stateType === "game"){
            let gameStateFields = {
                will: GAME_MANAGER.state.will,
                notes: GAME_MANAGER.state.notes,
                deathNote: GAME_MANAGER.state.deathNote,
            };
    
            this.state = {
                syncedFields: gameStateFields,
                localFields: gameStateFields
            };
        }
        
        this.listener = (type) => {
            if (type === "yourWill" || type === "yourNotes" || type === "yourDeathNote") {
                if(GAME_MANAGER.state.stateType === "game")
                    this.setState({
                        syncedFields: {
                            will: GAME_MANAGER.state.will,
                            notes: GAME_MANAGER.state.notes,
                            deathNote: GAME_MANAGER.state.deathNote,
                        },
                        localFields: {
                            will: GAME_MANAGER.state.will,
                            notes: GAME_MANAGER.state.notes,
                            deathNote: GAME_MANAGER.state.deathNote,
                        }
                    });
            }
        };  
    }
    componentDidMount() {
        GAME_MANAGER.addStateListener(this.listener);
    }
    componentWillUnmount() {
        GAME_MANAGER.removeStateListener(this.listener);
    }
    send(type: FieldType) {
        this.save(type);
        if (GAME_MANAGER.state.stateType === "game")
            GAME_MANAGER.sendSendMessagePacket('\n' + replaceMentions(
                this.state.localFields[type],
                GAME_MANAGER.state.players
            ));
    }
    save(type: FieldType) {
        if (type === "will")
            GAME_MANAGER.sendSaveWillPacket(this.state.localFields[type])
        else if (type === "notes")
            GAME_MANAGER.sendSaveNotesPacket(this.state.localFields[type])
        else if (type === "deathNote")
            GAME_MANAGER.sendSaveDeathNotePacket(this.state.localFields[type])
    }
    renderInput(type: FieldType) {
        return (<details open={type === "will"}>
            <summary>{translate("menu.will." + type)}</summary>
            <div>
                <button 
                    className={"material-icons-round " + (this.state.syncedFields[type] !== this.state.localFields[type] ? "highlighted" : "")}
                    onClick={() => this.save(type)}
                    aria-label={translate("menu.will.save")}
                >
                    save
                </button>
                <button 
                    className="material-icons-round"
                    onClick={() => this.send(type)}
                    aria-label={translate("menu.will.post")}
                >
                    send
                </button>
            </div>
            <div>
                <textarea
                    value={this.state.localFields[type]}
                    onChange={(e) => {
                        let fields = {...this.state.localFields};
                        fields[type] = e.target.value;
                        this.setState({ localFields: fields });
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
        <ContentTab close={ContentMenus.WillMenu}>{translate("menu.will.title")}</ContentTab>
        <section>
            {this.renderInput("will")}
            {this.renderInput("notes")}
            {this.renderInput("deathNote")}
        </section>
    </div>);}
}