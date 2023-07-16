import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameScreen, { ContentMenus } from "../GameScreen";
import "./willMenu.css"
import { StateListener } from "../../../game/gameManager.d";
import StyledText from "../../../components/StyledText";


type FieldType = "will" | "notes" | "deathNote";
type Fields = { [key in FieldType]: string };

interface WillMenuState {
    syncedFields : Fields
    localFields: Fields
}

export default class WillMenu extends React.Component<{}, WillMenuState> {
    listener: StateListener
    constructor(props: {}) {
        super(props);

        let gameStateFields = {
            will: GAME_MANAGER.gameState.will,
            notes: GAME_MANAGER.gameState.notes,
            deathNote: GAME_MANAGER.gameState.deathNote,
        };

        this.state = {
            syncedFields: gameStateFields,
            localFields: gameStateFields
        };
        this.listener = (type) => {
            if (type === "yourWill" || type === "yourNotes" || type === "yourDeathNote") {
                this.setState({
                    syncedFields: {
                        will: GAME_MANAGER.gameState.will,
                        notes: GAME_MANAGER.gameState.notes,
                        deathNote: GAME_MANAGER.gameState.deathNote,
                    }
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
    send(type: FieldType) {
        this.save(type);
        GAME_MANAGER.sendSendMessagePacket('\n' + this.state.localFields[type])
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
            <button 
                className={this.state.syncedFields[type] !== this.state.localFields[type] ? "highlighted" : undefined}
                onClick={() => this.save(type)}
            >
                {translate("menu.will.save")}
            </button>
            <button onClick={() => this.send(type)}>
                {translate("menu.will.post")}
            </button>
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
                            // Prevent the Save dialog from opening
                            e.preventDefault();
                            this.save(type);
                        } else if (e.key === "Enter") {
                            this.send(type);
                        }
                    }
                }}>
            </textarea>
        </details>)
    }
    render() {return (<div className="will-menu">
        <div>
            <div>
                <StyledText>
                    {translate("menu.will.title")}
                </StyledText>
            </div>

            <button onClick={()=>{
                GameScreen.instance.closeMenu(ContentMenus.WillMenu)
            }}>✕</button>
        </div>

        <section>
            {this.renderInput("will")}
            {this.renderInput("notes")}
            {this.renderInput("deathNote")}
        </section>
    </div>);}
}