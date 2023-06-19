import React from "react";
import translate from "../../../game/lang";
import GAME_MANAGER from "../../../index";
import GameScreen, { ContentMenus } from "../GameScreen";
import "./willMenu.css"
import { StateListener } from "../../../game/gameManager.d";


type FieldType = "will" | "notes";
type Fields = { [key in FieldType]: string };

interface WillMenuState {
    syncedFields : Fields
    localFields: Fields
}

const saveFieldFunctionMap = {
    will: (field: string) => GAME_MANAGER.sendSaveWillPacket(field),
    notes: (field: string) => GAME_MANAGER.sendSaveNotesPacket(field)
}

export default class WillMenu extends React.Component<{}, WillMenuState> {
    listener: StateListener
    constructor(props: {}) {
        super(props);

        let gameStateFields = {
            will: GAME_MANAGER.gameState.will,
            notes: GAME_MANAGER.gameState.notes
        };

        this.state = {
            syncedFields: gameStateFields,
            localFields: gameStateFields
        };
        this.listener = (type) => {
            if (type === "yourWill" || type === "yourNotes") {
                this.setState({
                    syncedFields: {
                        will: GAME_MANAGER.gameState.will,
                        notes: GAME_MANAGER.gameState.notes,
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
    renderInput(type: FieldType) {
        return (<div className="textarea-section">
            {translate("menu.will." + type)}
            <button 
                className={this.state.syncedFields[type] !== this.state.localFields[type] ? "highlighted" : undefined}
                onClick={() => saveFieldFunctionMap[type](this.state.localFields[type])}
            >
                {translate("menu.will.save")}
            </button>
            <button onClick={() => GAME_MANAGER.sendSendMessagePacket('\n' + this.state.syncedFields[type])}>
                {translate("menu.will.post")}
            </button>
            <textarea
                value={this.state.localFields[type]}
                onChange={(e) => {
                    let fields = this.state.localFields;
                    fields[type] = e.target.value;
                    this.setState({ localFields: fields });
                }}
                onKeyDown={(e) => {
                    if (e.ctrlKey && e.key === 's') {
                        // Prevent the Save dialog from opening
                        e.preventDefault();
                        saveFieldFunctionMap[type](this.state.localFields[type]);
                    }
                }}>
            </textarea>
        </div>)
    }
    render() {return (<div className="will-menu">
        <button onClick={()=>{GameScreen.instance.closeMenu(ContentMenus.WillMenu)}}>
            {translate("menu.will.title")}
        </button>
        <section>
            {this.renderInput("will")}
            {this.renderInput("notes")}
        </section>
    </div>);}
}